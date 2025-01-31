//! # Graphql
//!
//! The GraphQL API allows you to query and mutate your Appwrite server using
//! GraphQL.

use serde_json::{Map, Value};

use crate::{api_params, client::Client, enumm::HttpMethod, error::Error};

pub struct Graphql;

impl Graphql {
    /// GraphQL endpoint
    ///
    /// Execute a GraphQL mutation.
    pub async fn query(client: &Client, query: Map<String, Value>) -> Result<Value, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/graphql";

        let api_params = api_params!(
            "query"=> Some(query),
        );

        let mut api_headers = reqwest::header::HeaderMap::new();
        api_headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        api_headers.insert("x-sdk-graphql", "true".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }

    /// GraphQL endpoint
    ///
    /// Execute a GraphQL mutation.
    pub async fn mutation(client: &Client, query: Map<String, Value>) -> Result<Value, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/graphql/mutation";

        let api_params = api_params!(
            "query"=> Some(query),
        );

        let mut api_headers = reqwest::header::HeaderMap::new();
        api_headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse()?);
        api_headers.insert("x-sdk-graphql", "true".parse()?);

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &api_params, None)
            .await?;

        Ok(res.json().await?)
    }
}
