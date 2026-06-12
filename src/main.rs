use std::{collections::HashMap, env::args, fs::File, io::Read};

// Mini Data Processor:
// Goals:
// Input -> Processing -> Storage<Opt> -> Output
// What is our Input: <Files | Stdin aka Terminal Input etc>
// How to process: Load files -> Split by lines -> Count words -> Store Counts in Hashmap -> Return Results.
//
fn main() {
    // Task 1 (read file)
    let mut store: HashMap<&str, usize> = HashMap::new();
    let mut content = String::new();

    let file_arg = args().nth(1).unwrap_or("data.txt".to_string());

    let mut file = match File::open(file_arg) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error while reading file:\n{}", e);
            return;
        }
    };
    file.read_to_string(&mut content).unwrap();

    // Process the content
    if content.is_empty() {
        eprintln!("Content is empty");
    }

    for line in content.split("\n") {
        for word in line.split_whitespace() {
            *store.entry(word).or_insert(0) += 1;
        }
    }

    for (k, v) in &store {
        println!("{}\t{}", k, v);
    }
}
