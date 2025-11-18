include!(concat!(env!("OUT_DIR"), "/simple.rs"));

fn main() {
    println!("Hello, world! {}", unsafe { addthem(3, 7) });
}
