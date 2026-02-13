use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // turn iter to collection vector
    
    dbg!(args.join("-"));
}
