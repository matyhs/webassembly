use std::io;
use std::io::Read;
use std::{error::Error, fs::File};

use wasm3::Environment;
use wasm3::Module;

fn main() -> Result<(), Box<dyn Error>> {
     let env = Environment::new()?;
     let rt = env.create_runtime(1024 * 60)?;
     let mut f = File::open("../calc/target/wasm32-unknown-unknown/release/calc.wasm")?;
     let mut bytes = vec![];
     f.read_to_end(&mut bytes)?;
     let module = Module::parse(&env, &bytes)?;
     let module = rt.load_module(module)?;

     let add = module.find_function::<(i32, i32), i32>("add")?;
     let sub = module.find_function::<(i32, i32), i32>("sub")?;
     let mul = module.find_function::<(i32, i32), i32>("mul")?;
     let div = module.find_function::<(i32, i32), i32>("div")?;

     let mut first_num = String::new();
     let mut second_num = String::new();
     let mut operation = String::new();

     println!("First Number:");
     io::stdin()
          .read_line(&mut first_num)
          .expect("Failed to read line");

     let first_num: i32 = first_num.trim().parse().expect("Not a number!");
     
     println!("Second Number:");
     io::stdin()
          .read_line(&mut second_num)
          .expect("Failed to read line");

     let second_num: i32 = second_num.trim().parse().expect("Not a number!");

     println!("Operation:");
     io::stdin()
          .read_line(&mut operation)
          .expect("Failed to read line");

     match operation.trim() {
          "+" => println!("Adding {} and {}: {}", first_num, second_num, add.call(first_num, second_num)?),
          "-" => println!("Subtract {} and {}: {}", first_num, second_num, sub.call(first_num, second_num)?),
          "*" => println!("Multiply {} and {}: {}", first_num, second_num, mul.call(first_num,second_num)?),
          "/" => println!("Divide {} and {}: {}", first_num, second_num,div.call(first_num, second_num)?),
          _ => println!("Unknown operation"),
     }

     Ok(())
}