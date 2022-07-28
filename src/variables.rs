pub mod out {
    mod test_variables {
        use num::Complex;

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
            println!("x1 is i32,the value is {x1},x2 is i64,the value is {x2}");
            let result = x1.checked_pow(2);
            println!("10的2次方");
            match result {
                Some(p) => println!("result is {p}"),
                None => println!("result is none"),
            }
            println!("10的10次方");
            let result = x1.checked_pow(10);
            match result {
                Some(p) => println!("result is {p}"),
                None => println!("result is largest"),
            }
        }

        pub fn test_float() {
            let y1: f32 = 0.1;
            let y2: f32 = 0.2;
            println!("y1, y2 是 f32");
            println!(
                "y1:{y1};y2:{y2};y1+y2:{},y1+y2==0.3:{}",
                (y1 + y2),
                (y1 + y2 == 0.3)
            );
            println!(
                "y1:{y1};y2:{y2};y1+y2:{},y1+y2<=0.30001:{}",
                (y1 + y2),
                (y1 + y2 <= 0.30001)
            );

            let y1 = 0.1;
            let y2 = 0.2;
            println!("y1, y2 是 f64");
            println!(
                "y1:{y1};y2:{y2};y1+y2:{},y1+y2==0.3:{}",
                (y1 + y2),
                (y1 + y2 == 0.3)
            );
            println!("由于f64会出现精度丢失,所以使用指定精度要求去比较");
            println!(
                "y1:{y1};y2:{y2};y1+y2:{},y1+y2<=0.30001:{}",
                (y1 + y2),
                (y1 + y2 <= 0.30001)
            );
            let y3: f32 = (-42.1_f32).sqrt();
            println!("y3:{y3}");
            let y4 = y3 / 100.0_f32;

            if y4.is_nan() {
                println!("y3/100 is NaN");
            } else {
                println!("y3/100 is {y4}");
            }
        }

        pub fn test_range() {
            for i in 1..5 {
                println!("1..5:{i}");
            }
            for i in 1..=5 {
                println!("1..=5:{i}");
            }
        }
        pub fn test_complex() {
            let c1 = Complex { re: 1, im: 2 };
            let c2 = Complex::new(-1, -2);
            let c3 = c1 + c2;
            println!("sum is re:{};im:{}", c3.re, c3.im);
        }

        pub fn test_char() {
            let c: char = '日';
            println!("日 size :{}", (std::mem::size_of_val(&c)));
            let c = 'a';
            println!("a size :{}", (std::mem::size_of_val(&c)));
        }
        pub fn test_sum(a: i32, b: i32) -> i32 {
            let s = a + b;
            return s;
        }
    }

    pub fn out() {
        test_variables::test_let();
        println!("===========");
        test_variables::test_let_mut();
        println!("===========");
        test_variables::test_ignore();
        println!("===========");
        test_variables::test_const();
        println!("===========");
        test_variables::test_integer();
        println!("===========");
        test_variables::test_float();
        println!("===========");
        test_variables::test_range();
        println!("===========");
        test_variables::test_complex();
        println!("===========");
        test_variables::test_char();
        println!("===========");
        let s = test_variables::test_sum(10, 23);
        println!("a+b:{s}");
    }
}
