use std::ffi::NulError;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug)]
pub struct Dir {
    files: Vec<FileInfo>,
    last_dir_path: PathBuf,
    current_path: PathBuf,
}

#[derive(Debug)]
pub struct FileInfo {
    is_dir: bool,
    name: String,
    size: u64,
}

impl Dir {
    pub fn new(&self) -> Result<Self, io::Error> {
        Ok(Dir {
            files: return_dir_vec(),
            last_dir_path: PathBuf::new(),
            current_path: env::current_dir()?,
        })
    }
    pub fn from(&mut self, path: &Path) -> Result<Self, io::Error> {
        if path.exists() && path.is_dir() {}
    }

    fn get_dir(&mut self, path: &Path) {
        match fs::read_dir(path) {
            Ok(dirs) => {
                let files_info: Result<Vec<_>, _> = dirs
                    .filter_map(|f| f.ok())
                    .map(|f| {
                        FileInfo::new(
                            f.file_type()?.is_dir(),
                            f.file_name().to_str().unwrap().to_string(),
                            f.metadata()?.len(),
                        )
                    })
                    .collect();

                match files_info {
                    Ok(files) => {
                        self.files = files.into_iter().filter_map(|x| x.ok()).collect();
                        self.last_dir_path = path.to_path_buf();
                    }
                    Err(e) => eprintln!("Error processing files: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading dir: {}", e),
        }
    }
}

impl FileInfo {
    fn new(is_dir: bool, name: String, size: u64) -> Result<Self, io::Error> {
        Ok(FileInfo { is_dir, name, size })
    }
}

fn return_dir_vec(path: &Path) -> Result<(), Vec<FileInfo>> {
    match fs::read_dir(path) {
        Ok(dirs) => {
            let files_info: Result<Vec<_>, _> = dirs
                .filter_map(|f| f.ok())
                .map(|f| {
                    FileInfo::new(
                        f.file_type()?.is_dir(),
                        f.file_name().to_str().unwrap().to_string(),
                        f.metadata()?.len(),
                    )
                })
                .collect();

            match files_info {
                Ok(files) => Ok(files.into_iter().filter_map(|x| Some(x)).collect()),
                Err(e) => {
                    eprintln!("Error processing files: {}", e);
                    Err(Vec::<FileInfo>::new())
                }
            }
        }
        Err(e) => Err(Vec::<FileInfo>::new()),
    }
}
