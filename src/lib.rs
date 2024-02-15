

use std::collections::HashMap;
use chrono::{Utc, DateTime};


pub enum DBERROR {
    InsertError,
    UpdateError,
    DeleteError,
    SelectError,
    ConnectionError,
    QueryError,
    NoDataError,
}

#[derive(Debug, Clone)]
pub enum Datatypes {
    Strings(String),
    Integers(i32),
    UIntegers(u32),
    Floats(f32),
    Doubles(f64),
    Booleans(bool),
    BigIntegers(i64),
    BigUIntegers(u64),
}

pub struct Database {
    name: String,
    create_time: DateTime<Utc>,
    database: HashMap<String, DBtypes>,
}

pub struct DBtypes {
    name: String,
    numoftables: u32,
    tables: HashMap<String, Table>,
    info: Table,
}

pub struct Table {
    name: String,
    node: String,
    fields: Vec<(String, Datatypes)>,
    rows: Vec<Vec<Datatypes>>,
    config: Vec<(String, String)>,
}

impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name,
            create_time: Utc::now(),
            database: HashMap::new(),
        }
    }

    pub fn addtype(&mut self, dbtype: DBtypes) -> Result<(), DBERROR> {
        if self.database.contains_key(&dbtype.name) {
            return Err(DBERROR::InsertError);
        }
        self.database.insert(dbtype.name.clone(), dbtype);
        Ok(())
    }
}

impl DBtypes {
    pub fn new(name: String, info: Table) -> DBtypes {
        DBtypes {
            name,
            numoftables: 0,
            tables: HashMap::new(),
            info,
        }
    }

    pub fn addtable(&mut self, table: Table) -> Result<(), DBERROR> {
        if self.tables.contains_key(&table.name) {
            return Err(DBERROR::InsertError);
        }
        self.tables.insert(table.name.clone(), table);
        self.numoftables += 1;
        Ok(())
    }

    //this should be change
    pub fn add_info_table(&mut self, table: Table) -> Result<(), DBERROR> {
        self.info = table;
        Ok(())
    }

    pub fn search_table(&self, name: String) -> Result<&Table, DBERROR> {
        if self.tables.contains_key(&name) {
            Ok(self.tables.get(&name).unwrap())
        } else {
            Err(DBERROR::SelectError)
        }
    }
}

impl Table {
    pub fn new(name: String, node: String, fields: Vec<(String, Datatypes)>) -> Table {
        Table {
            name,
            node,
            fields,
            rows: Vec::new(),
            config: Vec::new(),
        }
    }

    pub fn search_row(&self, index: usize) -> Result<Vec<Datatypes>, DBERROR> {
        if index < self.rows.len() {
            Ok(self.rows[index].clone())
        } else {
            Err(DBERROR::SelectError)
        }
    }

    pub fn insert_row(&mut self, row: Vec<Datatypes>) -> Result<(), DBERROR> {
        self.rows.push(row);
        Ok(())
    }

    pub fn update_row(&mut self, index: usize, row: Vec<Datatypes>) -> Result<(), DBERROR> {
        if index < self.rows.len() {
            self.rows[index] = row;
            Ok(())
        } else {
            Err(DBERROR::UpdateError)
        }
    }
}
