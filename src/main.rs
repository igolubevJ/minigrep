use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Поиск {}", query);
    println!("В файле {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Что-то пошло не так при чтении файла.");

    println!("С текстом:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

