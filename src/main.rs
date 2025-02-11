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