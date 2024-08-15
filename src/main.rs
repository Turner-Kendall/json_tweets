use std::collections::HashMap;
use tokio::fs;

#[derive(serde::Serialize, serde::Deserialize)]
struct InputRecord {
    created_at: String,
    username: String,
    full_text: String,
    id_str: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Content {
    name: String,
    text: String,
    date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string("input.json").await?;
    let records: Vec<InputRecord> = serde_json::from_str(&data)?;
    let mut map: HashMap<String, Content> = HashMap::new();

    for record in records {
        let content = Content {
            name: record.username.clone(),
            text: record.full_text,
            date: record.created_at,
        };

        map.insert(record.id_str, content);
    }

    // Serialize the HashMap to a JSON string
    let output_data: String = serde_json::to_string_pretty(&map)?;
    fs::write("output.json", output_data).await?;
    Ok(())
}
