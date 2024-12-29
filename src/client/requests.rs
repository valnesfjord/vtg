use crate::{client::structs::FastFormSerializer, structs::context::Platform};
use bytes::Bytes;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Method, Request,
};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use lazy_static::lazy_static;
use log::debug;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::borrow::Cow;
use std::io::{self, Write};

/// Error enum for HyperRequestError
/// # Variants
/// * `RequestError` - Request error
/// * `ResponseError` - Response error
#[derive(Debug)]
pub enum HyperRequestError {
    RequestError(hyper_util::client::legacy::Error),
    ResponseError(String),
}
lazy_static! {
    static ref CLIENT: Client<HttpsConnector<HttpConnector>, Full<Bytes>> =
        Client::builder(TokioExecutor::new()).build::<_, Full<Bytes>>(HttpsConnector::new());
    static ref EMPTY_CLIENT: Client<HttpsConnector<HttpConnector>, Empty<Bytes>> =
        Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(HttpsConnector::new());
}
/// Sends a POST request with the specified access token and body.
/// # Returns
///
/// Returns the response body as a string.
pub async fn request(
    url: &str,
    access_token: &str,
    body: Vec<(Cow<'_, str>, Cow<'_, str>)>,
) -> Result<String, HyperRequestError> {
    let mut serializer = FastFormSerializer::new(&body);
    let form_body = serializer.extend_pairs(&body).finish();
    debug!("Request body: {}", form_body);
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header(CONTENT_LENGTH, form_body.len())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Full::from(form_body))
        .unwrap();
    let res = CLIENT
        .request(req)
        .await
        .map_err(HyperRequestError::RequestError)?;
    let body = res.collect().await.unwrap().to_bytes();
    String::from_utf8(body.to_vec()).map_err(|e| HyperRequestError::ResponseError(e.to_string()))
}
/// Sends a GET request to the specified URL, download files from it.
/// # Returns
///
/// Returns a File struct with the file content and type.
pub async fn get_file(url: &str) -> Result<File, HyperRequestError> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Empty::new())
        .unwrap();
    let res = EMPTY_CLIENT
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

    let bytes = res.collect().await.unwrap().to_bytes();

    Ok(File {
        filename,
        content: bytes.to_vec(),
        ftype,
    })
}

/// File struct with the file content and type.
/// # Fields
/// * `filename` - Name of the file, please use real name like cat.jpg or video.mp4
/// * `content` - File content
/// * `ftype` - File type
#[derive(Clone, Debug)]
pub struct File {
    pub filename: String,
    pub content: Vec<u8>,
    pub ftype: FileType,
}

/// File type enum.
/// # Variants
/// * `Photo` - Photo file
/// * `Video` - Video file
/// * `Audio` - Audio file
/// * `Document` - Document file
/// * `Voice` - Voice file
/// * `VideoNote` - Video note file
/// * `Animation` - Animation file
/// * `Other` - Other file
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

use std::fmt;

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FileType::Photo => "Photo",
            FileType::Video => "Video",
            FileType::Audio => "Audio",
            FileType::Document => "Document",
            FileType::Voice => "Voice",
            FileType::VideoNote => "Video_Note",
            FileType::Animation => "Animation",
            FileType::Other => "Other",
        };
        write!(f, "{}", s)
    }
}
/// Sends a POST request with the specified files data to VK or Telegram servers.
///
/// # Returns
///
/// Returns the response body as a string.
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
            let mut serializer = FastFormSerializer::new_vec(&data);
            "?".to_owned() + &serializer.extend_vec_pairs(&data).finish()
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
    let body = res.collect().await.unwrap().to_bytes();
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
