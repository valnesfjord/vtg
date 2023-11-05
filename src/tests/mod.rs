use crate::client::{requests::{request, file_request}, api_requests::api_call};

use httpmock::prelude::*;
use tokio::fs::File;

#[tokio::test]
async fn requests() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/test").query_param("test", "true");
        then.status(200).header("content-type", "").body("hello");
    });
    let resp = request(&server.url("/test?test=true"), "", vec![("test", "true")])
        .await
        .unwrap();
    mock.assert();
    assert_eq!("hello".to_string(), resp);
}

