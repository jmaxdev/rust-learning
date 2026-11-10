
// constant: defines a value that will not change, different from let, and requires a type definition 
// global variable
const _TEMPORAL_MAX_POINTS: u32 = 20;

// Temporary variables starting with "_" or containing "temporal" silence compiler warnings

fn main() {
    // Local Variables
    let _temporal_world = "World!";
    // let world = world.len(); // gets the string length 

    // Immutable variable 
    let _temporal_points = 10; // points = 10 (x)
     
    // Mutable variable
    let mut _temporal_mutable_points = 2; // mutable_points = 5 (O)

    // shadowing: allows declaring a new variable with the exact same name as a previous variable
    let _temporal_x = 5;
    let _temporal_x = _temporal_x + 1; // = 6 

    // types: i32 = int, f64 = float, bool = boolean (true/false), char = 'ñ' (Unicode character, uses 4 bytes). Rust infers the type when compiling the code (sometimes).
    // An integer like u8 is 8 bits (from 0 to 255). If exceeded, Rust will not compile the code.
    let _temporal_person: (&str, u32, f64) = ("Carlos", 20, 1.60); // tuple example: (name, age, height)
    let _temporal_todo_list: [&str; 3] = ["buy bread", "clean car", "pay taxes"];
    let _temporal_explicit: u64 = 1_300;
    let _temporal_ferris = '🦀';

    // 'i' (signed) or 'u' (unsigned) indicates whether it can be negative or positive, followed by the bit size. E.g., i8 = -128 to 127 / u8 = 0 to 255

    println!("i8:  {} to {}", i8::MIN, i8::MAX);
    println!("u8:  {} to {}", u8::MIN, u8::MAX);
    println!("i32: {} to {}", i32::MIN, i32::MAX);
    println!("u64: {} to {}", u64::MIN, u64::MAX);

    person("Carlos", 20, 1.60); // "Hello Carlos, your age is 20 and your height is 1.60."

    let final_price = discount(1.99, 10); // $1.99 - 10% = $ 1.89

    println!("The final price is: {}", final_price);

    let (d, f) = div(10, 4);
    println!("{d}, {f}");

    // loop

    let mut count = 0;

    let count_result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    
    println!("{count_result}"); 


    // for 
    let mut apples_in_box = 0;
    for apple in [1, 2, 5, 10, 30] {
        if apple >= 5 {
            apples_in_box += 1;
        }
    }
    
    println!("There are {apples_in_box} apples in the box.");   

    // while

    let mut children_in_room = 20;
    while children_in_room > 15 {
        children_in_room -= 1;
    }
    println!("There are {children_in_room} children in the classroom.");
    
}

fn person(name:&str, age:u32, height: f64) {
    println!("Hello {name}, your age is {age} and your height is {height}.")
}

fn discount(price: f64, discount: i32) -> f64 {
    price - (discount as f64 / 100.0)
}

fn div(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}