fn main() {
    let one: &i8;
    {
        let two: i8 = 2;
        one = &two;
    }
    println!("r: {}", one);
}
