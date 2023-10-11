use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body, Client, Method, Request,
};
use lazy_static::lazy_static;
use log::debug;
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
    url: String,
    access_token: String,
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
    Ok(String::from_utf8(bytes.to_vec())
        .map_err(|e| HyperRequestError::ResponseError(e.to_string()))?)
}
