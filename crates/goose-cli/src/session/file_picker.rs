use anyhow::Result;
use ignore::WalkBuilder;
use nucleo_picker::{render::StrRenderer, Picker};
use std::path::{Path, PathBuf};
use std::thread;

pub fn show_file_picker(root_dir: &Path) -> Result<Option<PathBuf>> {
    // Create a picker with string renderer
    let mut picker = Picker::new(StrRenderer);
    
    // Get the injector to add files
    let injector = picker.injector();
    let root_dir_clone = root_dir.to_path_buf();
    
    // Spawn a thread to collect files and send them to the picker
    thread::spawn(move || {
        let walker = WalkBuilder::new(&root_dir_clone)
            .git_ignore(true)
            .max_depth(Some(10)) // Reasonable limit for performance
            .build();
            
        for entry in walker {
            if let Ok(entry) = entry {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    // Convert to relative path for nicer display
                    if let Ok(relative) = entry.path().strip_prefix(&root_dir_clone) {
                        let path_str = relative.to_string_lossy().to_string();
                        injector.push(path_str);
                    }
                }
            }
        }
    });
    
    // Run the picker and get the result
    match picker.pick()? {
        Some(selected_file) => {
            let full_path = root_dir.join(&selected_file);
            Ok(Some(full_path))
        }
        None => Ok(None),
    }
} 