use std::{
    env::args,
    fs::read_dir,
    sync::Arc,
    thread::{self, JoinHandle},
};

use dashmap::DashMap;
use training::process_file;

#[doc = "# Mini Data Processor:\n
## Goals:
1. Input -> Processing -> Storage<Opt> -> Output.\n
2. What is our Input: <Files | Stdin aka Terminal Input etc>\n
3. How to process: Load files -> Split by lines -> Count words -> Store Counts in Hashmap -> Return Results.\n
4. There is currently lock contention problem causing each thread to wait until lock is freed.\n
5. That will be resolved in next iteration IA;
"]
fn main() {
    // Task 1 (read file)
    let store: Arc<DashMap<String, usize>> = Arc::new(DashMap::new());

    // Self Learnt:
    let dir = args().nth(1).unwrap_or("./".to_string());
    let files = match read_dir(dir) {
        Ok(files) => files,
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
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to find path!\n{}", e);
                return;
            }
        };

        if file_path.path().is_dir() {
            continue;
        }

        let handle = thread::spawn(move || {
            process_file(file_path.path(), cloned_store);
        });

        handles.push(handle);
    }

    // AI Taught:
    for handle in handles {
        handle.join().unwrap();
    }

    // AI Taught:
    println!("{:#?}", store);
}

// Key takeaways:
// This project is not structured well, (for now);
// But we will do it soon;
//
// Also I am writing tests in Main file because project is really small;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        // Look how clean it is (I forgot DoNot Repeat Yourself) -> 204
        let store: Arc<DashMap<String, usize>> = Arc::new(DashMap::new());

        // Self Learnt:
        let dir = args().nth(1).unwrap_or("./data/testing".to_string());
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
                process_file(file_path, cloned_store);
            });

            handles.push(handle);
        }

        // AI Taught:
        for handle in handles {
            handle.join().unwrap();
        }

        let total_words = store.iter().map(|r| *r.value()).sum::<usize>();
        assert_eq!(total_words, 917);
    }
}
