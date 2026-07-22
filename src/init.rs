use std::{fs, path::Path, env};
use configparser::ini;

use crate::repository;

#[derive(Debug)]
pub enum InitError {
    OS(String),
}

impl InitError {
    fn from_path(path: &str, message: String) -> Self {
        Self::OS(format!("Error resolving the path {path}: {message}"))
    }

}

fn create_folder(folder_name: &str) -> Result<(), InitError> {
    let creation_result = fs::create_dir_all(folder_name);
    if let Err(_) = creation_result {
        return Err(InitError::from_path(folder_name, creation_result.unwrap_err().to_string()))
    }
    Ok(())
}


fn create_file(file: (&str, &str)) -> Result<(), InitError> {
    let creation_result = fs::write(file.0, file.1);
    if let Err(_) = creation_result {
        return Err(InitError::from_path(file.0, creation_result.unwrap_err().to_string()))
    }
    Ok(())
}


fn create_folders(base_folder: &Path, folders: Vec<&str>) -> Result<(), InitError> {
    for sub in folders {
        let joined_path = base_folder.join(sub);
        let path = joined_path.to_str();
        if let None = path {
            return Err(InitError::from_path(sub, String::from("Problem creating the path for this folder")))
        }
        let creation_result = create_folder(path.unwrap());
        if let Err(err) = creation_result {
            return Err(err)
        }
    }
    Ok(())
}

fn create_files(base_folder: &Path, files: Vec<(&str, &str)>) -> Result<(), InitError> {
    for file in files {
        let joined_path = base_folder.join(file.0);
        let path = joined_path.to_str();
        if let None = path {
            return Err(InitError::from_path(file.0, String::from("Problem creating the path for this file")))
        }
        let creation_result = create_file((path.unwrap(), file.1));
        if let Err(err) = creation_result {
            return Err(err)
        }
    }
    Ok(())
}

fn create_config(path: &Path) -> Result<ini::Ini, InitError> {
    let path = path.join("config");
    let mut conf = ini::Ini::new();

    conf.set("core", "repositoryformatversion", Some("0".to_string()));
    conf.set("core", "filemode", Some("false".to_string()));
    conf.set("core", "bare", Some("false".to_string()));

    if let Err(e) = conf.write(&path) {
        return Err(InitError::from_path(path.to_str().unwrap(), e.to_string()))
    }

    Ok(conf)
}


pub fn init() -> Result<repository::Repository, InitError> {
    let act_folder = match env::current_dir() {
        Ok(path) => path,
        Err(err) => return Err(InitError::from_path(".", err.to_string()))
    };

    let base_folder = Path::new(".jims").to_path_buf();

    let subfolders = vec!["objects", "refs/heads", "refs/tags", "branches"];

    let folders_creation_result = create_folders(base_folder.as_path(), subfolders);
    if let Err(err) = folders_creation_result {
        return Err(err)
    }

    let files: Vec<(&str, &str)> = vec![
        ("HEAD", "ref: refs/heads/master\n"), 
        ("description", "Unnamed repository; edit this file 'description' to name the repository\n"),
    ];
    let files_creation_result = create_files(base_folder.as_path(), files);
    if let Err(err) = files_creation_result {
        return Err(err)
    }

    let conf = create_config(&base_folder);
    if let Err(err) = conf {
        return Err(err)
    }

    let repository = repository::Repository {
        worktree: act_folder,
        dir: base_folder,
        conf: conf.unwrap()
    };

    Ok(repository)
}