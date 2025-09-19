# 🦀 Rust Key-Value Store

A simple yet powerful in-memory key-value store implemented in Rust with an interactive command-line interface.

## 📋 Features

- **In-memory store**: HashMap-based for optimal performance
- **Interactive CLI interface**: Intuitive commands with colorful feedback
- **Complete operations**: SET, GET, REMOVE, LIST and more
- **Robust error handling**: Input validation and clear messages
- **Zero dependencies**: Uses only Rust's standard library

## 🚀 Installation

### Prerequisites
- Rust 1.70+ (edition 2024)
- Cargo

### Building
```bash
git clone https://github.com/gabrycaos/rust-kv.git
cd rust-kv
cargo build --release
```

### Running
```bash
cargo run
```

## 💻 Usage

Once you start the program, you'll see an interactive menu:

```
🚀 Welcome to the Rust Key-Value Store!

=== RUST KEY-VALUE STORE ===
1. SET <key> <value>    - Insert/update a key
2. GET <key>            - Retrieve the value of a key
3. REMOVE <key>         - Remove a key
4. KEYS                 - Show all keys
5. VALUES               - Show all values
6. LIST                 - Show all key-value pairs
7. LEN                  - Show the number of elements
8. CLEAR                - Clear the entire store
9. HELP                 - Show this menu
10. EXIT                - Exit the program
==============================

rust-kv>
```

### 📝 Command Examples

```bash
# Insert a key-value pair
rust-kv> SET name Mario

# Insert a value with spaces
rust-kv> SET message "Hello world from Rust"

# Retrieve a value
rust-kv> GET name
📄 name: Mario

# Show all keys
rust-kv> KEYS
🔑 Keys: ["name", "message"]

# Show all content
rust-kv> LIST
📚 Store contents:
  name => Mario
  message => Hello world from Rust

# Remove a key
rust-kv> REMOVE name
✅ Removed key: name

# Check the size
rust-kv> LEN
📊 Number of elements: 1

# Clear the entire store
rust-kv> CLEAR
🧹 Store cleared

# Exit the program
rust-kv> EXIT
👋 Goodbye!
```

## 🏗️ Architecture

The project is structured in two main modules:

### `src/store.rs`
Implements the `Store` structure with the following operations:
- `new()` - Creates a new empty store
- `set(key, value)` - Inserts/updates a key-value pair
- `get(key)` - Retrieves a value by key
- `remove(key)` - Removes a key
- `keys()` - Returns all keys
- `values()` - Returns all values
- `len()` - Number of elements
- `is_empty()` - Checks if the store is empty
- `clear()` - Empties the store
- `iter()` - Iterator over key-value pairs

### `src/main.rs`
Implements the interactive user interface with:
- User input loop
- Command parsing and validation
- Error handling and feedback
- Help menu

## 🔧 Store API

```rust
use rust_kv::store::Store;

let mut store = Store::new();

// Basic operations
store.set("key".to_string(), "value".to_string());
let value = store.get("key".to_string());
store.remove("key".to_string());

// Query operations
let keys = store.keys();
let values = store.values();
let size = store.len();
let empty = store.is_empty();

// Cleanup
store.clear();
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Test with verbose output
cargo test -- --nocapture

# Specific tests
cargo test store_tests
```

## 📊 Performance

- **Time complexity**:
  - SET: O(1) amortized
  - GET: O(1) amortized
  - REMOVE: O(1) amortized
  - KEYS/VALUES: O(n)
  - LIST: O(n)

- **Space complexity**: O(n) where n is the number of key-value pairs

## 🛠️ Development

### Project structure
```
rust-kv/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # Documentation
├── .gitignore          # Files to ignore in Git
└── src/
    ├── main.rs         # CLI interface
    └── store.rs        # Store implementation
```

### Release build
```bash
cargo build --release
```

The optimized executable will be available at `target/release/rust_kv`.

## 📈 Roadmap

- [ ] Disk persistence (JSON/Binary)
- [ ] Support for complex data types
- [ ] HTTP REST API
- [ ] Clustering and replication
- [ ] Data compression
- [ ] Metrics and monitoring

## 🤝 Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is released under the GNU GENERAL PUBLIC  LICENSE version 3.0. See the `LICENSE` file for details.

## 👨‍💻 Author

gabrycaos - [GitHub](https://github.com/gabrycaos)

## 🙏 Acknowledgments

- The Rust community for excellent documentation
- Cargo maintainers for the fantastic build system