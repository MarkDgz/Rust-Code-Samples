fn main() {

/*  
bool	 		Boolean (true / false)
char	 		character
f32, f64 		32-bits, 64-bits floats
i64, i32, i16, i8	signed 16- ... integers
u64, u32, u16, u8	unsigned 16-bits, ... integers
isize			pointer-sized signed integers
usize			pointer-sized unsigned integers
*/

  let _vbool = true;
  let _vchar = "This is a Char Variable";
  // Floats
  let _vf32:f32  = 32.323232;
  let _vf64:f64  = 64.64646464;
  // Signed Integers i64 i32 i16 i8;
  let _vi64:i64 = -64;
  let _vi32:i32 = -32;
  let _vi16:i16 = 16;
  let _vi8:i8   = 8;
  // Signed Integers u64 u32 u16 u8;
  let _vu64:u64 = 64;
  let _vu32:u32 = 32;
  let _vu16:u16 = 16;
  let _vu8:u8   = 8;
  // pointer sized signed integers ;
  let _visize:isize = -100000;
  // pointer sized signed integers ;
  let _vusize:usize = 353535;

// Prints the output
print!("Hello World\n");

// Appends a new line after printing
println!("Appending a new line");

// Prints as an error
eprint!("This is an error\n");

// Prints as an error with new line
eprintln!("This is an error with new line");

// Single Placeholder
println!("{}", 1);

// Multiple Placeholder
println!("{} {}", 1, 3);

// Positional Arguments
println!("{0} is {1} {2}, also {0} is a {3} programming language", "Rust", "cool", "language", "safe");

// Named Arguments
println!("{country} is a diverse nation with unity.", country = "India");

// Placeholder traits :b for binary, :0x is for hex and :o is octal
println!("Let us print 76 is binary which is {:b} , and hex equivalent is {:0x} and octal equivalent is {:o}", 76, 76, 76);

// Debug Trait
println!("Print whatever we want to here using debug trait {:?}", (76, 'A', 90));

// New Format Strings in 1.58
let x = "world";
println!("Hello {x}!");

}
