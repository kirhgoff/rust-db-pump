extern crate rust_db_pump;

use postgres::{Connection, TlsMode};

fn main() {
    let source_url = "postgres://postgres:postgres@localhost:5433/bams1";
    let source_connection = Connection::connect(source_url, TlsMode::None).unwrap();

    let target_url = "postgres://postgres:postgres@localhost:5434/bams2";
    let target_connection = Connection::connect(target_url, TlsMode::None).unwrap();

    let rows = source_connection.query("SELECT * FROM source_table", &[]).unwrap();
    for row in &rows {
        let id: i32 = row.get(0);
        let some_string: String = row.get(1);
        let some_value: f32 = row.get(2);

        println!("values: {:} {:} {:}", id, some_string, some_value);
        let _ = target_connection.execute(
            "INSERT INTO target_table(id, some_string, some_value) VALUES($1, $2, $3)",
            &[&id, &some_string, &some_value]
        );
    }
}