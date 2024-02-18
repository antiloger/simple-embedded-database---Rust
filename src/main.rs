use RDB::{Database, Datatypes};

fn main() {
    //create a new database
    let mut db = Database::new("test".to_string());

    //create a new DBtype
    db.create_type("restype".to_string()).unwrap();
    db.create_type("contype".to_string()).unwrap();

    //create a new table
    let restype = db.get_dbtype("restype".to_string()).unwrap();
    restype
        .create_table("res".to_string(), "restype".to_string())
        .unwrap();
    restype
        .create_table("restable2".to_string(), "restype".to_string())
        .unwrap();

    //create a column group
    let res = restype.get_table("res".to_string()).unwrap();
    let resgroup = res.create_columngroup("resgroup".to_string()).unwrap();

    //add a column to the column group
    resgroup
        .add_column(("rescol1".to_string(), Datatypes::Integers(0)))
        .unwrap();
    resgroup
        .add_column((
            "rescol2".to_string(),
            Datatypes::Strings("none".to_string()),
        ))
        .unwrap();
    resgroup
        .add_column(("rescol3".to_string(), Datatypes::Booleans(false)))
        .unwrap();

    // let resul = same_datatype_check(&Datatypes::Integers(1), &Datatypes::Integers(0));
    // println!("{:?}", resul);

    let col1 = vec![
        Datatypes::Integers(1),
        Datatypes::Strings("antiloger".to_string()),
        Datatypes::Booleans(true),
    ];
    resgroup.insert_row(col1).unwrap();

    let col2 = vec![
        Datatypes::Integers(2),
        Datatypes::Strings("saman".to_string()),
        Datatypes::Booleans(true),
    ];

    resgroup.insert_row(col2).unwrap();

    let searchitem = resgroup
        .search_row("rescol2", Datatypes::Strings("antiloger".to_string()))
        .unwrap();

    println!("{:?}", searchitem);

    let searchitem2 = resgroup
        .search_row("rescol2", Datatypes::Strings("saman".to_string()))
        .unwrap();

    println!("{:?}", searchitem2);
}

