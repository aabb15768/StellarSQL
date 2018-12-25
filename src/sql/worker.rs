use crate::component::database::Database;
use crate::component::table::Table;
use std::fmt;

pub struct SQL {
    pub database: Database,
}

#[derive(Debug)]
pub enum SQLError {}

impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl SQL {
    // Create a new database
    pub fn create_database(name: &str) -> Result<SQL, SQLError> {
        Ok(SQL {
            database: Database::new(name),
        })
    }

    // Load the database and create a new table
    pub fn create_table(db_name: &str, table: &Table) -> Result<SQL, SQLError> {
        let mut db = Database::load(db_name);
        db.insert_new_table(table.clone());
        Ok(SQL { database: db })
    }

    pub fn insert_into_table(
        db_name: &str,
        table_name: &str,
        attrs: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Result<SQL, SQLError> {
        let mut db = Database::load(db_name);
        Ok(SQL { database: db })
    }
}
