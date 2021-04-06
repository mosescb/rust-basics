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
 * Basic data types
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
 * Compound data types
 *
 *   -- Array : All elements should be of same data type
 *   -- Tuple : Elements can be of different data types
 *
 */
fn compound_datatypes() {
    // Integer Array
    let mut arr_u32_integers :[u32; 5];  // Array of 5 integers of type u32
    arr_u32_integers = [0;5];  // Initialize  5 integers with 0
    arr_u32_integers[4] = 7;
    let arr_len :usize = arr_u32_integers.len();
    println!("Length of array, arr_u32_integers = {}", arr_len);
    println!("arr_u32_integers[0] = {}", arr_u32_integers[0]);
    println!("arr_u32_integers[4] = {}\n", arr_u32_integers[arr_len - 1]);

    // 2d Array
    let mut arr_2d_u8_integers :[[u8; 10]; 5];  // 5 araays of 10 integers each
    arr_2d_u8_integers = [[0; 10]; 5];  // Initialize all positions with 0
    arr_2d_u8_integers[1][9] = 12;
    let arr_2d_len_ext = arr_2d_u8_integers.len();
    let arr_2d_len_int = arr_2d_u8_integers[0].len();
    println!("Length of external 2d array, arr_2d_u8_integers    = {}", arr_2d_len_ext);
    println!("Length of internal 2d array, arr_2d_u8_integers[0] = {}", arr_2d_len_int);
    println!("arr_2d_u8_integers[0][0] = {}", arr_2d_u8_integers[0][0]);
    println!("arr_2d_u8_integers[1][9] = {}\n", arr_2d_u8_integers[1][9]);

    // Tuple
    let mut tuple_data = ('M', 8053, '\u{221E}', 9.8);
    println!("tuple_data.0 = {}\ntuple_data.1 = {}", tuple_data.0, tuple_data.1);
    println!("tuple_data.2 = {}\ntuple_data.3 = {}", tuple_data.2, tuple_data.3);
    tuple_data.1 += 947;
    tuple_data.0 = 'C';
    println!("Modified tuple_data.0 = {}\nModified tuple_data.1 = {}\n", tuple_data.0, tuple_data.1);

    let (data0, data1, data2, data3) = tuple_data;
    println!("Split tuple into individual variables");
    println!("data0 = {}\ndata1 = {}", data0, data1);
    println!("data2 = {}\ndata3 = {}\n", data2, data3);
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
 * Bitwise operations
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

    // Basic data types
    basic_datatypes();

    // Compound data types
    compound_datatypes();

    // Casting of data types & println!() formatting data
    cast_and_format();

    // Bitwise data representation
    bitwise_ops();

    // Calculate average of 4 numbers of different data types
    calc_average();

    // Factorial by using iteration
    let u32_num :u32 = 10;
    let u64_factorial :u64 = factorial_iter(u32_num);
    println!("Factorial of {} = {}\n", u32_num, u64_factorial);

    // Addition of 3 bits
    println!("{:0b}+{:0b}+{:0b} = {:?}", 0,0,1, add_3_bits_bool(false, false, true));
    println!("{:0b}+{:0b}+{:0b} = {:?}\n", 1,0,1, add_3_bits_u8(1, 0, 1));

    // Temperature conversion
    let temp_in_celcius :f32 = 10.0;
    let temp_in_fahrenheit = celcius_to_fahrenheit(temp_in_celcius);
    println!("Temp in C: {}\tTemp in F: {}\n", temp_in_celcius, temp_in_fahrenheit);
}


/*
 * Factorial by using iteration
 *
 */
fn factorial_iter(f_u32_num :u32) -> u64 {
    if f_u32_num <= 0
    {
        return 0;
    }

    // f_u32_num >= 1
    let mut u64_res :u64 = f_u32_num as u64;

    for i in 1..f_u32_num as u64
    {
        u64_res *= i;
    }

    return u64_res;
}

/*
 * Addition of 3 bits using bool
 *
 * return: (carry,sum) as boolean values
 *
 */
fn add_3_bits_bool(f_bit1 :bool, f_bit2 :bool, f_bit3 :bool) -> (bool, bool) {
    let u8_sum :u8 = f_bit1 as u8 + f_bit2 as u8 + f_bit3 as u8;
    let mut carry_bit :bool = false;
    let mut sum_bit :bool = false;

    if u8_sum >= 2
    {
        carry_bit = true;
    }

    if 1 == (u8_sum % 2)
    {
        sum_bit = true;
    }

    return (carry_bit,sum_bit);
}

/*
 * Addition of 3 bits using u8
 *
 * return: (carry,sum) as u8 values
 *
 */
fn add_3_bits_u8(f_bit1 :u8, f_bit2 :u8, f_bit3 :u8) -> (u8, u8) {
    let u8_sum :u8 = f_bit1 + f_bit2 + f_bit3;

    if u8_sum > 3
    {
        panic!("Invalid Parameters");
    }

    let mut carry_bit :u8 = 0;
    let mut sum_bit :u8 = 0;

    if u8_sum >= 2
    {
        carry_bit = 1;
    }

    if 1 == (u8_sum % 2)
    {
        sum_bit = 1;
    }

    (carry_bit,sum_bit)  // Can be retured like this if it's a last statement
}

/*
 * Temperature conversion from celcius to fahrenheit
 *
 * return: temperature in fahrenheit
 *
 */
fn celcius_to_fahrenheit(f_f32_temp_in_celcius :f32) -> f32 {
    return (1.8 * f_f32_temp_in_celcius) + 32.0;
}
