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

pub async fn file_request(
    url: &str,
    file_path: &Path,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let boundary: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Boundary: {}", boundary);
    let mut file = image_data(file_path, &boundary).expect("Error while reading file");
    write!(file, "--").unwrap();
    debug!("[FILE] Request body len: {}", file.len());
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(file.into())
        .unwrap();
    println!("Request: {:?}", req);
    let res = CLIENT.request(req).await?;
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body.to_vec())?;

    Ok(body_str)
}

fn image_data(file_path: &Path, boundary: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(file_path)?;
    let mut file_data = vec![];
    f.read_to_end(&mut file_data).unwrap();
    let mut data = Vec::new();
    let filename = file_path.file_name().unwrap().to_str().unwrap();
    write!(data, "--{}\r\n", boundary)?;
    let mime_type = from_path(file_path).first_or_octet_stream();
    write!(
        data,
        "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
        filename
    )?;
    println!("Mime type: {}", mime_type.as_ref());
    //write!(data, "Content-Type: {}\r\n", mime_type.as_ref())?;
    write!(data, "\r\n")?;
    data.write_all(&file_data)?;
    write!(data, "\r\n")?;
    write!(data, "\r\n--{}", boundary)?;
    Ok(data)
}
