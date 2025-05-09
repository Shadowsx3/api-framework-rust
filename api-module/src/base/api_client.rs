use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT}};
use std::sync::Arc;
use once_cell::sync::OnceCell;

pub struct ApiClient {
    client: Arc<Client>,
}

impl ApiClient {
    fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client: Arc::new(client),
        }
    }

    pub fn get_instance() -> &'static ApiClient {
        static INSTANCE: OnceCell<ApiClient> = OnceCell::new();
        INSTANCE.get_or_init(ApiClient::new)
    }

    pub fn client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }
}
