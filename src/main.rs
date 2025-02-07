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
