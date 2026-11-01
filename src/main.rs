mod minigrep_file_reader;


use std::fs;
use std::env::args;
use std::path::Path;
use minigrep_file_reader::{FileReader, MiniGrepError};

#[derive(PartialEq)]
enum EntryType {
    File,
    Directory,
}

type EntryResult = Result<EntryType, MiniGrepError>;


fn main() -> Result<(), MiniGrepError> {
    let args: Vec<String> = args().collect();
    
    let pattern = &args[1];

    let path = Path::new(&args[2]);
    
    let entry_result = entry_type(path)?;

    match entry_result {
        EntryType::Directory => {
            println!("Directory: {}", path.display());
            let path_string = path.to_string_lossy().to_string();
            let entries = directory_entris(&path_string)?;
            println!("{:?}", entries);
        }
        EntryType::File => {
            println!("File: {}", path.display());
        }
    }
    
    let file_reader = FileReader::new(path.to_string_lossy().to_string());
    let found = file_reader.find_pattern(pattern)?;
    println!("Found: {}", found);
    
    println!("{:?}", args);
    Ok(())
}


fn entry_type(path: &Path) -> EntryResult {
    let metadata = fs::metadata(path)?;
    if metadata.is_symlink() {
        return Err(MiniGrepError::Other("Symbolic link".to_string()).into());
    }

    if metadata.is_dir() {
        Ok(EntryType::Directory)
    } else if metadata.is_file() {
        Ok(EntryType::File)
    } else {
        Err(MiniGrepError::Other("Unknown entry type".to_string()))
    }
}

fn directory_entris(path: &String) -> Result<Vec<String>, MiniGrepError> {
    let dir_entries = fs::read_dir(path).map_err(|e| MiniGrepError::Io(e))?;
    let mut entries: Vec<String> = Vec::new();
    
    for entry in dir_entries {
        let entry_path = entry.map_err(|e| MiniGrepError::Io(e))?;
        
        let entry_type = entry_type(&entry_path.path())?;
        if entry_type == EntryType::File {
            entries.push(entry_path.path().to_string_lossy().to_string());
        } else if entry_type == EntryType::Directory {
            entries.append(&mut directory_entris(&entry_path.path().to_string_lossy().to_string())?);
        }
    }

    Ok(entries)
}