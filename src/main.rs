mod temp; // Importing the temp module
    
use temp::temperature_convertor;
fn main() {
    temperature_convertor();
    println!("Hello, world!");

    let a: i32 = 5;
    let b: i32 = 10;

    println!("The sum of {} and {} is {}", a, b, a + b);
    println!("a={}, b={}", a, b);

    let mut c:i8 = 126;
    c = c + 1;
    //TODO: Check for overflow

    println!("The value of c is {}", c);

    let mut d:i8 = 1;
    let f:i16 = 200;
    d = f as i8 + d; 
    println!("d={}, f={}", d, f);


    //Float
    // let flNum: f32 = 3.14;
    // let flNum2: f64 = 2.718281828459045;
    // println!("{}", flNum as f64/ flNum2);

    //Boolean
    let is_active: bool = true;
    if is_active {
        println!("The system is active.");
    } else {
        println!("The system is inactive.");
    }

    //CompoundType
    let my_value: (i16, char) = (42, 'A');
    println!("The value is: {}, {}", my_value.0, my_value.1);

    //Array
    let my_arr: [i32; 4] = [1, 2, 3, 4];
    println!("{}", my_arr[2]);
}
