use RDB::{DBtypes, Database, Table, Datatypes};

fn main(){
    let mut db = Database::new("test".to_string());

    let resourcesinfofields = vec![
        ("id".to_string(), Datatypes::Integers(0)),
        ("name".to_string(), Datatypes::Strings("".to_string())),
        ("description".to_string(), Datatypes::Strings("".to_string())),
    ];

    let resourcesinfo = Table::new("resouces_info".to_string(), "node1".to_string(), resourcesinfofields);
    let resources = DBtypes::new("resources".to_string(), resourcesinfo);

    let resource1fields = vec![
        ("id".to_string(), Datatypes::Integers(0)),
        ("name".to_string(), Datatypes::Strings("".to_string())),
        ("description".to_string(), Datatypes::Strings("".to_string())),
    ];

    let resource1 = Table::new("resource1".to_string(), "resources".to_string(), resource1fields);

    
    db.addtype(resources).unwrap(); // add resources to database
    
    let res1 = db.get_dbtype("resources".to_string()).unwrap();
    res1.addtable(resource1).unwrap();
    
    let resource2fields = vec![
        ("id".to_string(), Datatypes::Integers(0)),
        ("name".to_string(), Datatypes::Strings("".to_string())),
        ("description".to_string(), Datatypes::Strings("".to_string())),
    ];

    let resource2 = Table::new("resource2".to_string(), "resources".to_string(), resource2fields);
    res1.addtable(resource2).unwrap();

    let mut tableRes1 = res1.get_table("resource1".to_string()).unwrap();

    let rowforres1 = vec![vec![
        Datatypes::Integers(1),
        Datatypes::Strings("res1".to_string()),
        Datatypes::Strings("res1 description".to_string()),
    ],
    vec![
        Datatypes::Integers(2),
        Datatypes::Strings("res2".to_string()),
        Datatypes::Strings("res2 description".to_string()),
    ],];

    tableRes1.insert_row(rowforres1[0].clone()).unwrap();
    tableRes1.insert_row(rowforres1[1].clone()).unwrap();    

    let rowdata = tableRes1.search_row(1).unwrap();
    println!("{:?}", rowdata);

}
