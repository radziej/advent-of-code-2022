use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

enum EdgeDirection {
    // Any,
    Incoming,
    Outgoing,
}

struct DirectedGraph<K: Eq + Hash + Clone, N> {
    nodes: HashMap<K, N>,
    incoming_edges: HashMap<K, HashSet<K>>,
    outgoing_edges: HashMap<K, HashSet<K>>,
}

impl<K: Eq + Hash + Clone, N> DirectedGraph<K, N> {
    fn new() -> DirectedGraph<K, N> {
        DirectedGraph {
            nodes: Default::default(),
            incoming_edges: Default::default(),
            outgoing_edges: Default::default(),
        }
    }

    pub fn insert_node(&mut self, key: K, node: N) -> Option<N> {
        self.nodes.insert(key, node)
    }

    pub fn node(&self, key: &K) -> Option<&N> {
        self.nodes.get(key)
    }

    pub fn nodes(&self) -> Iter<'_, K, N> {
        self.nodes.iter()
    }

    pub fn insert_edge(&mut self, from: &K, to: &K) {
        self.outgoing_edges
            .entry(from.clone())
            .and_modify(|hs| {
                hs.insert(to.clone());
            })
            .or_insert(HashSet::from([to.clone()]));
        self.incoming_edges
            .entry(to.clone())
            .and_modify(|hs| {
                hs.insert(from.clone());
            })
            .or_insert(HashSet::from([from.clone()]));
    }

    // pub fn edge(&self, key: &u64) -> Option<&N> {
    //     self.nodes.get(key)
    // }

    pub fn edges(&self, direction: EdgeDirection) -> Iter<'_, K, HashSet<K>> {
        match direction {
            EdgeDirection::Incoming => self.incoming_edges.iter(),
            EdgeDirection::Outgoing => self.outgoing_edges.iter(),
            // TraversalDirection::Any => self.incoming_edges.iter().chain(self.outgoing_edges.iter()),
        }
    }

    pub fn neighbors(
        &self,
        node: &K,
        direction: EdgeDirection,
    ) -> Option<std::collections::hash_set::Iter<'_, K>> {
        match direction {
            EdgeDirection::Incoming => self.incoming_edges.get(&node).and_then(|k| Some(k.iter())),
            EdgeDirection::Outgoing => self.outgoing_edges.get(&node).and_then(|k| Some(k.iter())),
            // TraversalDirection::Any => self.incoming_edges.get.iter().chain(self.outgoing_edges.iter()),
        }
    }
}

struct FsDirectory {
    name: String,
}

struct FsFile {
    name: String,
    size: u64,
}

enum FsElement {
    Directory(FsDirectory),
    File(FsFile),
}

fn main() {
    let file_path = "./input.txt";
    let mut lines = read_lines(file_path)
        .expect("Should be able to read input file")
        .peekable();

    let mut current_directory: String = "".to_string();
    let mut file_system: DirectedGraph<String, FsElement> = DirectedGraph::new();
    file_system.insert_node(
        current_directory.clone(),
        FsElement::Directory(FsDirectory {
            name: "".to_string(),
        }),
    );

    while let Some(line) = lines.next() {
        let line = line.expect("Should be able to read line from input file");
        if line == "$ ls" {
            // Parse directory content
            while let Some(content) = lines.next() {
                let content = content.expect("Should be able to read content line");
                let (var, name) = content.split_once(" ").expect(
                    "Directory content should be described by two items separated by a whitespace",
                );
                let path = format!("{}/{}", current_directory, name);
                file_system.insert_edge(&current_directory, &path);
                if var == "dir" {
                    file_system.insert_node(
                        path,
                        FsElement::Directory(FsDirectory {
                            name: name.to_string(),
                        }),
                    );
                } else {
                    let size: u64 = var
                        .parse()
                        .expect("File size should be medium sized integer");
                    file_system.insert_node(
                        path,
                        FsElement::File(FsFile {
                            name: name.to_string(),
                            size,
                        }),
                    );
                }

                let is_command = lines
                    .peek()
                    .and_then(|l| {
                        if l.is_ok() {
                            Some(l.as_ref().unwrap().starts_with("$"))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| false);
                if is_command {
                    break;
                }
            }
        } else if line == "$ cd /" {
            // Change current directory to root directory
            current_directory = "".to_string();
        } else if line == "$ cd .." {
            // Change current directory to parent directory
            current_directory = file_system
                .neighbors(&current_directory, EdgeDirection::Incoming)
                .unwrap()
                .next()
                .unwrap()
                .to_owned();
        } else if line.starts_with("$ cd") {
            current_directory = format!("{}/{}", current_directory, &line[5..]);
            assert!(file_system.node(&current_directory).is_some());
        } else {
            panic!(
                "Should only encounter cd or ls commands, but encountered unhandled line: {}",
                line
            );
        }
    }

    let sizes = directory_sizes(&file_system);
    let required_space = 30000000 - (70000000 - sizes.get("").unwrap());
    let smallest_deletion = sizes
        .values()
        .filter(|v| **v >= required_space)
        .min()
        .unwrap()
        .clone();
    // for size in sizes.values() {
    //     if *size >= required_space && *size < smallest_deletion {
    //         smallest_deletion = *size;
    //     }
    // }

    println!(
        "Smallest directory large enough to free the necessary disk space has size of: {}",
        smallest_deletion
    )
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn directory_sizes(file_system: &DirectedGraph<String, FsElement>) -> HashMap<String, u64> {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    traverse(file_system, "".to_string(), &mut sizes);
    sizes
}

fn traverse(
    file_system: &DirectedGraph<String, FsElement>,
    node: String,
    sizes: &mut HashMap<String, u64>,
) {
    sizes.insert(node.clone(), 0);
    for neighbor in file_system
        .neighbors(&node, EdgeDirection::Outgoing)
        .unwrap()
    {
        match file_system.node(neighbor).unwrap() {
            FsElement::File(f) => *sizes.get_mut(&node).unwrap() += f.size,
            FsElement::Directory(_) => {
                traverse(file_system, neighbor.to_owned(), sizes);
                *sizes.get_mut(&node).unwrap() += *sizes.get_mut(neighbor).unwrap();
            }
        }
    }
}
