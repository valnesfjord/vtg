use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Client, Method, Request,
};
use lazy_static::lazy_static;
use log::debug;
use std::{fs, io::Read, path::Path};
use std::{
    fs::File,
    io::{self, Write},
};
use streamer::{hyper::Streamer, StreamExt};
#[derive(Debug)]
pub enum HyperRequestError {
    RequestError(hyper::Error),
    ResponseError(String),
}
lazy_static! {
    static ref CLIENT: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body> = {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_all_versions()
            .build();

        Client::builder().build(https)
    };
}
pub async fn request(
    url: &str,
    access_token: &str,
    body: Vec<(&str, &str)>,
) -> Result<String, HyperRequestError> {
    let form_body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(body.iter())
        .finish();
    debug!("Request body: {}", form_body);
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header(CONTENT_LENGTH, form_body.len())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(form_body))
        .unwrap();
    let res = CLIENT
        .request(req)
        .await
        .map_err(HyperRequestError::RequestError)?;

    let bytes = hyper::body::to_bytes(res.into_body())
        .await
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?;
    String::from_utf8(bytes.to_vec()).map_err(|e| HyperRequestError::ResponseError(e.to_string()))
}
use mime_guess::from_path;
use rand::distributions::Alphanumeric;
use rand::Rng;

fn image_data(file_path: &str, boundary: &str) -> io::Result<Vec<u8>> {
    let f = fs::read(file_path)?;
    let mut data = Vec::new();
    write!(data, "--------------------------{}\r\n", boundary)?;
    let mime_type = from_path(file_path).first_or_octet_stream();
    let filename = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?;
    write!(
        data,
        "{}",
        &format!(
            "Content-Disposition: form-data; name=\"smfile\"; filename=\"{}\"\r\n",
            filename
        )
    )?;
    write!(data, "Content-Type: {}\r\n", mime_type.as_ref())?;
    write!(data, "\r\n")?;

    data.extend_from_slice(&f);

    write!(data, "\r\n")?;
    write!(data, "--------------------------{}--\r\n", boundary)?;
    Ok(data)
}

pub async fn file_request(url: &str, mut file: File) -> Result<String, HyperRequestError> {
    let streaming = Streamer::new(file.try_clone().unwrap());
    let body = streaming.streaming();
    let mut fl: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut fl);

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(CONTENT_LENGTH, fl.len())
        .header(CONTENT_TYPE, "multipart/form-data;")
        .body(body)
        .unwrap();
    let res = CLIENT
        .request(req)
        .await
        .map_err(HyperRequestError::RequestError)?;

    let bytes = hyper::body::to_bytes(res.into_body())
        .await
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?;
    String::from_utf8(bytes.to_vec()).map_err(|e| HyperRequestError::ResponseError(e.to_string()))
}
