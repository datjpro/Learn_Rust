fn main() {
    fn x(){
        let _a = "Hello";
        let _b = 100;
        _y()l

    }
    fn _y(){
        let _a = String::from("World");
    }

    // s1 đc cho mượn nên k thể sử dụng s1 nữa, muốn sử dụng s1 thì phải clone nó ra 1 bản sao .clone()
    let _s1 = String::from("Hello");
    let _s2 = _s1.clone();
    println!("{}", _s2);
}
