use directories_next::UserDirs;
use std::fs;
use log::{info, warn, error};
use env_logger;

fn main() {
    // Initialize the logger
    // env_logger::init();
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Get the user's home directory
    if let Some(user_dirs) = UserDirs::new() {
        // Construct the path to the `.ssh` directory
        let ssh_dir = user_dirs.home_dir().join(".ssh");
        info!("SSH directory: {:?}", ssh_dir);

        // Specify the SSH key file (e.g., `id_rsa`)
        let key_file = ssh_dir.join("id_rsa");
        info!("SSH key file: {:?}", key_file);

        // Check if the file exists
        if key_file.exists() {
            // Read the file's contents
            match fs::read_to_string(&key_file) {
                Ok(key_content) => {
                    info!("SSH key content:\n{}", key_content);
                }
                Err(e) => {
                    error!("Error reading SSH key file: {}", e);
                }
            }
        } else {
            warn!("SSH key file does not exist: {:?}", key_file);
        }
    } else {
        error!("Failed to get user directories");
    }
}
