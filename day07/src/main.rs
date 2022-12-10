#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

utils::entry!(main);
use alloc::vec::Vec;
use core::cell::Cell;
use ufmt_stdio::*;
use utils::to_str;

struct Node {
    name: Vec<u8>,
    size: Cell<u32>,
    value: NodeType,
}

struct Dir {
    entries: Vec<usize>,
    parent: usize,
}

enum NodeType {
    File,
    Dir(Dir),
}

struct FileSystem {
    curr_dir: usize,
    nodes: Vec<Node>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            curr_dir: 0,
            nodes: Vec::with_capacity(500),
        };
        fs.add_node(Node {
            name: b"/".to_vec(),
            size: Default::default(),
            value: NodeType::Dir(Dir {
                entries: Vec::new(),
                parent: 0,
            }),
        });
        fs
    }
    fn update_dir_sizes(&self, curr_dir: usize) {
        match &self.nodes[curr_dir].value {
            NodeType::Dir(dir) => {
                let mut size = 0;
                for entry in &dir.entries {
                    self.update_dir_sizes(*entry);
                    size += self.nodes[*entry].size.get();
                }
                self.nodes[curr_dir].size.set(size);
            }
            _ => (),
        }
    }
    fn cd(&mut self, name: &[u8]) {
        self.curr_dir = if name == b"/" {
            0
        } else {
            if let NodeType::Dir(dir) = &self.nodes[self.curr_dir].value {
                if name == b".." {
                    dir.parent
                } else {
                    *dir.entries
                        .iter()
                        .filter(|&&idx| self.nodes[idx].name == name)
                        .next()
                        .unwrap()
                }
            } else {
                unreachable!()
            }
        }
    }
    fn add_dir(&mut self, name: &[u8]) {
        self.add_node(Node {
            name: name.to_vec(),
            size: Default::default(),
            value: NodeType::Dir(Dir {
                entries: Vec::new(),
                parent: self.curr_dir,
            }),
        });
    }
    fn add_file(&mut self, name: &[u8], size: u32) {
        self.add_node(Node {
            name: name.to_vec(),
            size: Cell::new(size),
            value: NodeType::File,
        });
    }
    fn add_node(&mut self, node: Node) -> usize {
        let is_root = node.name == b"/";
        let index = self.nodes.len();
        self.nodes.push(node);
        if !is_root {
            if let NodeType::Dir(dir) = &mut self.nodes[self.curr_dir].value {
                dir.entries.push(index);
            }
        }
        return index;
    }
    fn iter_dirs(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().filter(|f| match f.value {
            NodeType::Dir(_) => true,
            _ => false,
        })
    }
    fn space_used(&self) -> u32 {
        return self.nodes[0].size.get();
    }
}
fn main() {
    mos_alloc::set_limit(20000);
    let mut fs = FileSystem::new();

    for line in utils::iter_lines!("../../input/day07/input.txt") {
        if line.starts_with(b"$ ") {
            let line = &line[2..];
            if line.starts_with(b"cd ") {
                fs.cd(&line[3..]);
            }
        } else {
            if line.starts_with(b"dir ") {
                fs.add_dir(&line[4..]);
            } else {
                let mut parts = line.split(|v| *v == b' ');
                let size = to_str(parts.next().unwrap()).parse().unwrap();
                let name = parts.next().unwrap();
                fs.add_file(name, size);
            }
        }
    }
    fs.update_dir_sizes(0);
    println!("nodes: {}", fs.nodes.len());
    println!(
        "free heap: {} of {}",
        mos_alloc::bytes_free(),
        mos_alloc::get_limit()
    );
    let part1 = fs
        .iter_dirs()
        .map(|f| f.size.get())
        .filter(|&size| size < 100000)
        .sum::<u32>();
    assert!(part1 == 1644735);
    println!("PART1: {}", part1);

    let total_disk_space = 70000000;
    let update_requires = 30000000;
    let unused_space = total_disk_space - fs.space_used();

    let required_space = update_requires - unused_space;

    let part2 = fs
        .iter_dirs()
        .map(|f| f.size.get())
        .filter(|&size| size >= required_space)
        .min()
        .unwrap();

    println!("PART2: {}", part2);
}
