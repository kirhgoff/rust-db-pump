extern crate postgres;
use std::env;

use postgres::{Connection, TlsMode};

pub const SOURCE_DATABASE_URL: &str = "SOURCE_DATABASE_URL";
pub const TARGET_DATABASE_URL: &str = "TARGET_DATABASE_URL";

pub fn create_connection(env_connection_name: &str) -> Connection {
    let database_url = env::var(env_connection_name)
        .expect(&format!("{:} must be set", env_connection_name));

    Connection::connect(database_url, TlsMode::None)
        .expect(&format!("Cannot connect to {:}", env_connection_name))
}

pub struct TableDescriptor {
    pub name: String,
    pub key_field_names: Vec<String>,
    pub field_names: Vec<String>
}

impl TableDescriptor {
    pub fn insert_query(&self) -> String {
        let fields = self.field_names.join(", ");

        let string_indices: Vec<String> = (1..(self.field_names.len() + 1))
            .map(|n| format!("${:}", n.to_string()))
            .collect();

        let indices = string_indices.join(", ");

        format!("INSERT INTO {:} ({:}) VALUES({:})", self.name, fields, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_descriptor_insert_query() {
        let desc = TableDescriptor {
            name: "bebe".to_string(),
            key_field_names: vec!["keyFieldC".to_string()],
            field_names: vec!["fieldA".to_string(), "fieldB".to_string()]
        };

        assert_eq!(desc.insert_query(), "INSERT INTO bebe (fieldA, fieldB) VALUES($1, $2)");
    }
}

