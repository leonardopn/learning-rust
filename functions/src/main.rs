fn main() {
    another_function(five());
    another_function(five_with_return());

    let first = String::from("Ferris");
    let full = add_suffix(first.clone());
    println!("{first}");
    println!("{full}");
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn five_with_return() -> i32 {
    return 5;
}

fn add_suffix(name: String) -> String {
    let mut name = name;
    name.push_str(" Jr.");
    name
}
