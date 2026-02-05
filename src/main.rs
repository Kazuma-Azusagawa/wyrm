//pub mod tokenize;

fn main() {
    let arg = std::env::args().nth(1).expect("No file path given\n");
    let path = std::path::PathBuf::from(arg);

    println!("Hello, {:?}!", path);
}
