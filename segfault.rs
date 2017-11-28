fn main() {
    let x = { 100283 as *const i32 };
    println!("{}", unsafe { *x });
}
