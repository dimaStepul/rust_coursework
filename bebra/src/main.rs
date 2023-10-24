use std::env;
use std::fs;

fn parse_config(args: &[String]) -> &str {
    let directory_name = &args[1];
    directory_name
}

fn list_elements_in_dir(dir: &str) {
    let paths = fs::read_dir(dir);
    if paths.is_err() {
        let err = paths.err().unwrap();
        eprintln!("error {} occurred", err);
        return;
    }
    for path in paths.unwrap() {
        match path {
            Ok(entity) => {
                if entity.path().is_dir() {
                    println!("Dir: {}", entity.path().display());
                }
                if entity.path().is_file() {
                    println!(
                        "File: {}",
                        entity.path().file_name().unwrap().to_str().unwrap()
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory_name = parse_config(&args);
    println!("Searching in dir {}", directory_name);
    list_elements_in_dir(directory_name);
}
