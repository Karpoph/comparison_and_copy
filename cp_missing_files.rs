use std::fs;

fn main() {
    let source_dir = "/home/watens/Desktop/pdfs_1";
    let dest_dir = "/home/watens/Downloads/pdfs_2";

    for entry in fs::read_dir(source_dir).unwrap() {
        let file_path = entry.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".pdf") || file_name.ends_with(".xml") {
            let dest_file_path = format!("{}/{}", dest_dir, file_name);
            if !fs::metadata(&dest_file_path).is_ok() {
                fs::copy(file_path, dest_file_path).unwrap();
            }
        }
    }
}
