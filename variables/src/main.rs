fn mutable() {
    println!("Variables can be mutable");
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);
}

fn constant() {
    println!("Constants need to be fixed");
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    {
        let x = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn type_of_integers_variables() {
    println!("Integers can be of different sizes:");
    const INT_SIZE: [u32; 4] = [8, 16, 32, 128];

    for i in INT_SIZE.iter() {
        let initial_number = u128::pow(2, i - 1);
        let final_number = initial_number - 1;

        let u_notation_final: u128;

        match i {
            8 => u_notation_final = u8::MAX as u128,
            16 => u_notation_final = u16::MAX as u128,
            32 => u_notation_final = u32::MAX as u128,
            128 => u_notation_final = u128::MAX,
            _ => panic!("Invalid integer size"),
        }

        println!("The i{i} accept numbers between: -{initial_number} and {final_number}");
        println!("The u{i} accept numbers between: 0 and {u_notation_final}");
        println!("");
    }
}

fn type_of_floats_variables() {
    println!("Floats can be of different sizes:");

    println!(
        "The f32 accept numbers between: {} and {}",
        f32::MIN,
        f32::MAX
    );
    println!(
        "The f64 accept numbers between: {} and {}",
        f64::MIN,
        f64::MAX
    );

    println!("");
}

fn main() {
    mutable();
    println!("");
    constant();
    println!("");
    shadowing();
    println!("");
    type_of_integers_variables();
    println!("");
    type_of_floats_variables();
}
