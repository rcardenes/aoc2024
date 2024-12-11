use std::io::{stdin, BufRead, BufReader, Read};
use std::fmt::Display;

enum Fragmentation {
    Allow,
    DontAllow,
}

#[derive(Clone, Debug)]
struct File {
    id: usize, blocks: usize
}

impl File {
    fn split(&mut self, blocks: usize) -> File {
        self.blocks -= blocks;

        File {
            id: self.id,
            blocks,
        }
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

    fn split_last(&mut self, blocks: usize) -> File {
        self.freespace += blocks;
        self.content.last_mut().unwrap().split(blocks)
    }

    fn append(&mut self, file: File) {
        assert!(self.freespace >= file.blocks);
        self.freespace -= file.blocks;
        self.content.push(file);
    }

    fn pop(&mut self) -> Option<File> {
        self.content.pop().and_then(|f| { self.freespace += f.blocks; Some(f) })
    }

    fn will_fit(&self, blocks: usize) -> bool {
        self.freespace >= blocks
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

        write!(f, "{files}{empty}|")
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

    fn compact(&self, frag: Fragmentation) -> FileSystem {
        let mut structure = self.structure.clone();

        // By definition the first empty spot will be at the second span
        let mut freespace_pointer = 1usize;
        let mut data_pointer = self.data_pointer;
        let mut last_data_block = None;

        while freespace_pointer < data_pointer {
            let (before_last_span, from_last_span) = structure.split_at_mut(data_pointer);
            let data_span = &mut from_last_span[0];
            let blocks_to_move = data_span.last().unwrap().blocks;

            match frag {
                Fragmentation::Allow => {
                    let free_span = &mut before_last_span[freespace_pointer];

                    if !free_span.will_fit(blocks_to_move) {
                        free_span.append(data_span.split_last(free_span.freespace));
                    } else {
                        free_span.append(data_span.pop().unwrap());
                    }
                }
                Fragmentation::DontAllow => {
                    for free_span in &mut before_last_span[freespace_pointer..] {
                        if free_span.will_fit(blocks_to_move) {
                            free_span.append(data_span.pop().unwrap());
                            break;
                        } else if last_data_block.is_none() {
                            last_data_block = Some(data_pointer);
                        }
                    }
                    data_pointer -= 1;
                }
            }

            while freespace_pointer < data_pointer && structure[freespace_pointer].is_full() {
                freespace_pointer += 1;
            }

            while freespace_pointer < data_pointer && structure[data_pointer].is_empty() {
                data_pointer -=1;
            }
        }

        FileSystem {
            structure,
            data_pointer: last_data_block.unwrap_or(data_pointer),
        }
    }

    fn checksum(&self) -> usize {
        let mut curr_block = 0usize;
        let mut ret = 0usize;

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
    let compact_fs = filesys.compact(Fragmentation::Allow);
    println!("Checksum for compacted:  {}", compact_fs.checksum());
    let compact_fs = filesys.compact(Fragmentation::DontAllow);
    println!("Checksum for defragment: {}", compact_fs.checksum());
}
