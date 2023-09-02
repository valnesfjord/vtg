#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::client::requests::request;

    use httpmock::prelude::*;
    #[tokio::test]
    async fn test_requests() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let mock = server.mock(|when, then| {
            when.method(POST).path("/test").query_param("test", "true");
            then.status(200).header("content-type", "").body("hello");
        });
        let mut hmap = HashMap::new();
        hmap.insert("test", "true");
        let resp = request(server.url("/test?test=true"), "".to_string(), hmap)
            .await
            .unwrap();
        mock.assert();
        assert_eq!("hello".to_string(), resp);
    }
}
