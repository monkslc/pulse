use rusqlite::{params, Connection};
use std::{error::Error, path::Path};

pub fn get_db() -> Result<Connection, Box<dyn Error>> {
    let home_dir = dirs::home_dir().expect("Can't get the home directory");
    let connection = Connection::open(Path::new(&home_dir).join(".pulse").join("pulses"))?;

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS pulses (
            id TEXT PRIMARY KEY,
            file_path TEXT,
            language TEXT,
            time INTEGER
        );
        CREATE INDEX IF NOT EXISTS time
        ON pulses(time);
        ",
        params![],
    )?;

    Ok(connection)
}
