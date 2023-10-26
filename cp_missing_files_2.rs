use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let source_dir = "/home/Desktop/pdfs_1";
    let destination_dir = "/home/Downloads/pdfs_2";

    // Ensure the destination directory exists
    fs::create_dir_all(destination_dir)?;

    // Read the contents of the source directory
    let entries = fs::read_dir(source_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                // Check if the file is a PDF or XML
                if file_name_str.ends_with(".pdf") || file_name_str.ends_with(".xml") {
                    let destination_file = Path::new(destination_dir).join(file_name);

                    // Check if the file doesn't exist in the destination directory
                    if !destination_file.exists() {
                        fs::copy(&path, &destination_file)?;
                        println!("Copied {} to {}", file_name_str, destination_dir);
                    }
                }
            }
        }
    }

    println!("Comparison and copy already done!!!!");
    Ok(())
}
