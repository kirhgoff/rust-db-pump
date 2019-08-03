extern crate rust_db_pump;
extern crate dotenv;
extern crate rand;

use dotenv::dotenv;
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::distributions::Alphanumeric;

use rust_db_pump::*;

fn main() {
    dotenv().ok();

    let connection = create_connection(SOURCE_DATABASE_URL);
    let table_descriptor = TableDescriptor {
        name: "source_table".to_string(),
        key_field_names: vec!["id".to_string()],
        field_names: vec!["some_string".to_string(), "some_value".to_string()]
    };

    loop {
        let mut rng = rand::thread_rng();

        let some_value: f32 = rng.gen::<f32>();
        let some_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(rng.gen_range(0, 100))
            .collect();

        println!("Writing to table ({:}, {:})", some_string, some_value);
        connection.execute(&table_descriptor.insert_query(), &[&some_string, &some_value]).unwrap();

        thread::sleep(Duration::from_millis(5000));
    }
}
