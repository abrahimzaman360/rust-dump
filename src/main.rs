use std::{
    collections::HashMap,
    env::args,
    fs::{File, read_dir},
    io::Read,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

// Mini Data Processor:
// Goals:
// Input -> Processing -> Storage<Opt> -> Output
// What is our Input: <Files | Stdin aka Terminal Input etc>
// How to process: Load files -> Split by lines -> Count words -> Store Counts in Hashmap -> Return Results.
//
// There is currently lock contention problem causing each thread to wait until lock is freed.
// That will be resolved in next iteration IA;
//

// Self Written
fn process<'a>(file: PathBuf, locked_store: Arc<Mutex<HashMap<String, usize>>>) {
    // AI Taught:
    // We need String in Hasmap<String, usize>, because content is local string, will drop at end and cause invalid word refs got in Hashmap.
    let mut content = String::new();
    match File::open(file) {
        Ok(mut f) => {
            f.read_to_string(&mut content).unwrap();
        }
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    // Process the content
    if content.is_empty() {
        eprintln!("Content is empty");
    }

    let mut unlocked_store = locked_store.lock().unwrap();

    for line in content.split("\n") {
        for word in line.split_whitespace() {
            unlocked_store
                .entry(word.into())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
}

fn main() {
    // Task 1 (read file)
    let store: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    // Self Learnt:
    let dir = args().nth(1).unwrap_or("./".to_string());
    let files = match read_dir(dir) {
        Ok(found) => found,
        Err(e) => {
            eprintln!("Error while scanning the directory!\n{}", e);
            return;
        }
    };

    // AI Taught:
    let mut handles: Vec<JoinHandle<_>> = Vec::new();

    // Self Learnt:
    for each_file in files {
        let cloned_store = Arc::clone(&store);
        let file_path = match each_file {
            Ok(f) => f.path(),
            Err(e) => {
                eprintln!("Unable to find path!\n{}", e);
                return;
            }
        };

        let handle = thread::spawn(move || {
            process(file_path, cloned_store);
        });

        handles.push(handle);
    }

    // AI Taught:
    for handle in handles {
        handle.join().unwrap();
    }

    // AI Taught:
    let unlocked = store.lock().unwrap();
    println!("{:#?}", unlocked);
}
