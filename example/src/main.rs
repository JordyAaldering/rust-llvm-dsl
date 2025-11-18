include!(concat!(env!("OUT_DIR"), "/simple.rs"));

fn main() {
    println!("Hello, world! {}", unsafe { addthem(4, 2) });
}
