fn read_text(text: &String) {
    println!("Inside function: {text}");
}

fn main() {
    let message = String::from("hello");

    println!("Before function: {message}");

    read_text(&message);

    println!("After function: {message}");
}