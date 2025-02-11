fn main() {
    println!("Hello, world!");
    print!("hello");
    print!("hello");
}

#[test]
fn hello_test() {
    println!("Hello from test");
}

#[test]
fn test_variable_immutable () {
    let name: &str = "Yudistira Satya Dewantara";
    println!("Hello {}", name);
}

#[test]
fn test_variable_mutable () {
    let mut name: &str = "Yudistira Satya Dewantara";
    println!("Hello {}", name);

    name = "joko anwar";

    println!("Hello {}", name);
}

#[test]
fn test_static_typing () {
    let mut name: &str = "Yudistira Satya Dewantara";
    println!("Hello {}", name);

    // name = 25;
    name = "texx";

    println!("Hello {}", name);
}

#[test]
fn test_shadowing () {
    let name: &str = "Yudistira Satya Dewantara";
    println!("Hello {}", name);

    let name = 25;

    println!("Hello {}", name);
}

#[test]
fn comment() {
    // Single line comment
    /*
    Multi line comment
    */
}

#[test]
fn explicit() {
    let age = 20;

    println!("{}", age);
}

#[test]
fn number () {
    let a: i8 = 10;

    println!("{}", a);

    let b: f32 = 10.5;

    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;

    println!("A : {}", a);

    let b:i16 = a as i16;

    println!("B : {}", b);

    let c:i32 = b as i32;

    println!("B : {}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;

    println!("Integer Overflow : {}", e);
}

#[test]
fn numeric_operator() {
    let a= 10;
    let b = 10;
    let c = a*b;
    let d = a/b; 
    println!("{} * {} = {}", a, b, c);
    println!("{} / {} = {}", a, b, d);
}

#[test]
fn augmented_assigment(){
    let mut a = 10;
    println!("A : {}", a);

    a += 10;

    println!("A : {}", a);

    a *= 10;

    println!("A : {}", a);
}

#[test]
fn boolean () {
    let a = true;
    let b: bool = false;

    println!("A : {}", a);
    println!("B : {}", b);

}

#[test]
fn comparison () {
    let a = 30;
    let b = 20;
    let result = a > b;

    println!("{} > {} = {}",a,b,result);
}

#[test]
fn boolean_operators () {
    let absen = 74;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus_absen && lulus_nilai_akhir;

    println!("Absen : {}", absen);
    println!("Nilai Akhir : {}", nilai_akhir);
    println!("Lulus : {}", lulus_final);
}

#[test]
fn char_type () {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple () {
    let mut data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;


    // destructure tuple 
    let (a, _, c) = data;

    println!("{}, {}", a,  c);

    // mutable 
    data.0 = 15;
    data.1 = 50.78;
    data.2 = false;

    println!("{:?}", data);

}

fn unit () {
    println!("Hello return tuple kosongzz");
}

#[test]
fn test_unit () {
    let result = unit();

    println!("{:?}", result);
}

#[test]
fn array() {
    let mut array = [1,2,3,4,5];

    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{}, {}", a, b);

    array[0] = 15;

    println!("{:?}", array);

    let array_length: usize = array.len();
    println!("Array Length : {}", array_length);

}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant(){
    const MINIMUM: i32 = 0;

    println!("{} {}", MINIMUM, MAXIMUM);
}


#[test]
fn variable_scope() {
    let yudis = 1;

    {
        println!("Inner yudis : {}", yudis);

        let tira = 2;

        println!("Inner Tira : {}", tira);
    }
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

#[test]
fn function_a() {
    let a: i32 = 10;
    let b: String = String::from("Yudistira");

    println!("{} {}", a, b);
}

#[test]
fn function_b() {
    let a: i32 = 20;
    let b: String = String::from("Satya");

    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name: &str = " Yudistira Satya Dewantara ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "yudistirasd";

    println!("{}", username);

    username = "yudistira.sd2";

    println!("{}", username);
}

#[test]
fn string_type () {
    let mut name: String = String::from("Yudistira");

    println!("Initial String : {}", name);

    name.push_str(" Satya");

    println!("Push New String : {}", name);

    let name_replaced = name.replace("Satya", "Dewantara");

    println!("Replace String : {}", name_replaced);

}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let mut a = 10;
    let b= a;

    println!("Init : {}, {}", a, b);

    a += 5;

    println!("Add : {}, {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Yudistira");

    let name2 = name1;

    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn ownership_clone() {
    let name1 = String::from("Yudistira");

    let name2 = name1.clone();

    println!("{}", name2);
    println!("{}", name1);
}

#[test]
fn if_expression() {
    let value = 5;

    if value >= 10 {
        println!("Uhuyy");
    } else {
        println!("Not Uhuyy");
    }
}

#[test]
fn if_elseif_expression() {
    let value = 1;

    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("Not Bad");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Very very bad");
    }
}

#[test]
fn if_elseif_expression_using_manual_let() {
    let value = 1;
    let result: &str;

    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Not Bad";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very very bad";
    }

    println!("{}", result);
}

#[test]
fn if_elseif_expression_using_automatic_let() {
    let value = 1;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very very bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 100 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result)
}

#[test]
fn nested_loop_with_label() {
    let mut number = 1;

    'first_loop: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'first_loop;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        if counter == 9 {
            break;
        }

        counter +=1 ;
    }
}

#[test]
fn array_manual_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);

        index +=1;
    }
}

#[test]
fn array_for_loop_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range_exclusive () {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5; // penulisan range exclusive

    println!("Start: {}", range.start); // start diambil
    println!("End: {}", range.end); // end tidak diambil

    for i in 0..5 {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4; // penulisan range inclusive

    println!("Start: {}", range.start()); // start diambil
    println!("End: {}", range.end()); // end diambil

    for i in range {
        println!("{}", array[i]);
    }
}

// function
fn say_hello() {
    println!("Heloo gayess");
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Good Bye {} {}", first_name, last_name);
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Yudistira", "Dewantara");
    say_goodbye("A", "B");
    say_goodbye("C", "D");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_loop(-5);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", times);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Yudistira"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n -1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);

    println!("{}", result);
}

// ownership di function
fn print_number(number: i32) {
    println!("number : {}", number);
}

fn haii(name: String) {
    println!("Hi, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;

    print_number(number);

    println!("{}", number);

    let name: String = String::from("Yudistira");
    haii(name);

    // println!("{}", name);
}

// ownership function with return
fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Yudistira");
    let last_name = String::from("Dewantara");
    let full_name = full_name(first_name, last_name);

    println!("{}", full_name);
    // println!("{}", first_name);
    // println!("{}", last_name);
}

// ownership function with return without losing ownership on heap using tuple as return
fn full_name_tuple(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name_with_tuple() {
    let first_name = String::from("Yudistira");
    let last_name = String::from("Dewantara");

    let (first_name, last_name, full_name) = full_name_tuple(first_name, last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}