use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

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

    let iter = stmt.query_map(params![], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for it in iter {
        println!("{:?}", it.unwrap());
    }

    Ok(())
}
