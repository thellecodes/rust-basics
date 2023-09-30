pub fn run() {
    let x = 1;
    let y = 2.5;
    let z :i64= 43541231235;

    println!("Max i32 {} ", std::i32::MAX);
    println!("Max i64 {} ", std::i64::MAX);

    let is_active : bool= true;
    let is_greater = 10 > 5;

    let a1 = 'a';

    println!("{:?}", (x, y, z, is_active, is_greater, a1));

}