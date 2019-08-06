extern crate rust_db_pump;
extern crate dotenv;
extern crate rand;

use dotenv::dotenv;
use std::thread;
use std::time::Duration;

use rust_db_pump::*;
//use postgres::types::{ToSql, FromSql};
//use postgres::rows::Row;

fn main() {
    dotenv().ok();

    let source_connection = create_connection(SOURCE_DATABASE_URL);
//    let target_connection = create_connection(TARGET_DATABASE_URL);

    let table_descriptor = TableDescriptor {
        name: "source_table".to_string(),
        key_field_names: vec!["id".to_string()],
        field_names: vec!["some_string".to_string(), "some_value".to_string()]
    };

    loop {
        let source_transaction = source_connection.transaction().unwrap();

        let rows = source_transaction.query(table_descriptor.select_query().as_str(), &[]).unwrap();

        //let target_transaction = target_connection.transaction().unwrap();

        for _row in &rows {
//            let values = (0..table_descriptor.all_fields_count())
//                .map(|i| Box::new(row.get(i)))
//                .collect::<Vec<Box<dyn FromSql>>>();
//            let values = []

//            target_connection.execute(table_descriptor.insert_query().as_str(), &values);
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
