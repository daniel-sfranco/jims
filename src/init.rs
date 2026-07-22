use std::{fs, path::Path};

#[derive(Debug)]
pub enum InitError {
    OS(String),
}

impl InitError {
    fn from_path(path: &str, message: String) -> Self {
        Self::OS(format!("Error creating the path {path}: {message}"))
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


pub fn init() -> Result<(), InitError> {
    let base_folder = Path::new(".jims");

    let subfolders = vec!["objects", "refs/heads", "refs/tags"];
    for sub in subfolders {
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

    let files: Vec<(&str, &str)> = vec![
        ("HEAD", "ref: refs/heads/master\n"), 
        ("description", "Unnamed repository; edit this file 'description' to name the repository\n"),
        ("config", "[core]\n    repositoryformatversion = 0\n    filemode = false\n    bare = false\n")
    ];
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