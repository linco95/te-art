use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = parse_config(&args);
    println!("Received path: {}", path);

    read_image(path);
}

fn read_image(path: &str) -> &str {
    path
}

fn parse_config(args: &[String]) -> &str {
    let path = &args[1];

    path
}
