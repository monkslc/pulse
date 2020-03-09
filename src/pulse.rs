use rusqlite::{params, Connection};
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Pulse {
    pub id: Uuid,
    pub file_path: PathBuf,
    pub language: String,
    pub time: u32,
}

impl Pulse {
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn Error>> {
        // Sqlite only supports up to u32, which is why we cast from u64
        let secs_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as u32;

        let language = match file_path.extension() {
            Some(extension) => extension.to_string_lossy().into_owned(),
            None => String::from(""),
        };
        Ok(Pulse {
            id: Uuid::new_v4(),
            file_path,
            language,
            time: secs_since_epoch,
        })
    }

    pub fn save(&self, db: &Connection) -> Result<usize, Box<dyn Error>> {
        let num_rows = db.execute(
            " INSERT INTO pulses (id, file_path, language, time)
        VALUES (?1, ?2, ?3, ?4)",
            params![
                self.id.to_hyphenated().to_string(),
                self.file_path.to_str(),
                self.language,
                self.time
            ],
        )?;
        Ok(num_rows)
    }
}

//TODO: come back to this later
pub fn log_pulses(db: &Connection, start_time: u32, end_time: u32) -> Result<(), Box<dyn Error>> {
    let mut statement = db
        .prepare("SELECT id, file_path, language, time FROM pulses WHERE time > ?1 AND time < ?2")
        .expect("Invalid Query");
    let mut rows = statement.query(params![start_time, end_time])?;
    while let Ok(Some(row)) = rows.next() {
        let id: String = row.get_unwrap(0);
        let file_path: String = row.get_unwrap(1);
        let language: String = row.get_unwrap(2);
        let time: u32 = row.get_unwrap(3);
        let pulse = Pulse {
            id: Uuid::parse_str(&id).expect("Not a uuid"),
            file_path: PathBuf::from(file_path),
            language,
            time,
        };
        println!(
            "{}, {:?}, {}, {}",
            pulse.id, pulse.file_path, pulse.language, pulse.time
        );
    }

    Ok(())
}
