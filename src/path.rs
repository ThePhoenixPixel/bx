use std::error::Error;
use std::{fs, io};
use std::path::PathBuf;


pub struct Path;

impl Path {
    pub fn create_path(path: &PathBuf) {
        let mut current_path = PathBuf::new();

        for component in path.components() {
            current_path.push(component);

            if !current_path.exists() {
                fs::create_dir(&current_path).expect("Fehler beim Erstellen des Ordners");
            }
        }
    }

    pub fn copy_folder_contents(from: &PathBuf, to: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Für jede Datei/Verzeichnis im Quellordner
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let entry_path = entry.path();
            let target_path = to.join(entry_path.file_name().ok_or("Invalid file name")?);

            // Wenn es sich um ein Verzeichnis handelt, rufe die Funktion rekursiv auf
            if entry_path.is_dir() {
                fs::create_dir_all(&target_path)?;
                Self::copy_folder_contents(&entry_path, &target_path)?;
            } else {
                fs::copy(&entry_path, &target_path)?;
            }
        }
        Ok(())
    }

    pub fn extract_filename_from_path(path: &PathBuf) -> Option<String> {
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            // Die letzten Teile der Pfadzeichenkette nach Pfadtrennern ('\') oder ('/') trennen und ausgeben
            let parts: Vec<&str> = file_name.split(|c| c == '/' || c == '\\').collect();
            if let Some(last_part) = parts.last() {
                return Some(last_part.to_string());
            }
        }
        None
    }

    pub fn get_last_folder_name(path: &PathBuf) -> String {
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                return name_str.to_string();
            }
        }
        String::new()
    }
    pub fn get_folders_with_prefix(path: &PathBuf, prefix: &str) -> Result<Vec<String>, io::Error> {
        let mut folders = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    if file_name.starts_with(prefix) {
                        folders.push(file_name.to_string());
                    }
                }
            }
        }

        Ok(folders)
    }

    pub fn get_files_name_from_path(path: &PathBuf) -> Vec<String> {
        let mut files_name: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        files_name.push(file_name.to_string());
                    }
                }
            }
        }
        files_name
    }

    pub fn get_folders_name_from_path(path: &PathBuf) -> Vec<String> {
        let mut folders_name: Vec<String> = Vec::new();
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(folder_name) = entry.file_name().to_str() {
                        if let Some(metadata) = entry.metadata().ok() {
                            if metadata.is_dir() {
                                folders_name.push(folder_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        folders_name
    }

}
