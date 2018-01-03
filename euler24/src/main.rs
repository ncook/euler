fn main() {
    let pd: u64 = 0;
    let i: u64 = 1;
    match pd - i {
        n if n > 0 => println!("Over"),
        n if n < 0 => println!("under"),
        n => println!("Equal"),
    }
    println!("Hello, world!");
}
