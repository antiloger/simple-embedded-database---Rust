use RDB::{DBtypes, Database, Table, Datatypes, DBERROR};

fn main(){
    let mut db = Database::new("test".to_string());

    let mut resourcesinfofields = vec![
        ("id".to_string(), Datatypes::Integers(0)),
        ("name".to_string(), Datatypes::Strings("".to_string())),
        ("description".to_string(), Datatypes::Strings("".to_string())),
    ];

    let mut resourcesinfo = Table::new("resouces_info".to_string(), "node1".to_string(), resourcesinfofields);
    let mut resources = DBtypes::new("resources".to_string(), resourcesinfo);

    let resource1fields = vec![
        ("id".to_string(), Datatypes::Integers(0)),
        ("name".to_string(), Datatypes::Strings("".to_string())),
        ("description".to_string(), Datatypes::Strings("".to_string())),
    ];

    let resource1 = Table::new("resource1".to_string(), "resources".to_string(), resource1fields);

    
    db.addtype(resources); // add resources to database
    //resources.add_info_table(resourcesinfo);
    // add table to resources


}
