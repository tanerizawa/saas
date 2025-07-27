use std::env;

fn main() {
    println!("Testing basic functionality");
    println!("Current directory: {:?}", env::current_dir().unwrap());
    println!("Project is compiling and running");
}
