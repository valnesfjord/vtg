use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Client, Method, Request,
};
use lazy_static::lazy_static;
use log::debug;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::{self, Write};
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

#[derive(Clone, Debug)]
pub struct File {
    pub filename: String,
    pub content: Vec<u8>,
}
pub async fn files_request(
    url: &str,
    files: &Vec<File>,
    data: Option<Vec<(&str, &str)>>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let boundary: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let mut body = vec![];
    let query = match data {
        Some(data) => {
            "?".to_owned()
                + &form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish()
        }
        None => String::new(),
    };
    for f in files {
        let file = file_data(f.clone(), &boundary).expect("Error while reading file");
        body.extend(file);
        body.extend_from_slice(b"\r\n--");
        body.extend_from_slice(boundary.as_bytes());
    }
    body.extend_from_slice(b"--");
    debug!("[FILE] Request body len: {}", body.len());
    let req = Request::builder()
        .method(Method::POST)
        .uri(url.to_owned() + &query)
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(body.into())
        .unwrap();
    let res = CLIENT.request(req).await?;
    println!("{:?}", res);
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body.to_vec())?;
    println!("{:?}", body_str);

    Ok(body_str)
}

fn file_data(file: File, boundary: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let filename = file.filename;
    write!(data, "--{}\r\n", boundary)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"photo\"; filename=\"{}\"\r\n",
        filename
    )?;
    write!(data, "\r\n")?;
    data.write_all(&file.content)?;
    write!(data, "\r\n")?;
    Ok(data)
}
