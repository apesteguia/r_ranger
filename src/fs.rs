use chrono::{DateTime, Local};
use std::os::unix::fs::PermissionsExt;
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
    pub permissions: String,
    pub last_modified: String,
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
                        let datetime: DateTime<Local> = f.metadata()?.modified()?.into();

                        FileInfo::new(
                            is_dir,
                            f.file_name().to_str().unwrap().to_string(),
                            size,
                            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                            format_permissions(f.metadata()?.permissions(), is_dir),
                        )
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

    pub fn father_path(&mut self) {
        self.last_dir_path = self.last_dir_path.parent().unwrap().to_path_buf();
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
    pub fn new(
        is_dir: bool,
        name: String,
        size: u64,
        last_modified: String,
        permissions: String,
    ) -> Result<Self, io::Error> {
        Ok(FileInfo {
            is_dir,
            name,
            size,
            permissions,
            last_modified,
        })
    }
}

fn format_permissions(permissions: fs::Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();

    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}

pub fn return_dir_vec(path: &Path) -> Result<Vec<FileInfo>, Box<dyn error::Error>> {
    match fs::read_dir(path) {
        Ok(dirs) => {
            let files_info: Result<Vec<_>, _> = dirs
                .filter_map(|f| f.ok())
                .map(|f| {
                    let datetime: DateTime<Local> = f.metadata()?.modified()?.into();
                    FileInfo::new(
                        f.file_type()?.is_dir(),
                        f.file_name().to_str().unwrap().to_string(),
                        f.metadata()?.len(),
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                        format_permissions(f.metadata()?.permissions(), f.file_type()?.is_dir()),
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
