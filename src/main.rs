pub mod variables;

fn main() {
    println!("Hello, world!");
    variables::test_variables::test_let();
    println!("===========");
    variables::test_variables::test_let_mut();
    variables::test_variables::test_ignore();
}
