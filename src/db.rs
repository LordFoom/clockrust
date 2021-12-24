use color_eyre::Report;
use rusqlite::{Connection, params};
use tracing::info;

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

    fn ensure_storage_exists(self, conn: &Connection) -> Result<(), Report> {
        //check for table's existence
        conn.execute("
            CREATE TABLE IF NOT EXISTS clock_rust_tasks(
                id INTEGER PRIMARY KEY ASC,
                command TEXT,
                task TEXT,
                hash INTEGER,
                cmd_date DATETIME
            )
        ", [])?;
        Ok(())
        //and create it if it does not exist
    }

    pub fn run_clock_command(self, cmd: Command) -> Result<(), Report> {
        //todo get some better pattern matching going on
        let conn = Connection::open(&self.connection_string)?;
        self.ensure_storage_exists(&conn)?;
        // con
        Ok(())
    }

    pub fn connection(&self)->Result<Connection, Report>{
        Ok(Connection::open(&self.connection_string.clone())?)
    }
}

#[cfg(test)]
mod tests{
    use color_eyre::Report;
    use tracing::error;
    use crate::config;

    use super::*;

    #[test]
    fn test_create_table(){
        config::setup(true);
        let db_file = "./clock_rust_test";
        let cr = ClockRuster::init(db_file);
        if let Ok(conn) = Connection::open(cr.connection_string.clone()){
                match cr.ensure_storage_exists(&conn){
                    Ok(_) => {info!("Successfully ran ensure_storage_exists")}
                    Err(why) => {panic!("Could not ensure_storage_exists: {}", why)}
                }
                let fp = std::path::Path::new(db_file);
                assert!(std::path::Path::exists(fp));
            //SELECT name FROM sqlite_master WHERE type='table' AND name='{table_name}';
            let tablecount = conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name='clock_rust_tasks'", []).unwrap();
            assert_eq!(tablecount, 1)

            }else{
            panic!("Failed to get connection");
        }

    }
}

