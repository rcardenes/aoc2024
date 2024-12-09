use std::io::{stdin, BufRead, BufReader, Read};
use std::fmt::Display;

#[derive(Clone, Debug)]
struct File {
    id: usize, blocks: usize
}

impl File {
    fn inc(&mut self) {
        self.blocks += 1
    }

    fn dec(&mut self) {
        self.blocks -= 1
    }

    fn checksum(&self, first_block: usize) -> usize {
        (first_block..(first_block+self.blocks)).map(|index| index*self.id).sum::<usize>()
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = format!("{}", self.id);
        let output = std::iter::repeat(id).take(self.blocks).collect::<String>();
        write!(f, "{output}")
    }
}

#[derive(Clone, Debug)]
struct Span {
    freespace: usize,
    content: Vec<File>,
}

impl Span {
    fn last(&mut self) -> Option<&File> {
        self.content.last()
    }

    fn last_mut(&mut self) -> Option<&mut File> {
        self.content.last_mut()
    }

    fn append(&mut self, file: File) {
        assert!(self.freespace >= file.blocks);
        self.freespace -= file.blocks;
        self.content.push(file);
    }

    // Returns true if the span becomes full
    fn inc(&mut self, id: usize) -> bool {
        assert!(!self.is_full());

        match self.content.last_mut() {
            Some(last_file) if last_file.id == id => {
                self.freespace -= 1;
                last_file.inc()
            },
            _ => self.append(File { id, blocks: 1 })
        }

        self.is_full()
    }

    // Returns true if the span becomes empty
    fn dec(&mut self) -> bool {
        assert!(!self.is_empty());

        self.freespace += 1;
        self.last_mut().unwrap().dec();
        if self.last().unwrap().blocks == 0 {
            self.content.pop();
        }

        self.is_empty()
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn is_full(&self) -> bool {
        self.freespace == 0
    }

    fn size(&self) -> usize {
        self.freespace + self.content.iter().map(|f| f.blocks).sum::<usize>()
    }

    fn checksum(&self, first_block: usize) -> usize {
        let mut curr_block = first_block;
        let mut ret = 0usize;

        for file in self.content.iter() {
            ret += file.checksum(curr_block);
            curr_block += file.blocks
        }

        ret
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let files = self.content
            .iter()
            .map(|f| format!("{f}"))
            .collect::<String>();
        let empty = if self.freespace == 0 { String::new() } else {
            std::iter::repeat('.').take(self.freespace).collect::<String>()
        };

        write!(f, "{files}{empty}")
    }
}

#[derive(Debug)]
struct FileSystem {
    structure: Vec<Span>,
    data_pointer: usize,
}

impl FileSystem {
    fn new(structure: Vec<Span>) -> Self {
        let last_span_is_free = structure.last().is_some_and(|s| s.is_empty());
        // The last span with data will be either the very last of the structure or the previous
        // one, if the last is empty
        let data_pointer = structure.len() - if last_span_is_free { 2 } else { 1 };

        FileSystem {
            structure,
            data_pointer,
        }
    }

    fn compact(&self) -> FileSystem {
        let mut structure = self.structure.clone();

        // By definition the first empty spot will be at the second span
        let mut freespace_pointer = 1usize;
        let mut data_pointer = self.data_pointer;

        while freespace_pointer < data_pointer {
            let (inc_free_span, dec_data_span) = {
                let (before_last_span, from_last_span) = structure.split_at_mut(data_pointer);
                let data_span = &mut from_last_span[0];
                let free_span = &mut before_last_span[freespace_pointer];
                let file = data_span.last_mut().unwrap();
                (free_span.inc(file.id), data_span.dec())
            };

            if inc_free_span {
                // The span just became full
                loop {
                    freespace_pointer += 1;
                    if !structure[freespace_pointer].is_full() {
                        break;
                    }
                }
            }

            if dec_data_span {
                // If the last span with data just became empty
                loop {
                    data_pointer -= 1;
                    if !structure[data_pointer].is_empty() {
                        break;
                    }
                }
            }
        }

        FileSystem {
            structure,
            data_pointer,
        }
    }

    fn checksum(&self) -> usize {
        let mut curr_block = 0usize;
        let mut ret = 0usize;

        // TODO: Rewrite as a fold with take_while
        for (i, span) in self.structure.iter().enumerate() {
            if i > self.data_pointer {
                break;
            }

            ret += span.checksum(curr_block);
            curr_block += span.size();
        }

        ret
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spans = self.structure.iter().map(|s| format!("{s}")).collect::<String>();

        write!(f, "{spans}")
    }
}

struct Reader {
    reading_file: bool,
    next_id: usize,
}

impl Reader {
    fn new() -> Self {
        Reader {
            reading_file: true,
            next_id: 0,
        }
    }

    fn next(&mut self, blocks: usize) -> Option<Span> {
        let is_file = self.reading_file;
        self.reading_file = !is_file;

        if blocks == 0 {
            None
        } else {
            let span = if is_file {
                let id = self.next_id;
                self.next_id += 1;
                let content = vec![File { id, blocks }];

                Span {
                    freespace: 0,
                    content
                }
            } else {
                Span {
                    freespace: blocks,
                    content: vec![]
                }
            };

            Some(span)
        }
    }
}

fn read_input<R>(mut stream: BufReader<R>) -> FileSystem
    where R: Read
{
    let mut buffer = String::new();
    let mut reader = Reader::new();

    stream.read_line(&mut buffer).unwrap();

    let spans = buffer
        .trim_end()
        .chars()
        .filter_map(|c| reader.next(c.to_digit(10).unwrap() as usize))
        .collect::<Vec<_>>();

    FileSystem::new(spans)
}

fn main() {
    let filesys = read_input(BufReader::new(stdin()));
    let compact_fs = filesys.compact();
    // eprintln!("{compact_fs}");
    println!("Checksum for compacted: {}", compact_fs.checksum());
}
