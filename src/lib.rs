use std::collections::HashMap;
use chrono::{Utc, DateTime};

pub type DBResult<T> = std::result::Result<T, DBERROR>;

#[derive(Debug)]
pub enum DBERROR {
    InsertError,
    UpdateError,
    DeleteError,
    SelectError,
    ConnectionError,
    QueryError,
    NoDataError,
}

#[derive(Debug, Clone, PartialEq)]
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
    info: Vec<(Datatypes, Datatypes)>,
}

pub struct Table {
    name: String,
    node: String,
    fields: HashMap<String, ColumnGroup>,
    config: Vec<(String, String)>,
}

pub struct ColumnGroup{
    name: String,
    numofcolumns: u32,
    feilds: Vec<(String, Datatypes)>,
    columns: Vec<Vec<Datatypes>>,
}


impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name,
            create_time: Utc::now(),
            database: HashMap::new(),
        }
    }

    pub fn addtype(&mut self, dbtype: DBtypes) -> DBResult<()> {
        if self.database.contains_key(&dbtype.name) {
            return Err(DBERROR::InsertError);
        }
        self.database.insert(dbtype.name.clone(), dbtype);
        Ok(())
    }

    pub fn get_dbtype(&mut self, name: String) -> DBResult<&mut DBtypes> {
        if let Some(dbtype) = self.database.get_mut(&name) {
            Ok(dbtype)
        } else {
            Err(DBERROR::SelectError)
        }
    }
    
}

impl DBtypes {
    pub fn new(name: String, info: Vec<(Datatypes, Datatypes)>) -> DBtypes {
        DBtypes {
            name,
            numoftables: 0,
            tables: HashMap::new(),
            info,
        }
    }

    pub fn addtable(&mut self, table: Table) -> DBResult<()> {
        if self.tables.contains_key(&table.name) {
            return Err(DBERROR::InsertError);
        }
        self.tables.insert(table.name.clone(), table);
        self.numoftables += 1;
        Ok(())
    }

    //this should be change
    // pub fn add_info_table(&mut self, table: Table) -> DBResult<()> {
    //     self.info = table;
    //     Ok(())
    // }

    pub fn search_table(&self, name: String) -> DBResult<&Table> {
        if self.tables.contains_key(&name) {
            Ok(self.tables.get(&name).unwrap())
        } else {
            Err(DBERROR::SelectError)
        }
    }

    pub fn get_table(&mut self, name: String) -> DBResult<&mut Table> {
        if let Some(table) = self.tables.get_mut(&name) {
            Ok(table)
        } else {
            Err(DBERROR::SelectError)
        }
    }
}

impl Table {
    pub fn new(name: String, node: String) -> Table {
        Table {
            name,
            node,
            fields: HashMap::new(),
            config: Vec::new(),
        }
    }


}

impl ColumnGroup {
    pub fn new(name: String) -> ColumnGroup {
        ColumnGroup {
            name,
            numofcolumns: 0,
            feilds: Vec::new(),
            columns: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: (String, Datatypes)) -> DBResult<()> {
        if self.feilds.contains(&column) {
            return Err(DBERROR::InsertError);
        }
        self.feilds.push(column);
        self.numofcolumns += 1;
        Ok(())
    }

    pub fn validate_row(&self, row: &Vec<Datatypes>) -> DBResult<()> {
        if row.len() != self.numofcolumns as usize {
            return Err(DBERROR::InsertError);
        }
        for (i, (_, datatype)) in self.feilds.iter().enumerate() {
            if row[i] != *datatype {
                return Err(DBERROR::InsertError);
            }
        }
        Ok(())
    }

    pub fn insert_row(&mut self, row: Vec<Datatypes>) -> DBResult<()> {
        if let Err(e) = self.validate_row(&row) {
            return Err(e);
        }
        self.columns.push(row);
        Ok(())
    }


}

    




// impl Table {
//     pub fn new(name: String, node: String) -> Table {
//         Table {
//             name,
//             node,
//             fields: Vec::new(),
//             rows: Vec::new(),
//             config: Vec::new(),
//         }
//     }

//     pub fn add_field_group(&mut self, fieldgroup: String) -> DBResult<()> {
//         if self.fields.iter().any(|(name, _)| name == &fieldgroup) {
//             return Err(DBERROR::InsertError);
//         }
//         self.fields.push((fieldgroup, Vec::new()));
//         Ok(())
//     }

//     pub fn add_field(&mut self, fieldgroup: String, field: (String, Datatypes)) -> DBResult<()> {
//         if let Some((_, vec_field)) = self.fields.iter_mut().find(|(name, _)| name == &fieldgroup){
//             if vec_field.contains(&field) {
//                 return Err(DBERROR::InsertError);
//             }
//             vec_field.push(field);
//             Ok(())
//         } else {
//             Err(DBERROR::InsertError)
//         }
//     }

//     pub fn add_config(&mut self, config: (String, String)) -> DBResult<()> {
//         if self.config.contains(&config) {
//             return Err(DBERROR::InsertError);
//         }
//         self.config.push(config);
//         Ok(())
//     }

//     pub fn search_row(&self, index: usize) -> DBResult<Vec<Datatypes>> {
//         if index < self.rows.len() {
//             Ok(self.rows[index].clone())
//         } else {
//             Err(DBERROR::SelectError)
//         }
//     }

//     pub fn search_by_column(&self, columnname: &str, item: Datatypes) -> DBResult<Vec<Datatypes>>{
//         if let Some(index) = self.fields.iter().position(|(s, _)| s == columnname) {
//             for row in &self.rows {
//                 if row[index] == item {
//                     return Ok(row.clone());
//                 }
//             }
//             Err(DBERROR::NoDataError)
//         } else {
//             Err(DBERROR::SelectError)
//         }
//     }


//     pub fn insert_row(&mut self, row: Vec<Datatypes>) -> DBResult<()> {
//         self.rows.push(row);
//         Ok(())
//     }

//     pub fn update_row(&mut self, index: usize, row: Vec<Datatypes>) -> DBResult<()> {
//         if index < self.rows.len() {
//             self.rows[index] = row;
//             Ok(())
//         } else {
//             Err(DBERROR::UpdateError)
//         }
//     }


    
// }
