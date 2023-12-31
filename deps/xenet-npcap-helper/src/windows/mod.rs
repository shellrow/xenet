pub mod app;
pub mod npcap;
mod sys;

#[cfg(feature = "download")]
/// Setup npcap and npcap SDK.
/// This function will install npcap and npcap SDK if they are not installed.
pub fn setup_npcap() -> Result<(), String> {
    // Check if npcap is installed
    if !npcap::npcap_installed() {
        println!("Installing Npcap...");
        match npcap::install_npcap() {
            Ok(_) => println!("Npcap installed successfully !"),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Npcap is already installed !");
    }
    // Check if npcap sdk is installed
    if !npcap::npcap_sdk_installed() {
        println!("Installing Npcap SDK...");
        match npcap::install_npcap_sdk() {
            Ok(_) => println!("Npcap SDK installed successfully !"),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Npcap SDK is already installed !");
    }
    Ok(())
}
