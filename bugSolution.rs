fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1];
    println!("Second element: {}", second_element);
    let first_element = vec.get(0).unwrap();
    println!("First element: {}", first_element);
    match vec.get(2) {
        Some(x) => println!("Third element: {}", x),
        None => println!("Index out of bounds")
    }
}    