use fst::Automaton;
use rustdoc_seeker::RustDoc;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples/search-index.js");
    let data = fs::read_to_string(path).unwrap();
    let rustdoc: RustDoc = data.parse().unwrap();
    let seeker = rustdoc.build();

    let regex = fst_regex::Regex::new(".*io.*").unwrap();
    for i in seeker.search(&regex) {
        println!("Regex {}", i);
    }

    let edist = fst_levenshtein::Levenshtein::new("spawn", 1).unwrap();
    for i in seeker.search(&edist) {
        println!("Edit Distance {}", i);
    }

    let subsq = fst::automaton::Subsequence::new("try_join");
    for i in seeker.search(&subsq) {
        println!("Subsequence {}", i);
    }

    let union = subsq.union(regex);
    for i in seeker.search(&union) {
        println!("Union {}", i);
    }

    let starts = edist.starts_with();
    for i in seeker.search(&starts) {
        println!("Starts_with {}", i);
    }
}
