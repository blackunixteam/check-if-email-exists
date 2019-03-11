// Try first with simple example

use http::{Request, Response, StatusCode, header};

 fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body("<!doctype html><html><head><title>A simple deployment with Now!</title></head><body><h1>Welcome to Rust on Now</h1></body></html>".to_string())
        .expect("failed to render response");

     Ok(response)
}

// extern crate check_if_email_exists;

// use check_if_email_exists::email_exists;
// use http::{header, Request, Response, StatusCode, Uri};
// use std::collections::HashMap;
// use url::Url;

// fn handler(request: Request<()>) -> http::Result<Response<String>> {
// 	let uri_str = request.uri().to_string();
// 	let url = Url::parse(&uri_str).unwrap();

// 	// Create a hash map of query parameters
// 	let hash_query: HashMap<_, _> = url.query_pairs().to_owned().collect();

// 	if let hash_query.get("to_email") = Some(ref to_email) {
// 		let from_email = hash_query.get("from_email").or_else("user@example.org");

// 		let response = Response::builder()
// 			.status(StatusCode::OK)
// 			.header(header::CONTENT_TYPE, "application/json")
// 			.body(email_exists(from_email, to_email).to_string())
// 			.expect("Failed to render response");
// 	} else {
// 		Response::builder()
// 			.status(StatusCode::BAD_REQUEST)
// 			.body("`to_email` is a required query param".to_string())
// 	}
// }
