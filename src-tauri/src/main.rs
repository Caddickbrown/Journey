// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use std::io::Error;
use tauri::api::Result;

#[derive(Debug, Serialize, Deserialize)]
struct JournalEntry {
    title: String,
    content: String,
    // Add more fields as needed (e.g., date, tags, etc.)
}

fn read_entries() -> Result<Vec<JournalEntry>> {
    // Read entries from a file (JSON, YAML, or any format you prefer)
    unimplemented!() // Placeholder; you should implement the actual read logic
}

fn write_entries(entries: &[JournalEntry]) -> Result<()> {
    // Write entries to a file
    unimplemented!() // Placeholder; you should implement the actual write logic
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_entries,
            write_entries,
            __cmd__read_entries,
            __cmd__write_entries,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
