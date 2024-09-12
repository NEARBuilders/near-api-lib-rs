use std::error::Error;

use near_crypto::PublicKey;
use near_primitives::{
    serialize::dec_format,
    types::{Balance, Nonce},
    views::{AccountView, QueryRequest},
};
use reqwest::{header, Client};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use url::form_urlencoded;

#[derive(Clone)]
pub struct FastNearHTTPClient {
    pub client: Client,
    headers: header::HeaderMap,
    server_addr: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct AccessKeyInfoView {
    pub nonce: Nonce,
    #[serde(with = "dec_format")]
    pub allowance: Option<Balance>,
    pub receiver_id: String,
    pub method_names: Vec<String>,
    pub public_key: PublicKey,
    pub r#type: String,
}

pub trait ViewFunctionData: std::marker::Send + 'static {}

impl ViewFunctionData for String {}
impl ViewFunctionData for Vec<u8> {}
impl ViewFunctionData for serde_json::Value {}

trait QueryInfo {
    fn url_fragment(&self, server_addr: &str) -> String;
}

impl QueryInfo for QueryRequest {
    fn url_fragment(&self, server_addr: &str) -> String {
        match self {
            QueryRequest::ViewAccount { account_id } => {
                format!("{}/account/{}", server_addr, account_id)
            }
            QueryRequest::ViewCode { account_id } => {
                format!("{}/account/{}/contract/methods", server_addr, account_id)
            }
            QueryRequest::ViewAccessKey {
                account_id,
                public_key,
            } => format!("{}/account/{}/key/{}", server_addr, account_id, public_key),
            QueryRequest::CallFunction {
                account_id,
                method_name,
                args,
            } => {
                // Convert the Vec<u8> into a String
                let json_str =
                    String::from_utf8(args.clone().to_vec()).expect("Failed to convert to String");

                // Parse the JSON string into a serde_json::Value
                let parsed_json: Value =
                    serde_json::from_str(&json_str).expect("Failed to parse JSON");

                let mut query_params = vec![];
                // Check if the JSON value is an object and iterate over key-value pairs
                if let Value::Object(map) = parsed_json {
                    for (key, value) in map {
                        // Convert the value to a string
                        let value_str = value.to_string();
                        // Remove quotes around strings if necessary
                        let value_str = if value_str.starts_with('"') && value_str.ends_with('"') {
                            &value_str[1..value_str.len() - 1]
                        } else {
                            &value_str
                        };
                        query_params.push((key, value_str.to_string()));
                    }
                } else {
                    println!("The JSON value is not an object");
                }

                // Encode query parameters
                let query_string: String = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(query_params)
                    .finish();
                format!(
                    "{}/account/{}/view/{}?{}",
                    server_addr, account_id, method_name, query_string
                )
            }
            _ => panic!("invalid QueryRequest"),
        }
    }
}

impl FastNearHTTPClient {
    pub fn new(url: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::with_capacity(2);
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: Client::new(),
            headers: headers.clone(),
            server_addr: url.to_owned(),
        }
    }

    /// Get a shared reference to the headers.
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// Get an exclusive reference to the headers.
    pub fn headers_mut(&mut self) -> &mut reqwest::header::HeaderMap {
        &mut self.headers
    }

    async fn fetch_data<T: DeserializeOwned>(
        &self,
        query: QueryRequest,
    ) -> Result<T, Box<dyn Error>> {
        let server_address = self.server_addr.clone();
        let url = query.url_fragment(&server_address);
        // let url = format!("{server_address}{url_fragment}");
        println!("url {:#?}", url);

        let request = self.client.get(url).headers(self.headers.clone());
        let response = request.send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                println!("API call is success")
            }
            non_status_ok => {
                let err_str = format!("API call failed with status code {non_status_ok}");
                return Err(err_str.into());
            }
        }

        let result = response.json::<T>().await;
        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub async fn account_info(
        &self,
        query: QueryRequest,
    ) -> Result<AccountView, Box<dyn std::error::Error>> {
        self.fetch_data::<AccountView>(query).await
    }

    pub async fn contract_methods(
        &self,
        query: QueryRequest,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.fetch_data::<Vec<String>>(query).await
    }

    pub async fn access_key(
        &self,
        query: QueryRequest,
    ) -> Result<AccessKeyInfoView, Box<dyn std::error::Error>> {
        self.fetch_data::<AccessKeyInfoView>(query).await
    }

    pub async fn view_function<T>(
        &self,
        query: QueryRequest,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: ViewFunctionData + serde::de::DeserializeOwned,
    {
        self.fetch_data::<T>(query).await
    }
}
