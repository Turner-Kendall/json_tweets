use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let directory = Path::new("./data");

    // Iterate over the files in the directory
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // Process only .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap().to_string();
            let mut file = File::open(&path)?;

            // Read and parse the JSON file
            let mut data: Value = serde_json::from_reader(&file)?;
            
            // Add 'keyword' field to each record in the JSON file
            if let Some(array) = data.as_array_mut() {
                for record in array {
                    if let Some(obj) = record.as_object_mut() {
                        obj.insert("keyword".to_string(), json!(filename));
                    }
                }
            }

            // Move the file pointer to the beginning of the file
            file.seek(SeekFrom::Start(0))?;

            // Write the updated JSON back to the file
            file.set_len(0)?;  // Truncate the file
            serde_json::to_writer_pretty(&file, &data)?;

            println!("Updated {:?} with the 'keyword' field.", path.file_name().unwrap());
        }
    }

    Ok(())
}
