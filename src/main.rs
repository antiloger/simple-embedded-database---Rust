use RDB::{Database, Datatypes, DBResult, create_dbtype_set, create_table_set, create_columngroup_set};
use rand::Rng;

fn main() -> DBResult<()> {
    let mut database = Database::new("MyDatabase".to_string());

    // Create 10 DBtypes
    for i in 0..10 {
        let dbtype_name = format!("DBType{}", i);
        create_dbtype_set(&mut database, vec![dbtype_name.clone()])?;

        if let Ok(dbtype) = database.get_dbtype(dbtype_name) {
            // Create 5 Tables for each DBtype
            for j in 0..5 {
                let table_name = format!("Table{}", j);
                create_table_set(dbtype, vec![table_name.clone()])?;

                if let Ok(table) = dbtype.get_table(table_name) {
                    // Create 3 ColumnGroups for each Table
                    for k in 0..3 {
                        let columngroup_name = format!("ColumnGroup{}", k);
                        create_columngroup_set(table, vec![columngroup_name.clone()])?;

                        if let Ok(columngroup) = table.search_columngroup(columngroup_name) {
                            // Create 2 columns of type String for each ColumnGroup
                            for l in 0..2 {
                                let column_name = format!("Column{}", l);
                                columngroup.add_column((column_name, Datatypes::Strings("".to_string())))?;
                            }
                        }
                    }
                }
            }
        }
    }

    add_dummy_data(&mut database)?;
    print_database(&database)?;

    Ok(())
}

pub fn add_dummy_data(database: &mut Database) -> DBResult<()> {
    let mut rng = rand::thread_rng();

    for dbtype in database.database.values_mut() {
        for table in dbtype.tables.values_mut() {
            for columngroup in table.fields.values_mut() {
                for _ in 0..100 {
                    let mut row = Vec::new();
                    for (_, datatype) in &columngroup.feilds {
                        let dummy_data = match datatype {
                            Datatypes::Strings(_) => Datatypes::Strings(rng.gen::<u32>().to_string()),
                            Datatypes::Integers(_) => Datatypes::Integers(rng.gen()),
                            Datatypes::UIntegers(_) => Datatypes::UIntegers(rng.gen()),
                            Datatypes::Floats(_) => Datatypes::Floats(rng.gen()),
                            Datatypes::Doubles(_) => Datatypes::Doubles(rng.gen()),
                            Datatypes::Booleans(_) => Datatypes::Booleans(rng.gen()),
                            Datatypes::BigIntegers(_) => Datatypes::BigIntegers(rng.gen()),
                            Datatypes::BigUIntegers(_) => Datatypes::BigUIntegers(rng.gen()),
                        };
                        row.push(dummy_data);
                    }
                    columngroup.insert_row(row)?;
                }
            }
        }
    }

    Ok(())
}

pub fn print_database(database: &Database) -> DBResult<()> {
    for (dbtype_name, dbtype) in &database.database {
        println!("DBType: {}", dbtype_name);
        for (table_name, table) in &dbtype.tables {
            println!("  Table: {}", table_name);
            for (columngroup_name, columngroup) in &table.fields {
                println!("    ColumnGroup: {}", columngroup_name);
                for (column_name, datatype) in &columngroup.feilds {
                    print!("      Column: {}", column_name);
                    match datatype {
                        Datatypes::Strings(value) => println!(" Value: {}", value),
                        Datatypes::Integers(value) => println!(" Value: {}", value),
                        Datatypes::UIntegers(value) => println!(" Value: {}", value),
                        Datatypes::Floats(value) => println!(" Value: {}", value),
                        Datatypes::Doubles(value) => println!(" Value: {}", value),
                        Datatypes::Booleans(value) => println!(" Value: {}", value),
                        Datatypes::BigIntegers(value) => println!(" Value: {}", value),
                        Datatypes::BigUIntegers(value) => println!(" Value: {}", value),
                    }
                }
            }
        }
    }

    Ok(())
}