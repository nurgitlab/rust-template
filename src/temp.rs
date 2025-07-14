use std::io;

fn c_to_f (c: f32) -> f32 {
    return c * 9.0 / 5.0 + 32.0;
}

fn f_to_c (f: f32) -> f32 {
    return (f  - 32.0) * 5.0 / 9.0;
}

pub fn temperature_convertor () {
    println!("Temperature convertor");

    let mut my_string: String = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let n_choice = my_string.trim().parse::<f32>().expect( "Please enter a valid number");

    println!("C->F: {}, F->C: {}", c_to_f(n_choice), f_to_c(n_choice));
}