fn take_ownership(text: String) {
    println!("Inside function: {text}");
}

fn main() {
    let message = String::from("hello");

    take_ownership(message);

    // Uncomment later to trigger a compiler error:
    // println!("After function: {message}");
}