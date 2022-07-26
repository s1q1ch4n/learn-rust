
use http::method::Method;
use crate::http;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}