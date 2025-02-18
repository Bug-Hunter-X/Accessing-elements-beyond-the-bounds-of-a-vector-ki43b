fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    println!("First element: {:?}", first);
    let second = vec.get(1);
    println!("Second element: {:?}", second);
    // This will panic!
    let third = vec.get(2);
    println!("Third element: {:?}", third);
}