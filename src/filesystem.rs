use serde::Serialize;
use serde::Deserialize;
use std::fs::{File, DirEntry};
use std::io::{BufReader, BufWriter, Result, Error, ErrorKind};
use std::fs;
use std::path::Path;
use std::collections::{HashSet, HashMap};
use std::borrow::{BorrowMut, Borrow};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct FileData {
    mtime: u64
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Directory {
    files: HashMap<String, FileData>,
    subdirs: HashMap<String, Directory>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Metadata {
    base_path: String,
    directory: Directory,
}

macro_rules! unwrap_result_or {
    ($e: expr, $s: stmt) => {
        match $e {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e);
                $s
            }
        }
    }
}

macro_rules! unwrap_option_or {
    ($e: expr, $f: expr, $s: stmt) => {
        match $e {
            Some(e) => e,
            None => {
                eprintln!("{}", $f);
                $s
            }
        }
    }
}

impl Metadata {
    pub fn deserialize(filename: &str) -> Result<Metadata> {
        let mut file = File::open(&filename)?;
        let mut reader = BufReader::new(file);
        serde_json::from_reader(&mut reader).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    pub fn serialize(&self, filename: &str) -> Result<()> {
        let mut file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    fn update_rec(&self, dir: &Directory, path: &mut Path) -> Result<()> {
        let list = unwrap_result_or![fs::read_dir(path), return];

        for entry_res in list {
            let entry: DirEntry = unwrap_result_or![entry_res, continue];
            let ftype: fs::FileType = unwrap_result_or![entry.file_type(), continue];
            let fname: &str = unwrap_option_or![entry.file_name().as_os_str().to_str(), "could not unwrap filename", continue];

            if ftype.is_file() {
                if let Some(file) = dir.files.borrow().get_mut(fname) {
                    let meta: std::fs::Metadata = unwrap_result_or![entry.metadata(), continue];
                    let mtime: SystemTime  = unwrap_result_or![meta.modified(), continue];
                    let m64: u64 = mtime.duration_since(UNIX_EPOCH).expect("mtime found before UNIX_EPOCH").as_secs();

                    if m64 != file.mtime {
                        println!("{}", fname);
                        file.mtime = m64;
                    }
                }
            }
            else if ftype.is_dir() {

            }

        }

        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        let mut path = Path::new(&self.base_path);
        self.update_rec(&self.directory, &mut path)
    }
}