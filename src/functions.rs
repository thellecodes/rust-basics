pub fn run(){
    greeting("Hello", "Samuel");
    
    let get_sum = add(1, 2);
    println!("Sum: {}", get_sum);


    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Add: {}", add_nums(1, 2));
}

fn greeting(greet: &str, name: &str){
    println!("{} {} nice to meet you.",greet, name)
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}