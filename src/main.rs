use RDB::{Column, ColumnGroup, DBtypes, Database, Datatypes, Table};


fn main(){
    let mut db = Database::new("test".to_string());
    let restype = DBtypes::new("res".to_string(), vec![(Datatypes::Integers(0), Datatypes::Strings("h".to_string()))]);
    db.addtype(restype).unwrap();
    let mut dbtype = db.get_dbtype("res".to_string()).unwrap();
    let restable = Table::new("restable".to_string(), "restype".to_string());
    dbtype.addtable(restable).unwrap();
    let usertime = ColumnGroup::new("usertime".to_string());
    let mut restable = dbtype.get_table("restable".to_string()).unwrap();
    restable.add_columngroup(usertime).unwrap();
    let time = Column::new("time".to_string(), Datatypes::Integers(0));
    let user = Column::new("user".to_string(), Datatypes::Strings("h".to_string()));
    let mut usertime = restable.get_columngroup("usertime".to_string()).unwrap();
    usertime.add_column(time).unwrap();
    usertime.add_column(user).unwrap();
    
}
