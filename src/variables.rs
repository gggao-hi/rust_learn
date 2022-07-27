pub mod test_variables {
    pub fn test_let() {
        let age = 5;
        println!("the age is {} ", age);
        println!("let age = 10");
        let age = 10;
        println!("the next age is {} ", age);
    }
    pub fn test_let_mut() {
        let mut age = 6;
        println!("the age is {} ", age);
        println!("age = 199 ");
        age = 199;
        println!("the age is {} ", age);
    }
    pub fn test_ignore() {
        let _x = 100;
        println!("_x :使用下划线作为变量名的开头,Rust 不要警告未使用的变量")
    }
}
