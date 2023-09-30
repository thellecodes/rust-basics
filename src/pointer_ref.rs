pub fn run(){
    let array1 = [1, 2, 3];
    let array2 = array1;

    let vec1 = [1, 2, 3];
    let vec2 = &vec1;

    println!("Values {:?}", (vec1, vec2));
}