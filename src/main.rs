use std::env;
use std::option::Option;
use std::fs;
use std::fmt;

struct SearchResults {
    occurrences: Vec<String>,
    search_term: String,
}

impl SearchResults {
    fn get_num_occurrences(self: &Self) -> usize {
        self.occurrences.len()
    }
}

impl fmt::Display for SearchResults {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I have located {} occurrences of {}. They appear in the following lines: {:?}", self.get_num_occurrences(), self.search_term, self.occurrences)
    }
}

fn main() {
    let cl_args: Vec<String> = env::args().collect();

    let file_name = get_arg(cl_args.get(1));
    let search_term = get_arg(cl_args.get(2));

    if file_name == "" {
        panic!("No filename provided (1st argument).")
    }

    if search_term == "" {
        panic!("No search term provided (2nd argument).")
    }

    let file_contents = load_file_contents(file_name);
    let search_results: SearchResults = find_occurrences(file_contents, search_term);

    println!("{}", search_results);
}

fn get_arg(arg: Option<&String>) -> String {
    match arg {
        Some(v) => v.to_string(),
        None => "".to_string()
    }
}

fn load_file_contents(filename: String) -> String {
    fs::read_to_string(filename).expect("Couldn't read the file provided. Does it exist?")
}

fn find_occurrences(file_contents: String, search_term: String) -> SearchResults {
    let mut results = SearchResults {
        occurrences: vec![],
        search_term
    };
                 
    for line in file_contents.split("\n") {
        for word in line.split(" ") {
            if word == results.search_term {
                results.occurrences.push(line.trim().to_string())
            }
        }
    }

    results
}
