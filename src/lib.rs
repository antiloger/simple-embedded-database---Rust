use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
    TestError,
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
    pub database: HashMap<String, DBtypes>,
}

pub struct DBtypes {
    name: String,
    numoftables: u32,
    pub tables: HashMap<String, Table>,
    info: Vec<(String, String)>,
}

pub struct Table {
    name: String,
    node: String,
    numofcolumns: u32,
    pub fields: HashMap<String, ColumnGroup>,
    config: Vec<(String, String)>,
}

pub struct ColumnGroup {
    name: String,
    numofcolumns: u32,
    pub feilds: Vec<(String, Datatypes)>,
    pub columns: Vec<Vec<Datatypes>>,
}

impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name,
            create_time: Utc::now(),
            database: HashMap::new(),
        }
    }

    pub fn get_info(&self) -> Vec<(String, String)> {
        let numoftypes = self.database.len();
        let totalnumtables = self
            .database
            .iter()
            .fold(0, |acc, (_, dbtype)| acc + dbtype.numoftables);

        return vec![
            ("name".to_string(), self.name.clone()),
            ("create_time".to_string(), self.create_time.to_string()),
            ("numoftypes".to_string(), numoftypes.to_string()),
            ("totalnumtables".to_string(), totalnumtables.to_string()),
        ];
    }

    pub fn addtype(&mut self, dbtype: DBtypes) -> DBResult<&DBtypes> {
        if self.database.contains_key(&dbtype.name) {
            return Err(DBERROR::InsertError);
        }
        let dbtypename = dbtype.name.clone();
        self.database.insert(dbtype.name.clone(), dbtype);

        if let Some(refdbtype) = self.database.get(&dbtypename) {
            Ok(refdbtype)
        } else {
            Err(DBERROR::InsertError)
        }
    }

    pub fn create_type(&mut self, name: String) -> DBResult<&DBtypes> {
        let new = DBtypes::new(name);
        self.addtype(new)
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
    pub fn new(name: String) -> DBtypes {
        DBtypes {
            name,
            numoftables: 0,
            tables: HashMap::new(),
            info: Vec::new(),
        }
    }

    pub fn create_table(&mut self, name: String, node: String) -> DBResult<&Table> {
        let new = Table::new(name, node);
        self.addtable(new)
    }

    pub fn addtable(&mut self, table: Table) -> DBResult<&Table> {
        if self.tables.contains_key(&table.name) {
            return Err(DBERROR::InsertError);
        }

        let tablename = table.name.clone();
        self.tables.insert(table.name.clone(), table);
        self.numoftables += 1;

        if let Some(reftable) = self.tables.get(&tablename) {
            Ok(reftable)
        } else {
            Err(DBERROR::InsertError)
        }
    }

    pub fn add_config(&mut self, config: (String, String)) -> DBResult<()> {
        if self.info.contains(&config) {
            return Err(DBERROR::InsertError);
        }
        self.info.push(config);
        Ok(())
    }

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
            numofcolumns: 0,
            fields: HashMap::new(),
            config: Vec::new(),
        }
    }

    pub fn get_info(&self) -> Vec<(String, String)> {
        let numofcolumngroups = self.fields.len();
        let totalnumcolumns = self
            .fields
            .iter()
            .fold(0, |acc, (_, column)| acc + column.numofcolumns);

        return vec![
            ("name".to_string(), self.name.clone()),
            ("node".to_string(), self.node.clone()),
            (
                "numofcolumngroups".to_string(),
                numofcolumngroups.to_string(),
            ),
            ("totalnumcolumns".to_string(), totalnumcolumns.to_string()),
        ];
    }

    pub fn add_columngroup(&mut self, column: ColumnGroup) -> DBResult<&mut ColumnGroup> {
        if self.fields.contains_key(column.name.as_str()) {
            return Err(DBERROR::InsertError);
        }

        let columngname = column.name.clone();
        self.fields.insert(column.name.to_string(), column);
        self.numofcolumns += 1;

        if let Some(refcolumn) = self.fields.get_mut(&columngname) {
            Ok(refcolumn)
        } else {
            Err(DBERROR::InsertError)
        }
    }

    pub fn create_columngroup(&mut self, name: String) -> DBResult<&mut ColumnGroup> {
        let new = ColumnGroup::new(name);
        self.add_columngroup(new)
    }

    pub fn add_config(&mut self, config: (String, String)) -> DBResult<()> {
        if self.config.contains(&config) {
            return Err(DBERROR::InsertError);
        }
        self.config.push(config);
        Ok(())
    }

    pub fn search_columngroup(&mut self, name: String) -> DBResult<&mut ColumnGroup> {
        if self.fields.contains_key(&name) {
            Ok(self.fields.get_mut(&name).unwrap())
        } else {
            Err(DBERROR::SelectError)
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
            //println!("{:?}, {:?}", &row[i], datatype);
            if !&row[i].is_same_type(datatype) {
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

    pub fn update_row(
        &mut self,
        columnname: &str,
        item: Datatypes,
        row: Vec<Datatypes>,
    ) -> DBResult<()> {
        if let Some(index) = self.feilds.iter().position(|(s, _)| s == columnname) {
            for (i, column) in self.columns.iter_mut().enumerate() {
                if column[index] == item {
                    if let Err(e) = self.validate_row(&row) {
                        return Err(e);
                    }
                    self.columns[i] = row;
                    return Ok(());
                }
            }
            return Err(DBERROR::UpdateError);
        }
        Err(DBERROR::SelectError)
    }

    pub fn delete_row(&mut self, columnname: &str, item: Datatypes) -> DBResult<()> {
        if let Some(index) = self.feilds.iter().position(|(s, _)| s == columnname) {
            for (i, column) in self.columns.iter().enumerate() {
                if column[index] == item {
                    self.columns.remove(i);
                    return Ok(());
                }
            }
            return Err(DBERROR::DeleteError);
        }
        Err(DBERROR::SelectError)
    }

    pub fn search_row(&self, columname: &str, item: Datatypes) -> DBResult<Vec<Datatypes>> {
        if let Some(index) = self.feilds.iter().position(|(s, _)| s == columname) {
            for row in self.columns.iter() {
                if row[index] == item {
                    return Ok(row.clone());
                }
            }
            return Err(DBERROR::NoDataError);
        }
        Err(DBERROR::SelectError)
    }

    pub fn get_column(&self, column: String) -> DBResult<Vec<Datatypes>> {
        if let Some(index) = self.feilds.iter().position(|(s, _)| s == &column) {
            let mut result = Vec::new();
            for row in self.columns.iter() {
                result.push(row[index].clone());
            }
            return Ok(result);
        }
        Err(DBERROR::SelectError)
    }
}

impl Datatypes {
    pub fn is_same_type(&self, other: &Datatypes) -> bool {
        match (self, other) {
            (Datatypes::Strings(_), Datatypes::Strings(_)) => true,
            (Datatypes::Integers(_), Datatypes::Integers(_)) => true,
            (Datatypes::UIntegers(_), Datatypes::UIntegers(_)) => true,
            (Datatypes::Floats(_), Datatypes::Floats(_)) => true,
            (Datatypes::Doubles(_), Datatypes::Doubles(_)) => true,
            (Datatypes::Booleans(_), Datatypes::Booleans(_)) => true,
            (Datatypes::BigIntegers(_), Datatypes::BigIntegers(_)) => true,
            (Datatypes::BigUIntegers(_), Datatypes::BigUIntegers(_)) => true,
            _ => false,
        }
    }
}

pub fn create_dbtype_set(database: &mut Database, dbtypename: Vec<String>) -> DBResult<()> {
    for dbtype in dbtypename {
        database.create_type(dbtype)?;
    }

    Ok(())
}

pub fn create_table_set(dbtype: &mut DBtypes, tablename: Vec<String>) -> DBResult<()> {
    for table in tablename {
        let node = dbtype.name.clone();
        dbtype.create_table(table, node)?;
    }

    Ok(())
}

pub fn create_columngroup_set(table: &mut Table, columngroupname: Vec<String>) -> DBResult<()> {
    for columngroup in columngroupname {
        table.create_columngroup(columngroup)?;
    }

    Ok(())
}

