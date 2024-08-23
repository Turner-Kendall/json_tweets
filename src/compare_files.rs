use serde_json::Value;
use std::fs;
use std::io::Result;
use std::path::Path;

// Compares two JSON files, returns true if they are equal, false otherwise
fn are_json_files_equal(file1: &Path, file2: &Path) -> Result<bool> {
    let json1: Value = serde_json::from_reader(fs::File::open(file1)?)?;
    let json2: Value = serde_json::from_reader(fs::File::open(file2)?)?;
    Ok(json1 == json2)
}

// Removes duplicate JSON files in a given directory
fn remove_duplicate_files(directory: &Path) -> Result<()> {
    let mut files_to_remove = Vec::new();

    let files: Vec<_> = fs::read_dir(directory)?
        .filter_map(Result::ok)
        .collect();

    for i in 0..files.len() {
        for j in (i + 1)..files.len() {
            let file1 = files[i].path();
            let file2 = files[j].path();

            if are_json_files_equal(&file1, &file2)? {
                println!(
                    "Removing duplicate files: {:?} and {:?}",
                    file1.file_name().unwrap(),
                    file2.file_name().unwrap()
                );
                files_to_remove.push(file2);
            }
        }
    }

    // Once you have looped through all files, remove the duplicates
    for file in files_to_remove {
        fs::remove_file(file)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let directory_path = Path::new("./temp");
    println!("Checking for duplicate files in: {:?}", directory_path);
    remove_duplicate_files(directory_path)?;
    println!("Duplicate files removal complete.");

    Ok(())
}
