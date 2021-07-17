#[link(wasm_import_module = "utilities")]
extern "C" {
     pub fn random() -> i32;
}

#[no_mangle]
extern fn addto(x: i32) -> i32 {
     unsafe {
       x + random()
     }
}

#[link(wasm_import_module = "math")]
extern "C" {
     pub fn pi() -> f32;
}

#[no_mangle]
extern fn area(line_segment: i32, shape: i32) -> f32 {
     unsafe {
          match shape {
               1 => line_segment as f32 * line_segment as f32,
               2 => pi() * line_segment as f32 * line_segment as f32,
               _ => 0.
          }
     }
}