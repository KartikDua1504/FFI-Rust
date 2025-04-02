unsafe extern "C" {
    unsafe fn add(a: i32, b: i32) -> i32;
}
fn main() {
    unsafe {
        let result = add(40, 5);
        println!("{result}");
    }
}
