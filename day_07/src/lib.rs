use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct File {
    name: String,
    size: u32
}

pub struct FileSystem {
    root_dir: Directory,
    current_path: Vec<String>
}


impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root_dir: Directory::new("/".to_string()),
            current_path: Vec::new()
        }
    }

    fn current_dir(&mut self) -> &mut Directory {
        let mut current_dir = &mut self.root_dir;
        for item in &self.current_path {
            current_dir = current_dir.get_dir_mut(item).unwrap();
        }
        current_dir
    }

    fn cd(&mut self, dir: String) {
        if dir == "/" {
            self.cd_root()
        } else if dir == "." {
            // do nothing - stay in the current dir
        } else if dir == ".." {
            self.cd_parent()
        } else {
            self.cd_name_relative(dir)
        }
    }

    fn cd_root(&mut self) {
        self.current_path = Vec::new();
    }

    fn cd_parent(&mut self) {
        self.current_path.pop();
    }

    fn cd_name_relative(&mut self, dir: String) {
        if self.current_dir().directories.contains_key(&dir) {
            self.current_path.push(dir);
        }
    }

    fn create_dir_if_not_exists(&mut self, dir_name: &str) {
        let current_dir = self.current_dir();
        if !current_dir.files.contains_key(dir_name) &&
            !current_dir.directories.contains_key(dir_name) {
            current_dir.insert_dir(dir_name);
        }
    }

    fn create_file_if_not_exists(&mut self, file_size: u32, file_name: &str) {
        let current_dir = self.current_dir();
        if !current_dir.files.contains_key(file_name) &&
            !current_dir.directories.contains_key(file_name) {
            current_dir.insert_file(file_size, file_name);
        }
    }

    pub fn get_size(&self) -> u32 {
        self.root_dir.get_size()
    }

    pub fn get_small_directory_size(&self) -> u32 {
        self.root_dir.get_small_directory_size()
    }

    pub fn get_all_directory_sizes(&self) -> Vec<u32> {
        self.root_dir.get_all_directory_sizes()
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path: {:?}\nDirectory: {}\n", self.current_path, self.root_dir)
    }
}

struct Directory {
    name: String,
    directories: HashMap<String, Directory>,
    files: HashMap<String, File>
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            directories: HashMap::new(),
            files: HashMap::new()
        }
    }

    fn get_dir_mut(&mut self, dir: &String) -> Option<&mut Directory> {
        self.directories.get_mut(dir)
    }

    fn insert_dir(&mut self, dir_name: &str) {
        self.directories.insert(dir_name.to_string(), Directory{
            name: dir_name.to_string(),
            directories: HashMap::new(),
            files: HashMap::new()
        });
    }

    fn insert_file(&mut self, file_size: u32, file_name: &str) {
        self.files.insert(file_name.to_string(), File {
            name: file_name.to_string(),
            size: file_size
        });
    }

    fn get_size(&self) -> u32 {
        let mut sum = 0;
        for file in self.files.values() {
            sum += file.size;
        }
        for directory in self.directories.values() {
            sum += directory.get_size();
        }
        sum
    }

    fn get_small_directory_size(&self) -> u32 {
        let mut sum = 0;
        let size = self.get_size();
        if size <= 100000 {
            sum += size;
        }
        for directory in self.directories.values() {
            sum += directory.get_small_directory_size();
        }
        sum
    }

    fn get_all_directory_sizes(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();
        vec.push(self.get_size());
        for directory in self.directories.values() {
            vec.append(&mut directory.get_all_directory_sizes());
        }
        vec
    }
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory Name: {}\nFiles: {:?}\nDirectories: {:?}", self.name, self.files.keys(), self.directories.keys())
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<FileSystem, &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut file_system = FileSystem::new();
        let mut create_mode = false;
        for line in lines {
            if let Ok(l) = line{
                let v = l.split(" ").collect::<Vec<_>>();
                if v[0] == "$" {
                    // command mode (ls, cd)
                    create_mode = false;
                    if v[1] == "cd" {
                        file_system.cd(v[2].to_string());
                    } else if v[1] == "ls" {
                        create_mode = true;
                    }
                } else if create_mode {
                    // output mode for creating directories
                    if v[0] == "dir" {
                        file_system.create_dir_if_not_exists(v[1]);
                    } else {
                        file_system.create_file_if_not_exists(v[0].parse().unwrap(), v[1]);
                    }
                }
            }
        }
        file_system.cd_root();
        Ok(file_system)
    } else {
        Err("no valid file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::read_in_file_system;

    #[test]
    fn test_size() {
        let mut filesystem = read_in_file_system("input_test").unwrap();
        println!("File System: {}\n", filesystem);
        filesystem.cd("a".to_string());
        println!("{}\n", filesystem);
        filesystem.cd("e".to_string());
        println!("{}\n", filesystem);
        assert_eq!(filesystem.current_dir().get_size(), 584);
        filesystem.cd("/".to_string());
        filesystem.cd("a".to_string());
        assert_eq!(filesystem.current_dir().get_size(), 94853);
        filesystem.cd("/".to_string());
        filesystem.cd("d".to_string());
        assert_eq!(filesystem.current_dir().get_size(), 24933642);
        assert_eq!(filesystem.root_dir.get_size(), 48381165);
        assert_eq!(filesystem.root_dir.get_small_directory_size(), 95437)
    }
}
