pub mod test_variables {
    pub fn test_let() {
        let age = 5;
        println!("the age is {} ", age);
        println!("let age = 10 age 重新绑定");
        let age = 10;
        println!("the next age is {} ", age);
    }
    pub fn test_let_mut() {
        let mut age = 6;
        println!("the age is {} ", age);
        println!("age = 199 age 重新赋值");
        age = 199;
        println!("the age is {} ", age)
    }
    pub fn test_ignore() {
        let _x = 100;
        println!("_x :使用下划线作为变量名的开头,Rust 不要警告未使用的变量")
    }
    pub fn test_const() {
        const MAX_COUNT: i32 = 10;
        println!("const var is {}", MAX_COUNT);
    }
    pub fn test_integer() {
        let x1: i32 = 10;
        let x2: i64 = 10;
        println!(
            "x1 is i32,the value is {},x2 is i64,the value is {}",
            x1, x2
        );
    }
}
