fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Safe way to access the first element
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty"),
    };

    //Alternative using expect for custom error message
    let first = vec.get(0).expect("Vector is empty");
    println!("First element: {}", first);
} 