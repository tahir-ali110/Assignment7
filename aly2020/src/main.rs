mod  printing {
    pub mod math{
        pub fn table(data:u32){
            println!("Calcuations");
            for count in 1..=3{
                println!("{}*{}={}", data,count,data*count);
            }
        }
    }
}

pub mod lib;

use std::io;

fn main() {
    loop {

    println!("Please input your number:");
    
    let mut input1= String::new();

    io::stdin().read_line(&mut input1)
    .expect("failed to read line");

    // let input1:u32= input1.trim().parse().unwrap();

    let input1: u32 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };

    println!("You Entered: {}",input1);
    crate::printing::math::table(input1);
    printing::math::table(input1);
    lib::table(input1);

    break;

   
    }

}   