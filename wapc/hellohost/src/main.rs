extern crate wapc;
use std::error::Error;
use wapc::WapcHost;
use wasmtime_provider::WasmtimeEngineProvider;

pub fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let module = load_file()?;
  let engine = WasmtimeEngineProvider::new(&module, None);
  let host = WapcHost::new(Box::new(engine), move |_id, bd, ns, op, payload| {
    handle_callback(bd, ns, op, payload)
    })?;

  let res = host.call("SayHello", b"Alice")?;
  let s = std::str::from_utf8(&res)?;
  println!("{}", s);
  assert_eq!(s, "Ahoy There, Alice!");
  Ok(())
}

fn handle_callback(
    _binding: &str,
    _namespace: &str,
    operation: &str,
    _payload: &[u8],
  ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    if operation == "GetGreeting" {
      Ok(b"Ahoy There".to_vec())
    } else {
      Err("Unsupported host call!".into())
    }
  }

  fn load_file() -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("../helloguest/target/wasm32-unknown-unknown/release/helloguest.wasm")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}