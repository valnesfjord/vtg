use crate::client::requests::request;

use httpmock::prelude::*;

#[tokio::test]
async fn requests() {
    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
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
