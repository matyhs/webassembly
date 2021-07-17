use std::io::Read;
use std::{error::Error, fs::File};

use wasm3::{CallContext, Environment};
use wasm3::Module;

fn main() -> Result<(), Box<dyn Error>> {

     let env = Environment::new()?;
     let rt = env.create_runtime(1024 * 60)?;

     let bytes = {
       let mut f =
      File::open(
 "../importer/target/wasm32-unknown-unknown/release/importer.wasm")?;

       let mut bytes = vec![];
       f.read_to_end(&mut bytes)?;
       bytes
     };

     let module = Module::parse(&env, &bytes)?;
     let mut module = rt.load_module(module)?;

     if let Err(_e) = module.link_closure(
        "math",
        "pi",
        move |_ctx: &CallContext, ()| -> f32 {
           use std::f32;

           f32::consts::PI
         }) {
         return Err("Failed to link closure".into());
     }

     let area = module.find_function::<(i32, i32), f32>("area")?;

     println!("Result: {}", area.call(2, 1)?);

     Ok(())
}