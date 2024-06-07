fn main() {
let decimal:f64 = 9.665671;


// converting to unsigned integer  type

let integer = decimal as u16;

println!("From Decimal {} to Integer {}",  decimal, integer);

let character:char = 'A';
let integer =  character as u8;
println!("From Character {} to Integer {}",  character, integer);
}
