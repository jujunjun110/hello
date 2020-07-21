use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;
    cn.execute(
        "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)",
        params![],
    )?;
    let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?, ?, ?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    dbg!("WORKING!");

    let mut stmt = cn.prepare("SELECT * FROM users")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        println!("{}{}", id, name);
    }
    Ok(())
}
