use directories_next::UserDirs;
use std::fs;

fn main() {
    // Get the user's home directory
    if let Some(user_dirs) = UserDirs::new() {
        // Construct the path to the `.ssh` directory
        let ssh_dir = user_dirs.home_dir().join(".ssh");
        println!("SSH directory: {:?}", ssh_dir);

        // Specify the SSH key file (e.g., `id_rsa`)
        let key_file = ssh_dir.join("id_rsa");
        println!("SSH key file: {:?}", key_file);

        // Check if the file exists
        if key_file.exists() {
            // Read the file's contents
            match fs::read_to_string(&key_file) {
                Ok(key_content) => {
                    println!("SSH key content:\n{}", key_content);
                }
                Err(e) => {
                    eprintln!("Error reading SSH key file: {}", e);
                }
            }
        } else {
            eprintln!("SSH key file does not exist: {:?}", key_file);
        }
    } else {
        eprintln!("Failed to get user directories");
    }
}
