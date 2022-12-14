use std::{
    collections::HashMap,
    sync::{
        Arc,
        RwLock
    }
};

pub type DirectoryMap = Arc<RwLock<HashMap<String, Directory>>>;

#[derive(Debug)]
struct Directory {
    pub path: String,
    pub children: Vec<String>,
    pub files: HashMap<String, u64>,
    pub size: Option<u64>
}

pub fn read_directory(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);
    let dir_map_lock: DirectoryMap = Arc::new(RwLock::new(HashMap::new()));

    let root = Directory {
        path: "".to_string(),
        children: Vec::new(),
        files: HashMap::new(),
        size: None
    };

    let mut dir_map = dir_map_lock.write().unwrap();
    dir_map.insert("/".to_string(), root);
    drop(dir_map);

    let mut i = 0;
    let mut curr_dir = "/".to_string();
    while i < lines.len() {
        let cmd = lines[i].as_str();
        match &cmd[0..4] {
            "$ cd" => {
                let tokens: Vec<&str> = cmd.split_whitespace().collect();
                if tokens[2] == "/" {
                    curr_dir = "/".to_string()
                } else if tokens[2] == ".." {
                    curr_dir = parent(&curr_dir).to_string()
                } else {
                    curr_dir = format!("{}/{}", curr_dir, tokens[2]);
                }
                i += 1;
            }
            "$ ls" => {
                let mut dir_map = dir_map_lock.write().unwrap();
                let mut dir = dir_map.get_mut(&curr_dir).unwrap();
                let mut children = Vec::new();

                i += 1;
                while i < lines.len() && !lines[i].starts_with("$") {
                    let tokens: Vec<&str> = lines[i].split_whitespace().collect();
                    if tokens[0] == "dir" {
                        let path = format!("{}/{}", curr_dir, tokens[1]);
                        dir.children.push(path.clone());

                        let child = Directory {
                            path,
                            children: Vec::new(),
                            files: HashMap::new(),
                            size: None
                        };
                        children.push(child);
                    } else {
                        let size: u64 = tokens[0].parse().unwrap();
                        dir.files.insert(tokens[1].to_string(), size);
                    }
                    i += 1;
                }
                for child in children {
                    dir_map.insert(child.path.clone(), child);
                }
                drop(dir_map);
            }
            _ => {}
        }
    }

    let (_, result) = small_sizes(dir_map_lock.clone(), "/");
    println!("result: {}", result);

    let dir_map = dir_map_lock.read().unwrap();
    let mut to_delete = "/";
    let mut perfect_size = dir_map.get("/").unwrap().size.unwrap();
    let threshold = 30000000 - (70000000 - perfect_size);
    for (_, dir) in dir_map.iter() {
        let size = dir.size.unwrap();
        if size < perfect_size && size >= threshold {
            to_delete = dir.path.as_str();
            perfect_size = size;
        }
        println!("dir: {:?}", dir);
    }
    println!("to delete: {}, size: {}", to_delete, perfect_size);
}

fn small_sizes(dir_map_lock: DirectoryMap, curr_dir: &str) -> (u64, u64) {
    let dir_map = dir_map_lock.read().unwrap();
    let dir = dir_map.get(curr_dir).unwrap();
    let children = dir.children.clone();
    let files = dir.files.clone();
    drop(dir_map);

    let mut dir_size = 0;
    for (_, size)  in files {
        dir_size += size;
    }
    let mut result = 0;
    for child in children {
        let (size, res) = small_sizes(dir_map_lock.clone(), &child);
        dir_size += size;
        result += res;
    }
    if dir_size < 100000 {
        result += dir_size;
    }

    let mut dir_map = dir_map_lock.write().unwrap();
    let mut dir = dir_map.get_mut(curr_dir).unwrap();
    dir.size = Some(dir_size);
    drop(dir_map);

    (dir_size, result)
}

fn parent(path: &str) -> &str {
    let tokens: Vec<&str> = path.split("/").collect();
    let length = tokens.len();
    if length <= 1 {
        "/"
    } else {
        let subtract = path.len() - (tokens[length - 1].len() + 1);
        &path[0..subtract]
    }
}