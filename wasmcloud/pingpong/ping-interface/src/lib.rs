mod generated;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    Handlers::register_ping(ping);
}

fn ping(_request: Ping) -> HandlerResult<Pong> {
    Ok(Pong::default()) // TODO: Provide implementation.
}
