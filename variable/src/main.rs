fn main() {
    let mut x = 10;
    println!("x = {}", x);
    x = 20;
    println!("x = {}", x);

    // const HANG_SO: u128 = 100_000_000_000_000;
    // println!("HANG_SO = {}", HANG_SO);

    //Shawdowing:
    // let x = 10;
    // println!("x = {}", x);
    // let x = "ten";
    // println!("x = {}", x);

    //Data types:
      // Scalar types:
    // Integer types:
    let a = 100;// decimal
    let b = 0xff;// hexadecimal
    let c = 0o77;// octal
    let d = 0b1111_0000;// binary
    let e = b'A';// byte (u8 only)
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);
    
    // Float types:
    let _f = 2.0; // f64
    let _g: f32 = 3.0; // f32

    let sum = 3+4;
    let subtraction = 5-2;
    let multiplication = 9*3;
    let division = 22.4/10.4;
    let remainder = 43%4;
    println!("sum = {}", sum);
    println!("subtraction = {}", subtraction);
    println!("multiplication = {}", multiplication);
    println!("division = {}", division);
    println!("remainder = {}", remainder);

    //Boolean types:
    let i = true;
    let y = false;
    println!("i = {}", i);
    println!("y = {}", y);

    // String types:
    let c = 'z';
    let icon = '😻';
    println!("c = {}", c);
    println!("icon = {}", icon);

      // Compound types:
    // Tuple types: 1 dạng kiểu dữ liệu kết hợp với nhiều kiểu dữ liệu khác nhau trong 1 tuple;
    let tup = (5_000, 6.4, 'a');
    println!("tup = {:?}", tup);
    let ( _string , _integer, _char) = tup;
    let _string = tup.0;
    let _integer = tup.1;
    let _char = tup.2;
    println!("string = {}", _string);
    println!("integer = {}", _integer);
    println!("char = {}", _char);

    //Array types:là 1 danh sách có kích thước cố định và chứa các phần tử cùng kiểu dữ liệu;
    let nums = [10_000, 20_000, 30_000, 40_000, 50_000];
    let get_num = nums[2];
    println!("get_num = {}", get_num);

    let _hashing = [0;32];
    println!("hashing = {:?}", _hashing);
    for i in _hashing.iter(){
      print!("{}", i);
    }

}
