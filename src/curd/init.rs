use rusqlite:: {
    Connection,
    Result as SqliteResult
};

pub fn init_sql() -> SqliteResult<()> {
    let _ = std::fs::create_dir_all("./data");
    let conn_data = Connection::open("./data/data.db")?;
    let conn_log = Connection::open("./data/log.db")?;

    conn_data.execute(
        "CREATE TABLE student (
            id INTEGER PRIMARY KEY NOT NULL, 
            name TEXT NOT NULL,
            credits REAL NOT NULL
        )", 
        ()
    )?;

    conn_data.execute(
        "CREATE TABLE describe (
            state INTEGER NOT NULL,
            description TEXT NOT NULL
        )", 
        ()
    )?;

    conn_data.execute(
        "CREATE TABLE user (
            user_name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        () 
    )?;
    conn_data.execute("INSERT INTO user (user_name, password) VALUES (?1, ?2)", ("admin", "123456")).unwrap();

    conn_log.execute(
        "CREATE TABLE log (
            time TEXT NOT NULL,
            id INTEGER NOT NULL,
            name TEXT NOT NULL,
            credits REAL NOT NULL,
            description TEXT NOT NULL
        )", 
        ()
    )?;

    Ok(())
}