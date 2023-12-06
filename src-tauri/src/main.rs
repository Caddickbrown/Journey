use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Entry {
    text: String,
    // Add any other fields you need for an entry
}

// Function to handle the "new_entry" command
#[tauri::command]
fn new_entry(entry: Entry) -> Result<Vec<Entry>, tauri::Error> {
    // Read existing entries from the JSON file
    let entries = read_entries_from_file()?;
    println!("Existing entries: {:?}", entries);

    // Add the new entry
    let mut updated_entries = entries.clone();
    updated_entries.push(entry.clone());  // Clone the entry here
                                          
    // Save the updated entries to the JSON file
    save_entries_to_file(&updated_entries)?;
    println!("Entry added: {:?}", entry);

    Ok(updated_entries)
}

// Function to read entries from the JSON file
fn read_entries_from_file() -> Result<Vec<Entry>, io::Error> {
    let file_path = "path/to/your/entries.json"; // Update with your file path
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let entries: Vec<Entry> = serde_json::from_str(&content)?;

    Ok(entries)
}

// Function to save entries to the JSON file
fn save_entries_to_file(entries: &[Entry]) -> Result<(), io::Error> {
    let file_path = "path/to/your/entries.json"; // Update with your file path
    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    let content = serde_json::to_string_pretty(entries)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
