use ignore::Walk;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use rusqlite::Connection;
use std::sync::mpsc;
use std::time::Duration;

use crate::pulse::Pulse;

pub fn watch_activity(db: &Connection) {
    let (sender, receiver) = mpsc::channel();
    let mut watcher = watcher(sender, Duration::from_secs(3)).expect("Couldn't create the watcher");
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                if let Err(e) = watcher.watch(entry.path(), RecursiveMode::NonRecursive) {
                    println!("Error tyring to watch file: {:?}\n{}", entry.path(), e);
                } else {
                    println!("watching file: {}", entry.path().display());
                }
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }

    loop {
        let event = receiver.recv();
        match event {
            Ok(DebouncedEvent::NoticeWrite(path))
            | Ok(DebouncedEvent::Create(path))
            | Ok(DebouncedEvent::Write(path))
            | Ok(DebouncedEvent::Chmod(path)) => match Pulse::new(path) {
                Ok(pulse) => {
                    if let Err(e) = pulse.save(&db) {
                        println!("Error saving the pulse: {:?}\n{:?}", pulse, e)
                    } else {
                        println!("Saved pulse {} to database", pulse.id);
                    }
                }
                Err(e) => println!("Error creating pulse: {}", e),
            },
            Ok(_) => {}
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
