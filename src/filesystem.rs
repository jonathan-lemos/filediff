use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::{Read, BufReader, BufWriter};
use std::fs;
use std::path::Path;
use std::collections::{HashSet, HashMap};

#[derive(Serialize, Deserialize)]
struct FileEntry {
    name: String,
    mtime: u64
}

#[derive(Serialize, Deserialize)]
struct Directory {
    files: HashSet<FileEntry>,
    subdirs: HashMap<string, Directory>
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    base_path: string,
    directory: Directory
}


impl Metadata {
    pub fn deserialize(filename: &str) -> Result<Metadata> {
        let mut file = File::open(&filename)?;
        let mut reader = BufReader::new(file);
        serde_json::from_reader(&reader)
    }

    pub fn serialize(&self, filename: &str) -> Result<()> {
        let mut file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&writer, &self)
    }

    pub fn update(&self) -> Result<()> {
        let mut dir_stack = Vec::new();
        let mut cur_path = Path::new(self.base_path);
        dir_stack.push(&self.directory);

        while dir_stack.len() > 0 {
            let (cur, pat) = dir_stack.pop()?;
            let list = fs::read_dir(pat)?;

            for entry in list {
                match entry {
                    Ok(ent) => {
                        let etype = ent.file_type()?;
                        if etype.is_dir() {
                            if let Some(subdir) = cur.subdirs.get(&ent.file_name()) {

                                continue;
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e);
                }
            }
        }

        Ok(())
    }
}