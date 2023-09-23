use std::str::FromStr;

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry)
}

#[derive(Debug, Clone, Copy)]
struct GenericParseErr;

impl FromStr for Line {
    type Err = GenericParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$") {
            Ok(Line::Command(Command::from_str(s).unwrap()))
        }
        else {
            Ok(Line::Entry(Entry::from_str(s).unwrap()))
        }
    }
}

#[derive(Debug)]
enum Command {
    List,
    ChangeDir(String)
}

impl FromStr for Command {
    type Err = GenericParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string_split = s.split(" ");
        let mut split_iter = string_split.into_iter();
        split_iter.next(); //We don't care about this since we've already checked if the $ is present in Line::from_str

        let command = split_iter.next().unwrap();
        match command {
            "ls" => Ok(Command::List),
            "cd" => Ok(Command::ChangeDir(split_iter.next().unwrap().to_string())),
            _ => Err(GenericParseErr)
        }
    }
}

#[derive(Debug, Clone)]
enum Entry {
    Directory(String),
    File(String, usize)
}

impl FromStr for Entry {
    type Err = GenericParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string_split = s.split(" ");
        let mut string_split_iter = string_split.into_iter();
        let dir_or_file_sizes_str = string_split_iter.next().unwrap();
        let entry_name = string_split_iter.next().unwrap();
        match dir_or_file_sizes_str {
            "dir" => Ok(Entry::Directory(entry_name.to_string())),
            _ => {
                let file_size = usize::from_str(dir_or_file_sizes_str).unwrap();
                Ok(Entry::File(entry_name.to_string(), file_size))
            }
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<FileSystemNode>
}

impl FileSystem {
    fn add(&mut self, parent_index: usize, entry: Entry) {
        let current_node_count = self.nodes.len();
        let child_node = FileSystemNode {
            self_index: current_node_count,
            entry: entry,
            child_node_indices: vec![],
            parent_index: Some(parent_index)
        };

        self.nodes[parent_index].child_node_indices.push(current_node_count);
        self.nodes.push(child_node);
    }

    fn get_node(&self, idx: usize) -> &FileSystemNode {
        return &self.nodes[idx];
    }

    fn find_dir(&self, parent_index: usize, name: &str) -> Option<FileSystemNode> {
        let parent = &self.nodes[parent_index];
        for child_idx in &parent.child_node_indices {
            let child_node = &self.nodes[*child_idx];
            if let Entry::Directory(dir_name) = child_node.entry.clone() {
                if dir_name == name {
                    return Some(child_node.clone());
                }
            }
        }
        None
    }

    fn get_entry_size(&self, at_index: usize) -> usize {
        let node = &self.nodes[at_index];
        let entry = node.entry.clone();
        match entry {
            Entry::File(_, size) => size,
            Entry::Directory(name) => {
                let total = node.child_node_indices.iter().map(|idx| self.get_entry_size(*idx)).sum();
                total
            }
        } 
    }

    fn get_dirs_with_sizes(&self) -> Vec<(String, usize)> {
        self.nodes.iter()
            .filter(|n| {
                match n.entry.clone() {
                    Entry::Directory(_) => true,
                    _ => false
                }
            })
            .map(|n| { 
                let entry = n.entry.clone();
                if let Entry::Directory(dir_name) = entry {
                    return (dir_name, self.get_entry_size(n.self_index))
                }
                ("".into(), 0)
            }).collect()
    } 

    fn get_total_consumed(&self) -> usize {
        self.nodes.iter().filter_map(|n| {
            match n.entry.clone() {
                Entry::File(name, size) => Some(size),
                Entry::Directory(_) => None
            }
        }).sum()
    }
}

#[derive(Debug, Clone)]
struct FileSystemNode {
    self_index: usize,
    entry: Entry,
    child_node_indices: Vec<usize>,
    parent_index: Option<usize>
}

impl Default for FileSystem {
    fn default() -> Self {
        FileSystem {
            nodes: vec![FileSystemNode { self_index: 0, entry: Entry::Directory("/".into()), child_node_indices: vec![], parent_index: None }]
        }
    }
}


fn main() {
    let directories_text = include_str!("input.txt");
    let lines = directories_text.lines().map(|line| Line::from_str(line).unwrap()).collect::<Vec<_>>();

    let mut file_system = FileSystem::default();
    let mut parent_index = 0 as usize; 
    for line in lines {
        match line {
            Line::Command(command) => {
                match command {
                    Command::ChangeDir(dir_name) => {
                        parent_index = match dir_name.as_str() {
                            ".." => file_system.get_node(parent_index).parent_index.unwrap(),
                            otherwise => file_system.find_dir(parent_index, otherwise).unwrap().self_index 
                        }
                    },
                    _ => {} //LS Case is implicitly handled by the case below for entries
                }
            },
            Line::Entry(entry) => {
                file_system.add(parent_index, entry)
            } 
        }
    }

    //Part 1
    let dirs_with_sizes = file_system.get_dirs_with_sizes();
    let small_dirs_sum: usize = dirs_with_sizes.iter().filter(|(_, b)| *b <= 100000).map(|(_, b)| *b).sum();
    println!("Sum of smaller directories is: {}", small_dirs_sum);

    let disk_size: usize = 70000000;
    let space_needed: usize = 30000000;
    let space_consumed: usize = file_system.get_total_consumed();
    let space_remaining = disk_size - space_consumed;

    let file_size_to_delete = space_needed - space_remaining;
    let smallest_delete_size = dirs_with_sizes.iter().filter(|(_, size)| *size >= file_size_to_delete).map(|(_, b)| b).min();

    println!("The smallest file to delete is {}", smallest_delete_size.unwrap());
}
