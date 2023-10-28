#[allow(dead_code)]
#[allow(unused_variables)]
mod generated_httpmock;

#[cfg(test)]
mod tests {
    use super::generated_httpmock::*;
    use httpmock::prelude::*;
    use sdk::*;

    async fn send_create_image_variant(
        client: &Client,
        img: Vec<u8>,
    ) -> Result<ResponseValue<types::ImagesResponse>, Error> {
        let img = reqwest::multipart::Part::bytes(img);
        let req = client.create_image_variation().image(img);
        req.send().await
    }

    #[tokio::test]
    #[should_panic(expected = "No request has been received by the mock server.")]
    async fn wrong_create_image_variant_url() {
        let server = MockServer::start();
        let mock = server.create_image_variation(|_when, _then| {});
        let client = Client::new(&server.url("wrong_base_url"));
        let res = send_create_image_variant(&client, b"foo_image".to_vec()).await;
        dbg!(&res);
        assert!(matches!(res, Err(Error::CommunicationError(_))));
        assert_eq!(mock.hits(), 0);
        mock.assert_async().await;
    }
    #[tokio::test]
    async fn smoke_create_image_variant() {
        let server = MockServer::start();
        let img_ascii = "bogus-img-data-1337-hax0r";
        let response = types::ImagesResponse {
            created: 1,
            data: vec![types::Image {
                b64_json: None,
                url: Some("https://ms.com/image.png".to_owned()),
            }],
        };
        let mock = server.create_image_variation(|when, then| {
            // no binary data checks supported:
            // https://github.com/alexliesenfeld/httpmock/issues/39#issuecomment-983140233
            // but we can check ascii text
            when.image(()).into_inner().body_contains(img_ascii);
            then.ok(&response);
        });
        let client = Client::new(&server.url(""));
        let img = reqwest::multipart::Part::bytes(img_ascii.as_bytes().to_vec());
        let req = client.create_image_variation().image(img);
        let res = req.send().await.unwrap();
        assert_eq!(
            serde_json::to_string(&res.into_inner()).unwrap(),
            serde_json::to_string(&response).unwrap(),
        );
        mock.assert_async().await;
    }
}
