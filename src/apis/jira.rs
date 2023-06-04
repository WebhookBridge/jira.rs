use base64::{engine::general_purpose::STANDARD, Engine as _};

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, ClientBuilder};
use std::error::Error;

pub struct Credentials {
    pub email: String,
    pub token: String,
}

impl Credentials {
    pub fn new(email: &str, token: &str) -> Self {
        Credentials {
            email: email.to_string(),
            token: token.to_string(),
        }
    }
}

pub struct Jira {
    client: Client,
    credentials: Credentials,
    host: String,
}

impl Jira {
    pub fn new(domain: String, email: String, api_token: String) -> Result<Self, Box<dyn Error>> {
        let auth_string = format!(
            "Basic {}",
            STANDARD.encode(format!("{}:{}", email, api_token))
        );

        let auth_header = HeaderValue::from_str(auth_string.as_str())
            .expect("Unable to set the auth string as a header value");

        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, auth_header);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Unable to build the reqwest client");

        Ok(Jira {
            host: domain,
            client,
            credentials: Credentials::new(email.as_str(), api_token.as_str()),
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_api_call() {
//         let jira_client = Jira::new("your-domain", "your-email", "your-api-token").unwrap();
//         let response = jira_client.api_call("rest/api/3/myself").await.unwrap();
//         println!("{}", response);
//     }
// }
