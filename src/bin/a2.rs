fn main() {
    let result = add(8, 3);
    display_add(result);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn display_add(a: i32) {
    println!("Hasil: {:?}", a);
}
