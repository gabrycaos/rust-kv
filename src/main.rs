mod store;

use std::io::{self, Write};

/// Displays the menu of available operations
fn display_menu() {
    println!("\n=== RUST KEY-VALUE STORE ===");
    println!("1. SET <key> <value>    - Insert/update a key");
    println!("2. GET <key>            - Retrieve the value of a key");
    println!("3. REMOVE <key>         - Remove a key");
    println!("4. KEYS                 - Show all keys");
    println!("5. VALUES               - Show all values");
    println!("6. LIST                 - Show all key-value pairs");
    println!("7. LEN                  - Show the number of elements");
    println!("8. CLEAR                - Clear the entire store");
    println!("9. HELP                 - Show this menu");
    println!("10. EXIT                - Exit the program");
    println!("==============================");
}

/// Reads user input
fn read_input() -> String {
    print!("rust-kv> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()
}

/// Processes the user command
fn process_command(store: &mut store::Store, command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.is_empty() {
        return true;
    }
    
    match parts[0].to_uppercase().as_str() {
        "SET" => {
            if parts.len() < 3 {
                println!("❌ Usage: SET <key> <value>");
                return true;
            }
            let key = parts[1].to_string();
            let value = parts[2..].join(" ");
            store.set(key.clone(), value.clone());
            println!("✅ Set: {} = {}", key, value);
        },
        "GET" => {
            if parts.len() < 2 {
                println!("❌ Usage: GET <key>");
                return true;
            }
            let key = parts[1].to_string();
            match store.get(key.clone()) {
                Some(value) => println!("📄 {}: {}", key, value),
                None => println!("❌ Key '{}' not found", key),
            }
        },
        "REMOVE" => {
            if parts.len() < 2 {
                println!("❌ Usage: REMOVE <key>");
                return true;
            }
            let key = parts[1].to_string();
            if store.get(key.clone()).is_some() {
                store.remove(key.clone());
                println!("✅ Removed key: {}", key);
            } else {
                println!("❌ Key '{}' not found", key);
            }
        },
        "KEYS" => {
            let keys = store.keys();
            if keys.is_empty() {
                println!("📭 No keys present");
            } else {
                println!("🔑 Keys: {:?}", keys);
            }
        },
        "VALUES" => {
            let values = store.values();
            if values.is_empty() {
                println!("📭 No values present");
            } else {
                println!("📋 Values: {:?}", values);
            }
        },
        "LIST" => {
            if store.is_empty() {
                println!("📭 Store is empty");
            } else {
                println!("📚 Store contents:");
                for (key, value) in store.iter() {
                    println!("  {} => {}", key, value);
                }
            }
        },
        "LEN" => {
            println!("📊 Number of elements: {}", store.len());
        },
        "CLEAR" => {
            store.clear();
            println!("🧹 Store cleared");
        },
        "HELP" => {
            display_menu();
        },
        "EXIT" => {
            println!("👋 Goodbye!");
            return false;
        },
        _ => {
            println!("❌ Unrecognized command: '{}'. Type HELP to see available commands.", parts[0]);
        }
    }
    
    true
}

fn main() {
    let mut store = store::Store::new();
    
    println!("🚀 Welcome to the Rust Key-Value Store!");
    display_menu();
    
    loop {
        let input = read_input();
        
        if !process_command(&mut store, &input) {
            break;
        }
    }
}