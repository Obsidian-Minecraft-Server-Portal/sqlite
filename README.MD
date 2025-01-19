# Obsidian SQLite Library

This is a Rust library designed for use with the [Obsidian Server Portal](https://github.com/obsidian-server-portal/obsidian-server-portal), a Minecraft server panel. The library simplifies managing SQLite databases specifically tailored for integration with the Obsidian Server Portal.

## Features

- Create and manage SQLite database connections.
- Provides utilities to work with the Obsidian Server Portal's data storage needs.
- Supports creating and managing database tables for server configurations, player data, and more.

## Installation

This library is only intended to be added as a Git dependency. To add it to your project, include it in your `Cargo.toml`:

```toml
[dependencies]
obsidian-sqlite = { git = "https://github.com/your-repo/obsidian-sqlite" }
```

> Replace `https://github.com/your-repo/obsidian-sqlite` with the actual repository URL for this library.

## Usage

To use this library, include it in your project and leverage its utilities for SQLite database management.

Here’s a simple example of creating a database table:

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let conn = obsidian_sqlite::create_appdb_connection()?;
    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);")?;
    Ok(())
}
```

### Features in Detail

- **Database Connection Management**: Easily create and manage connections to the SQLite database that powers the server panel.
- **Table Creation Utilities**: Create or update tables required by the server portal with ease.

## Contributing

Contributions are welcome! If you’d like to improve this library, feel free to open a pull request or an issue in the repository.

## License

This library is licensed under the MIT License. See the `LICENSE` file for more information.

---

This library is part of the Obsidian Server Portal ecosystem. For further details on the full project, visit the [Obsidian Server Portal repository](https://github.com/obsidian-server-portal/obsidian-server-portal).