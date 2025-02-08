fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // Accessing an element using index
    println!("Second element: {}", second_element);
    let first_element = vec.get(0).unwrap(); // Accessing an element using get()
    println!("First element: {}", first_element);
    let third_element = vec.get(2).unwrap(); //Error: index out of bounds
    println!("Third element: {}", third_element);
}