pub fn run(){
    let  name ="Samuel";
    let mut age = 29;
    age = 30;

    println!("My name is {} and I am {} ", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Samuel", 29);
    println!("{} is {}", my_name, my_age);
}