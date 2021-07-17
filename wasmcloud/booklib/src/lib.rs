use serde::{Deserialize, Serialize};
use serde_json;
use wapc_guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
use wasmcloud_actor_keyvalue as kv;

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(handle_http_request);
}

fn handle_http_request(req: http::Request) -> HandlerResult<http::Response> {
    let tokens: Vec<&str> = req.path.split("/").collect();

    if tokens.len() < 2 || tokens[1] != "books" {
        return Ok(http::Response::bad_request());
    }

    match req.method.as_ref() {
        "POST" => create_book(req.body),
        "GET" if tokens.len() == 3 => get_book(tokens[2]),
        "PUT" if tokens.len() == 3 => update_book(tokens[2], req.body),
        "DELETE" if tokens.len() == 3 => delete_book(tokens[2]),
        _ => Ok(http::Response::bad_request())
    }
}

fn create_book(raw: Vec<u8>) -> HandlerResult<http::Response> {
    let book: Book = serde_json::from_slice(&raw)?;
    kv::default().set(book.isbn.to_string(), serde_json::to_string(&book)?, 0)?;
    Ok(http::Response::ok())
}

fn get_book(isbn: &str) -> HandlerResult<http::Response> {
    let val = kv::default().get(isbn.to_string())?;

    if !val.exists {
        Ok(http::Response::not_found())
    } else {
        let book: Book = serde_json::from_str(&val.value)?;
        Ok(http::Response::json(
           &book, 200, "OK"
        ))
    }
}

fn update_book(isbn: &str, raw: Vec<u8>) -> HandlerResult<http::Response> {
    let book: Book = serde_json::from_slice(&raw)?;
    kv::default().set(isbn.to_string(), serde_json::to_string(&book)?, 0)?;
    Ok(http::Response::ok())
}

fn delete_book(isbn: &str) -> HandlerResult<http::Response> {
    kv::default().del(isbn.to_string())?;
    Ok(http::Response::ok())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Book {
   pub isbn: String,
   pub title: String,
   pub description: String,
   pub price: u32,
}
