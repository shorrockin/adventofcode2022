use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

// Represents a directory on the file system, used to access the various
// listings and ancestors from the file system. Can be cd'd into.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Directory {
    name: String,
    path: String,
}

impl Directory {
    fn new(name: &str, parent: Option<Directory>) -> Directory {
        Directory {
            name: name.to_string(),
            path: match parent {
                Some(dir) => dir.full_path(),
                None => "".to_string(),
            },
        }
    }

    fn full_path(&self) -> String {
        self.path.clone() + &self.name
    }
}

// Represents the contents of a directory. Separate from Directory to retain
// easy memory access of a directory via cloning and whatnot.
struct DirectoryListing {
    directories: HashSet<Directory>,
    files: HashSet<File>,
}

impl DirectoryListing {
    fn new() -> DirectoryListing {
        DirectoryListing {
            directories: HashSet::new(),
            files: HashSet::new(),
        }
    }

    fn mkdir(&mut self, dir: Directory) {
        self.directories.insert(dir);
    }

    fn touch(&mut self, file: File) {
        self.files.insert(file);
    }
}

// A file exists in a directory, it has a size. The size of a directory is equal
// to the sum of all the files and the directories contained within.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> File {
        File { name, size }
    }
}

// The file system as a whole. Retains a memory of ownership as well as the
// index of where things are stored on disk.
struct FileSystem {
    root: Directory,
    ancestors: HashMap<Directory, Directory>,
    listings: HashMap<Directory, DirectoryListing>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            root: Directory::new("/", None),
            listings: HashMap::new(),
            ancestors: HashMap::new(),
        };
        fs.listings.insert(fs.root.clone(), DirectoryListing::new());
        fs
    }

    fn mkdir(&mut self, parent: Directory, name: &str) -> Directory {
        let dir = Directory::new(name, Some(parent.clone()));
        let listings = self.listings.get_mut(&parent).unwrap();
        listings.mkdir(dir.clone());

        if !self.listings.contains_key(&dir) {
            self.ancestors.insert(dir.clone(), parent);
            self.listings.insert(dir.clone(), DirectoryListing::new());
        }
        dir
    }

    fn touch(&mut self, dir: Directory, size: u32, name: &str) {
        let file = File::new(name.to_string(), size);
        let listings = self.listings.get_mut(&dir).unwrap();
        listings.touch(file);
    }

    fn dir_size(&self, dir: &Directory) -> u32 {
        let listings = self.listings.get(dir).unwrap();
        let file_sizes: u32 = listings.files.iter().map(|file| file.size).sum();

        let dir_sizes: u32 = listings
            .directories
            .iter()
            .map(|subdir| self.dir_size(subdir))
            .sum();

        file_sizes + dir_sizes
    }

    fn parent(&self, dir: &Directory) -> Directory {
        assert_ne!(self.root, *dir);
        let parent = self.ancestors.get(dir).unwrap();
        parent.clone()
    }
}

// The shell represents our interaction state with the file system. Contains the
// fire system as well as what directory we're currently in.
struct Shell {
    file_system: FileSystem,
    current: Directory,
}

impl Shell {
    fn new(file_system: FileSystem) -> Shell {
        let current = file_system.root.clone();
        Shell {
            file_system,
            current,
        }
    }

    fn cd(&mut self, path: &str) {
        match path {
            "/" => self.current = self.file_system.root.clone(),
            ".." => self.current = self.file_system.parent(&self.current),
            _ => self.current = self.file_system.mkdir(self.current.clone(), path),
        };
    }

    fn touch(&mut self, size: u32, name: &str) {
        self.file_system.touch(self.current.clone(), size, name);
    }

    fn dir_sizes(&self) -> Vec<(Directory, u32)> {
        self.file_system
            .listings
            .iter()
            .map(|(key, _)| (key.clone(), self.file_system.dir_size(key)))
            .sorted_by(|left, right| Ord::cmp(&right.1, &left.1))
            .collect()
    }
}

fn run_commands(input: &str) -> Shell {
    let mut shell = Shell::new(FileSystem::new());

    input
        .lines()
        .map(|line| line.split(' ').collect())
        .for_each(|parts: Vec<&str>| match parts[0] {
            "$" => {
                if parts[1].eq("cd") {
                    shell.cd(parts[2])
                }
            }
            "dir" => (),
            _ => shell.touch(parts[0].parse().unwrap(), parts[1]),
        });
    shell
}

pub fn part_one(input: &str) -> u32 {
    run_commands(input)
        .dir_sizes()
        .iter()
        .filter(|(_, size)| size <= &100_000)
        .map(|(_, size)| size)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let shell = run_commands(input);
    let dir_sizes = shell.dir_sizes();
    let total_disk_size = 70_000_000;
    let desired_unused = 30_000_000;
    let current_unused = total_disk_size - shell.dir_sizes()[0].1;
    let deletion_size = desired_unused - current_unused;

    let to_delete: Vec<u32> = dir_sizes
        .iter()
        .filter(|(_, size)| size >= &deletion_size)
        .map(|(_, size)| *size)
        .collect();

    *to_delete.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs;

    static EXAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"};

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(95437, part_one(EXAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1182909, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(24933642, part_two(EXAMPLE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2832508, part_two(&read_input_file()));
    }
}
