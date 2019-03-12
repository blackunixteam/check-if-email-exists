use check_if_email_exists::email_exists;
use http::{header, Request, Response, StatusCode};
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;

fn handler(request: Request<()>) -> http::Result<Response<String>> {
	let uri_str = request.uri().to_string();
	let url = Url::parse(&uri_str).unwrap();

	// Create a hash map of query parameters
	let hash_query: HashMap<_, _> = url.query_pairs().to_owned().collect();

	if let Some(ref to_email) = hash_query.get("to_email") {
		let from_email = hash_query
			.get("from_email")
			.unwrap_or(&Cow::Borrowed("user@example.org"));
		let response = Response::builder()
			.status(StatusCode::OK)
			.header(header::CONTENT_TYPE, "application/json")
			.body(email_exists(from_email, to_email).to_string())
			.expect("Failed to render response");

		Ok(response)
	} else {
		Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body("`to_email` is a required query param".to_string())
	}
}
