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

    fn inc(&mut self, id: usize) {
        assert!(!self.is_full());

        match self.content.last_mut() {
            Some(last_file) if last_file.id == id => {
                self.freespace -= 1;
                last_file.inc()
            },
            _ => self.append(File { id, blocks: 1 })
        }
    }

    // Returns true if the span becomes empty
    fn dec(&mut self) -> bool {
        assert!(!self.is_empty());

        self.freespace += 1;
        self.last_mut().unwrap().dec();
        if self.last().unwrap().blocks == 0 {
            self.content.pop();
        }

        self.content.is_empty()
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
    freespace_list: Vec<usize>,
    with_file_list: Vec<usize>,
}

impl FileSystem {
    fn new(structure: Vec<Span>) -> Self {
        let (with_file_list, freespace_list): (Vec<(usize, usize)>, Vec<(usize, usize)>) = structure
            .iter()
            .enumerate()
            .map(|(index, s)| (index, s.freespace))
            .partition(|(_, fs)| *fs == 0);
        let with_file_list = with_file_list.into_iter().map(|(i, _)| i).collect();
        let freespace_list = freespace_list.into_iter().map(|(i, _)| i).collect();

        FileSystem {
            structure,
            freespace_list,
            with_file_list,
        }
    }

    fn compact(&self) -> FileSystem {
        let mut freespace_list = self.freespace_list.clone();
        let mut with_file_list = self.with_file_list.clone();
        let mut structure = self.structure.clone();

        loop {
            let last_index= *with_file_list.last().expect("With_file_list is empty!");
            let first_free = *freespace_list.first().expect("Empty free space list!");

            if first_free > last_index {
                break;
            }

            let (before_last_span, from_last_span) = structure.split_at_mut(last_index);
            let last_span = &mut from_last_span[0];
            let free_span = &mut before_last_span[first_free];

            let file = last_span.last().unwrap();
            free_span.inc(file.id);
            last_span.dec();

            if free_span.is_full() {
                freespace_list.remove(0);
            }
            if last_span.is_empty() {
                with_file_list.pop();
            }
        }

        FileSystem {
            structure,
            with_file_list,
            freespace_list,
        }
    }

    fn checksum(&self) -> usize {
        let last_with_content = *self.with_file_list.last().unwrap();
        let mut curr_block = 0usize;
        let mut ret = 0usize;

        for (i, span) in self.structure.iter().enumerate() {
            if i > last_with_content {
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
    // eprintln!("{filesys}");
    let compact_fs = filesys.compact();
    // eprintln!("{compact_fs}");
    println!("Checksum for compacted: {}", compact_fs.checksum());
}
