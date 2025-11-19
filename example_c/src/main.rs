include!(concat!(env!("OUT_DIR"), "/simple.rs"));

fn main() {
    println!("Hello, world! {}", addthem(3, 7));
}
