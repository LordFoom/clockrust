use color_eyre::Report;
use rusqlite::{Connection, params};

use crate::command::Command;

pub struct ClockRuster {
    connection_string: String,
}

impl ClockRuster {
    pub fn new() -> Self {
        Self {
            connection_string: String::from("./.clockrust"),
        }
    }

    pub fn init(conn_str: &str) -> Self {
        let connection_string = String::from(conn_str);
        Self {
            connection_string
        }
    }

    fn ensure_storage_exists(self, conn: Connection) -> Result<(), Report> {
        //check for table's existence
        conn.execute("
            CREATE TABLE clock_rust_tasks(
                id INTEGER PRIMARY KEY ASC,
                command: TEXT,
                task: TEXT,
                hash: INTEGER,
                cmd_date: DATETIME
            )
        ", [])?;
        Ok(())
        //and create it if it does not exist
    }

    pub fn run_clock_command(self, cmd: Command) -> Result<(), Report> {
        let conn = Connection::open(&self.connection_string)?;
        self.ensure_storage_exists(conn)?;
        Ok(())
    }
}

