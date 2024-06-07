fn main() {
    println!("Variables");
    let age = 31;
    let name = "Sam";
    println!("Age: {}",age );
    println!("Name: {}", name );

    // Rust varibales are immutable
    // To create a varable mutable, we have to use 'mut' keyword

    let mut x = 1;
    println!("x: {}",x);
    x=2;
    println!("x: {}",x);

    // DataTypes
    println!("Data Types in Rust: " );
    println!("1. Integer" );
    println!("2. Float" );
    println!("3. Boolean" );
    println!("4. Char" );
    
    // Integer types
    println!("Integer types Data include two sub types:" );
    println!("a. Signed" );
    
    let x : i32 = -100;
    let y : i32 = 100;
    println!("x: {}",x);
    println!("y: {}",y);
    
    println!("b. UnSigned" );
    let x : u32 = 50;
    println!("x: {}",x);
    
    //Floating 
    println!("Floating types Data include two sub types:" );
    println!("a. f32" );
    println!("b. f64" );

    let x: f32 = 1.73;
    let y: f64 = 3.114678989;

    println!("x: {}", x);
    println!("y: {}", y);

    // Boolean Data Types
    let flag: bool = true;
    println!("Boolean Value: {}", flag );

    // Char Data Types
    let c: char = '@';
    println!("Character Value: {}", c );

    
}
