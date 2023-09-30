// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

struct  Person {
    first_name: String,
    last_name: String
}

struct Color(u8, u8, u8);

impl Person {
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    fn to_tuples(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run (){
    // let mut c = Color{red: 255, green: 0, blue: 0};

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let c = Color(200, 0, 0);
    // println!("{} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Samuel", "Anthony");
    println!("Person {}", p.full_name());
    p.set_last_name("Thelle");
    println!("Person {}", p.full_name());
    println!("{:?}", p.to_tuples())
}