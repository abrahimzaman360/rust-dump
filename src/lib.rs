use dashmap::DashMap;
use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

// Self Written
#[doc = "# Process:
Does it 's job really well, but there is still contention problem,
I see even after using Dashmap (aka Sharded Hashmap Imp),
We might need per-thread level Hashmap + Main thread Merge of all results computed by each child thread.
"]
pub fn process_file(file: PathBuf, store: Arc<DashMap<String, usize>>) {
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

    for line in content.split("\n") {
        for word in line.split_whitespace() {
            store
                .entry(word.to_string())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }
}
