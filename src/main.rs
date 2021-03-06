/*
 * Basics of rust programming language
 * Author: Moses Christopher Bollavarapu <moseschristopherb@gmail.com>
 *
 * BUILD  : cargo build
 * RUN    : cargo run
 *
 */

use std::io::prelude::*;
use std::env;
use std::fs;

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

    // Calculate minimum, maximum, average from an array of integers
    calc_min_max_avg();

    // Strings and slices
    let mut str_message :String = String::from("    Hello this is a new string");
    str_message.push_str(" which got appended by another string     ");
    println!("{}\n", str_message);

    // Structs by-default stored on stack
    struct_basics();

    // Tuple struct
    tuple_struct_basics();

    // Rectangle struct
    rectangle_struct();

    // Printing arguments passed
    for (index, arg) in env::args().enumerate() {
        println!("arg {} is {}", index, arg);
    }

    // File operations
    file_ops();
}

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

/*
 * Calculate minimum, maximum, average from an array of integers
 *
 */
fn calc_min_max_avg() {
    let i32_arr_integers = [1,2,3,0,32,94,-34,-19,290,-29,-489,899,8,0,1];
    let mut i32_min :i32 = i32_arr_integers[0];
    let mut i32_max :i32 = i32_arr_integers[0];
    let mut f32_avg :f32;
    let mut f32_sum :f32 = 0.0;
    println!("Method1");
    for i in 0..i32_arr_integers.len() {
        if i32_arr_integers[i] < i32_min {
            i32_min = i32_arr_integers[i];
        }
        if i32_arr_integers[i] > i32_max {
            i32_max = i32_arr_integers[i];
        }

        f32_sum += i32_arr_integers[i] as f32;
    }
    f32_avg = f32_sum / i32_arr_integers.len() as f32;
    println!("MIN= {}\t MAX= {}\t AVG= {}\n", i32_min, i32_max, f32_avg);

    println!("Method2");
    f32_avg = 0.0;
    for &j in i32_arr_integers.iter() {
        if j < i32_min {
            i32_min = j;
        }
        if j > i32_max {
            i32_max = j;
        }

        f32_avg += j as f32;
    }

    f32_avg = f32_avg / i32_arr_integers.len() as f32;
    println!("MIN= {}\t MAX= {}\t AVG= {}\n", i32_min, i32_max, f32_avg);
}

/*
* Struct
*
*/
#[derive(Debug)] // Needed to print data using {:?}
#[derive(Clone)] // Needed to clone strings
struct Person {
    name: String,
    gender: String,
    age: u8,
    height: f32,
    weight: f32
}

// Struct methods & associated functions
impl Person {
    // methods contain &self
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u8 {
        self.age
    }
    fn set_age(&mut self, new_age :u8) {
        self.age = new_age;
    }

    // associated functions do not have &self argument
    fn new(entered_name: &str) -> Person {
        Person {
            name: String::from(entered_name),
            gender: String::from("Undefined"),
            age: 0,
            height: 0.0,
            weight: 0.0
        }
    }
}

/*
 * Struct basics
 *
 */
fn struct_basics() {
    let Person1 = Person{
        name: String::from("Chris"),
        gender: String::from("Male"),
        age: 95,
        height: 180.01,
        weight: 78.65
    };

    let Person2 = Person{
        name: String::from("Moses"),
        gender: String::from("Male"),
        ..Person1  // Copy rest of the values from Person1 to Person2
    };

    let mut Person3 = Person{
        name: String::from("Steve"),
        ..Person1.clone()  // Copy rest of the values from Person1 to Person3 (even string as clone)
    };

    let mut Person4 = Person::new("Lisa");

    Person3.age = 36;
    Person4.gender = String::from("Female");

    println!("Data from struct: \nname: {}\tage: {}\tgender: {}\n", Person1.name, Person1.age, Person1.gender);

    println!("Print struct data using Debug print\n");
    println!("{:?}", Person1);
    println!("{:?}", Person2);
    println!("{:?}", Person3);
    println!("{:?}\n", Person4);

    // let person3_name = Person3.get_name(); // --> Produces error as it is called as immutable first
    // Update person3 age to 45
    Person3.set_age(45); // Mutable call
    let person3_name = Person3.get_name(); // --> No error here because first call is mutable
    println!("Person3 details from methods: name = {}\t age = {}\n", person3_name, Person3.get_age());
}

// Tuple structs
struct Color(u8, u8, u8);   // Red Green Blue
struct Point3D(u8, u8, u8); // X,Y,Z Coordinates
struct Point2D(u8, u8);     // X,Y Coordinates

/*
 * Sample function with tuple struct as an argument
 *
 */
fn get_point3d_x(px :&Point3D) -> u8 {
    px.0
}

/*
 * Tuple struct basics
 *
 */
fn tuple_struct_basics() {
    let col_red = Color(255,0,0);
    let point_3d = Point3D(3,6,1);
    let point_2d = Point2D(12, 5);

    println!("Red : {}", col_red.0);
    println!("X in 3d point : {}", get_point3d_x(&point_3d));
    println!("Z in 3d point : {}", point_3d.2);
    println!("Y in 2d point : {}\n", point_2d.1);
}

/*
 * Rectangle struct
 *
 */
fn rectangle_struct() {
    struct Rectangle {
        width :f64,
        height :f64
    }

    impl Rectangle {
        fn get_area(&self) -> f64 {
            self.width * self.height
        }
        fn scale(&mut self, mult :f64) {
            self.width *= mult;
            self.height *= mult;
        }

        fn new(input_width :f64, input_height :f64) -> Rectangle {
            Rectangle {
                width: input_width,
                height: input_height
            }
        }
    }

    let mut rect1 = Rectangle::new(10.0, 2.5);
    println!("Width: {}\t Height: {}", rect1.width, rect1.height);
    assert_eq!(rect1.get_area(), 25.0);
    rect1.scale(2.0);
    println!("Width: {}\t Height: {}\n", rect1.width, rect1.height);
    assert_eq!(rect1.get_area(), 100.0);
    println!("Rectangle struct works as expected");
}

/*
 * File operations
 *
 */
fn file_ops() {
    // Write data to file
    let name_of_text_file = String::from("text-data/names.txt");
    let mut names_data = String::new();
    names_data.push_str("Moses\n");
    names_data.push_str("Christopher\n");
    names_data.push_str("CarrotPi");
    fs::write(&name_of_text_file, names_data).unwrap();  // needs std::fs

    // Append data to file
    let mut file_with_append = fs::OpenOptions::new().append(true).open(&name_of_text_file).unwrap();
    file_with_append.write(b"\nSteve").unwrap(); // needs std::io::prelude

    // Read complete file, line by line
    let file_content = fs::read_to_string(&name_of_text_file).unwrap();
    for line in file_content.lines()
    {
        println!("line-by-line: {}", line);
    }

    // Commandline check with files  --> needs std::env
    if env::args().enumerate().len() <= 2 {
        println!("Need two arguments to process");
        println!("\nUsage: cargo run path-to-file search-text");
        println!("Example: cargo run {} Moses\n",name_of_text_file);
        return;
    }
    // Usage: cargo run text-data/names.txt Moses
    let read_path = env::args().nth(1).unwrap();
    let match_string = env::args().nth(2).unwrap();
    // println!("Read path is {}", read_path);

    let file_content = fs::read_to_string(&read_path).unwrap();
    for line in file_content.lines()
    {
        if line.contains(&match_string) {
            println!("Found a match");
            break;
        }
        // println!("No match found");
    }
}
