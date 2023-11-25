use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Client, Method, Request,
};
use lazy_static::lazy_static;
use log::debug;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::{self, Write};

use crate::structs::context::Platform;
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
pub async fn get_file(url: &str) -> Result<File, HyperRequestError> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Body::empty())
        .unwrap();
    let res = CLIENT
        .request(req)
        .await
        .map_err(HyperRequestError::RequestError)?;

    let content_type = res
        .headers()
        .get("Content-Type")
        .and_then(|value| value.to_str().ok());
    let mut filename = String::new();
    let ftype = content_type
        .map(|value| {
            let mut parts = value.split('/');
            let media_type = parts.next().unwrap_or("");
            let subtype = parts.next().unwrap_or("");
            filename = format!("something.{}", subtype);
            match (media_type, subtype) {
                ("image", _) => FileType::Photo,
                ("video", _) => FileType::Video,
                ("audio", _) => FileType::Audio,
                _ => FileType::Document,
            }
        })
        .unwrap_or(FileType::Other);

    let bytes = hyper::body::to_bytes(res.into_body())
        .await
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?;

    Ok(File {
        filename,
        content: bytes.to_vec(),
        ftype,
    })
}

#[derive(Clone, Debug)]
pub struct File {
    pub filename: String,
    pub content: Vec<u8>,
    pub ftype: FileType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    Photo,
    Video,
    Audio,
    Document,
    Voice,
    VideoNote,
    Animation,
    Other,
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::Photo => "Photo".to_string(),
            FileType::Video => "Video".to_string(),
            FileType::Audio => "Audio".to_string(),
            FileType::Document => "Document".to_string(),
            FileType::Voice => "Voice".to_string(),
            FileType::VideoNote => "Video_Note".to_string(),
            FileType::Animation => "Animation".to_string(),
            FileType::Other => "Other".to_string(),
        }
    }
}
pub async fn files_request(
    url: &str,
    files: &[File],
    data: Option<Vec<(&str, &str)>>,
    platform: Platform,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    debug!("Request url: {}", url);
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
    for (index, f) in files.iter().enumerate() {
        let mut name: String = f.ftype.to_string();
        if platform == Platform::VK {
            name = "file".to_string();
        }
        if index != 0 {
            name = name + &index.to_string();
        }
        let mut is_last: bool = false;
        if index == files.len() - 1 {
            is_last = true;
        }
        let file = file_data(
            f.clone(),
            &boundary,
            &name.replace('_', "").to_lowercase(),
            is_last,
        )
        .expect("Error while reading file");
        body.extend(file);
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
    let body = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body.to_vec())?;
    debug!("Response body: {}", body_str);

    Ok(body_str)
}

fn file_data(file: File, boundary: &str, name: &str, is_last: bool) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let filename = file.filename;
    write!(data, "--{}\r\n", boundary)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
        name, filename
    )?;
    write!(data, "\r\n")?;
    data.write_all(&file.content)?;
    write!(data, "\r\n")?;
    if is_last {
        write!(data, "\r\n--{}", boundary)?;
    }
    Ok(data)
}
