unsafe extern "C" {
    fn addthem(x: i32, y: i32) -> i32;
}

fn main() {
    println!("Hello, world! {}", unsafe { addthem(4, 2) });
}
