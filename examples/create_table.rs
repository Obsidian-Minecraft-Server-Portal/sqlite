use anyhow::Result;

fn main() -> Result<()> {
    let conn = obsidian_sqlite::create_appdb_connection()?;
    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);")?;
    Ok(())
}
