/*
 * Basics of rust programming language
 * Author: Moses Christopher Bollavarapu <moseschristopherb@gmail.com>
 *
 * BUILD  : cargo build
 * RUN    : cargo run
 *
 */


 /*
 * Mutable vs Immutable
 *
 */
fn mutable_vs_immutable() {
    let x = 10; // Immutable
    println!("x = {}", x);
    let mut y = x;
    println!("y = {}", y);
    y = 20;
    println!("Changed y now to 20");
    println!("y = {}\n", y);
}

/*
 * Data types
 *
 */
fn basic_datatypes() {
    // int            --> i8 < i16 < i32 < i64 < i128
    // unsigned int   --> u8 < u16 < u32 < u64 < u128
    // float          -->            f32 < f64
    // boolean        --> true(1) / false(0)
    // char (4bytes)  -->
    let i32_val :i32 = 1000;
    let u8_val :u8 = 255;
    let f32_val :f32 = 123.123456789123456789;
    let f64_val :f64 = 123.123456789123456789;
    let char_cap_a = 'A';
    let char_a = 'a';
    let char_copyright = '\u{00A9}';
    println!("i32_val: {}\t\t u8_val: {}", i32_val, u8_val);
    println!("f32_val: {}\t f64_val: {}", f32_val, f64_val);
    println!("Boolean {:5} in int is {}", true, true as i32);
    println!("Boolean {:5} in int is {}", false, false as i32);
    println!("Chars: {} {} {}\n", char_cap_a, char_a, char_copyright);
}

/*
 * Casting of data types & println!() formatting data
 *
 */
fn cast_and_format() {
    // cast data
    let i32_num = 22;   // Default int is i32
    let f64_den = 7.0;  // Default float is f64
    // let f64_div_result = i32_num / f64_den; // Can't mix data types - Produces error
    let f64_div_result = i32_num as f64 / f64_den;
    println!("{}/{} = {}\n", i32_num, f64_den, f64_div_result);

    // format data
    println!("Division result: {:10.5}", f64_div_result); // total 10chars to represent number and 5spaces for decimal value after .
    println!("Division result: {:09.4}", f64_div_result);
    println!("Division of {0} by {1} is: {0}/{1} = {2}\n", i32_num, f64_den, f64_div_result); // Using position values
}

/*
 * bitwise operations
 *
 */
fn bitwise_ops() {
    let u8_bit_data = 0b10100101u8;
    println!("Decimal Rep     : {}", u8_bit_data);
    println!("Binary Rep      : {:08b}", u8_bit_data);
    println!("Hexadecimal Rep : {:02x}", u8_bit_data);
    println!("Octal Rep       : {:03o}\n", u8_bit_data);

    // bitwise operations on u8_bit_data
    // bitwise not
    println!("!{0:08b} =  {1:08b}", u8_bit_data, !u8_bit_data);
    // bitwise or
    println!("{:08b} | 11110000   =  {:08b}", u8_bit_data, u8_bit_data | 0b1111_0000u8);
    // bitwise and
    println!("{:08b} & 11110000   =  {:08b}", u8_bit_data, u8_bit_data & 0b1111_0000u8);
    // bitwise xor
    println!("{:08b} ^ 11110000   =  {:08b}", u8_bit_data, u8_bit_data ^ 0b1111_0000u8);
    // left shift by 2
    println!("{:08b} << 2    =  {:08b}", u8_bit_data, u8_bit_data << 2);
    // right shift by 3
    println!("{:08b} >> 3    =  {:08b}\n", u8_bit_data, u8_bit_data >> 3);
    // short-circuiting OR  ||  --> only left side will be executed if it is true
    println!("Short Or: {}", true || panic!());
    // short-circuiting AND &&  --> only left side will be executed if it is false
    println!("Short And: {}\n", false && panic!());
}

/*
 * Calculate average of 4 numbers of different data types
 *
 */
fn calc_average() {
    let u32_val1 :u32 = 100;
    let f64_val2 :f64 = 20.5;
    let f32_val3 :f32 = 49.5;
    let i64_val4 :i64 = 30;

    let f64_sum :f64 = u32_val1 as f64 + f64_val2 + f32_val3 as f64 + i64_val4 as f64;
    let f64_avg :f64 = f64_sum / 4.0;

    assert_eq!(f64_avg, 50.0);
    println!("Test Passed: Averare computed = {}\n", f64_avg);
}

fn main() {
    // Mutable vs Immutable
    mutable_vs_immutable();

    // Data types
    basic_datatypes();

    // Casting of data types & println!() formatting data
    cast_and_format();

    // bitwise data representation
    bitwise_ops();

    // Calculate average of 4 numbers of different data types
    calc_average();
}
