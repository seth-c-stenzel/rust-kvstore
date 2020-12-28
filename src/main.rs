use std::collections::HashMap;
use std::io::Error as ioError;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let db = Database::new();

    let  contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents);
    println!("Key: {} | Value: {} pair has been stored", key, value);
}

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, ioError> {
        
        /* Using the ? operator allows us ot use this pattern
        let contents = match std::fs::read_to_string("kv.db") {
            Result::Ok(contents) => contents,
            Result::Err(e) => return Err(e),
        };
        */
        let contents = std::fs::read_to_string("kv.db")?;

        Ok(Database {
            inner: HashMap::new()
        })
    }
}
