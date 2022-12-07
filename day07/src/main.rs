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
    value: IntNode,
}

struct Dir {
    entries: Vec<usize>,
    parent: usize,
}

enum IntNode {
    File,
    Dir(Dir),
}

impl IntNode {
    fn as_mut_dir(&mut self) -> &mut Dir {
        match self {
            IntNode::Dir(ref mut dir) => dir,
            IntNode::File => unreachable!(),
        }
    }
    fn as_dir(&self) -> &Dir {
        match self {
            IntNode::Dir(dir) => dir,
            IntNode::File => unreachable!(),
        }
    }
}

fn update_dir_sizes(nodes: &Vec<Node>, curr_dir: usize) {
    match &nodes[curr_dir].value {
        IntNode::Dir(dir) => {
            let mut size = 0;
            for entry in &dir.entries {
                update_dir_sizes(nodes, *entry);
                size += nodes[*entry].size.get();
            }
            nodes[curr_dir].size.set(size);
        }
        _ => (),
    }
}

fn main() {
    mos_alloc::set_limit(20000);
    let mut nodes: Vec<Node> = Vec::with_capacity(512);
    nodes.push(Node {
        name: b"/".to_vec(),
        size: Default::default(),
        value: IntNode::Dir(Dir {
            entries: Vec::new(),
            parent: 0,
        }),
    });

    let mut curr_dir = 0;

    for line in utils::iter_lines!("input.txt") {
        if line.starts_with(b"$ cd ") {
            let name = &line[5..];
            if name == b"/" {
                curr_dir = 0;
            } else if name == b".." {
                curr_dir = nodes[curr_dir].value.as_dir().parent;
            } else {
                for entry in &nodes[curr_dir].value.as_dir().entries {
                    if nodes[*entry].name == name {
                        curr_dir = *entry;
                    }
                }
            }
        } else if line[0] != b'$' {
            if line.starts_with(b"dir ") {
                let name = line[4..].to_vec();
                let index = nodes.len();
                nodes.push(Node {
                    name,
                    size: Default::default(),
                    value: IntNode::Dir(Dir {
                        entries: Vec::new(),
                        parent: curr_dir,
                    }),
                });
                nodes[curr_dir].value.as_mut_dir().entries.push(index);
            } else {
                let mut parts = line.split(|v| *v == b' ');
                let size = to_str(parts.next().unwrap()).parse().unwrap();
                let name = parts.next().unwrap().to_vec();
                let index = nodes.len();
                nodes.push(Node {
                    name,
                    size: Cell::new(size),
                    value: IntNode::File,
                });
                nodes[curr_dir].value.as_mut_dir().entries.push(index);
            }
        }
    }
    update_dir_sizes(&nodes, 0);

    let dirs = nodes.iter().filter(|f| match f.value {
        IntNode::Dir(_) => true,
        _ => false,
    });

    let part1 = dirs
        .clone()
        .filter(|f| match f.value {
            IntNode::Dir(_) => f.size.get() < 100000,
            _ => false,
        })
        .map(|f| f.size.get())
        .sum::<u32>();
    assert!(part1 == 1644735);
    println!("PART1: {}", part1);

    let total_disk_space = 70000000;
    let update_requires = 30000000;
    let unused_space = total_disk_space - nodes[0].size.get();

    let required_space = update_requires - unused_space;

    let part2 = dirs
        .clone()
        .map(|f| f.size.get())
        .filter(|size| *size >= required_space)
        .min()
        .unwrap_or(0);

    println!("PART2: {}", part2);
}
