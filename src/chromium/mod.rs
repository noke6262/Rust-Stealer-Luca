pub mod main;
pub mod dumper;
pub mod decryption_core;
pub mod models;



pub fn grab(path: String) {
    main::chrome_main(path);
}