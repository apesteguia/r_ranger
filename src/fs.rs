use std::path::{Path, PathBuf};
use std::{env, error, fs, io};

#[derive(Debug, Clone)]
pub struct Dir {
    pub files: Vec<FileInfo>,
    pub last_dir_path: PathBuf,
    pub current_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileInfo {
    pub is_dir: bool,
    pub name: String,
    pub size: u64,
}

impl Dir {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Dir {
            files: return_dir_vec(&env::current_dir()?).expect("Error creating new Dir"),
            last_dir_path: PathBuf::new(),
            current_path: env::current_dir()?,
        })
    }

    pub fn from(path: &Path) -> Result<Self, io::Error> {
        Ok(Dir {
            files: return_dir_vec(path).expect("Error creating new Dir"),
            last_dir_path: PathBuf::new(),
            current_path: env::current_dir()?,
        })
    }

    pub fn get_dir(&mut self, path: &Path) {
        match fs::read_dir(path) {
            Ok(dirs) => {
                let files_info: Result<Vec<_>, _> = dirs
                    .filter_map(|f| f.ok())
                    .map(|f| {
                        let is_dir = f.file_type()?.is_dir();
                        let size = if is_dir { 0 } else { f.metadata()?.len() };

                        FileInfo::new(is_dir, f.file_name().to_str().unwrap().to_string(), size)
                    })
                    .collect();

                match files_info {
                    Ok(files) => {
                        self.files = files.into_iter().collect();
                        self.last_dir_path = path.to_path_buf();
                    }
                    Err(e) => eprintln!("Error processing files: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading dir: {}", e),
        }
    }

    pub fn order_alphabetically(&mut self) {
        self.files
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    pub fn go_back(&mut self) {
        self.get_dir(&self.last_dir_path.clone());
    }
}

impl FileInfo {
    fn new(is_dir: bool, name: String, size: u64) -> Result<Self, io::Error> {
        Ok(FileInfo { is_dir, name, size })
    }
}

pub fn return_dir_vec(path: &Path) -> Result<Vec<FileInfo>, Box<dyn error::Error>> {
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
                Ok(files) => Ok(files),
                Err(e) => {
                    eprintln!("Error processing files: {}", e);
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(Box::new(e))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Dir::new().unwrap();
        assert_eq!(
            a.current_path,
            Path::new("/home/mikel/Escritorio/r_ranger/")
        )
    }
}
