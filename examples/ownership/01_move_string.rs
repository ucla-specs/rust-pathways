fn take_ownership(text: String) {
    println!("Inside function: {text}");
}

fn main() {
    let message = String::from("hello");

    take_ownership(message);

    // Uncomment this line after running the valid version:
    // println!("After function: {message}");
}