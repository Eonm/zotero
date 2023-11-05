use async_trait::async_trait;
use bytes::Bytes;
use hyperx::header::{Link, RelationType};
use serde_json::Value;

use crate::{ZoteroApiAsyncExecutor, ZoteroApiError, ZoteroApiExecutor};

impl ZoteroApiExecutor for http::Request<Bytes> {
    fn execute<'a, T: serde::Deserialize<'a>, Z: crate::ZoteroApi<'a>>(
        self,
        zotero_api: &Z,
    ) -> Result<T, crate::ZoteroApiError> {
        let client = reqwest::blocking::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().unwrap_or("".to_string()),
            ));
        }

        let mut next_page = get_next_page(res.headers().clone());

        let response = res
            .json::<Value>()
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

        match next_page {
            None => {
                return T::deserialize(response)
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))
            }
            Some(_) => {
                let mut responses: Vec<Value> = vec![];
                responses.push(response);

                // follow pagination if any
                while let Some(np) = next_page {
                    let res = client
                        .execute(zotero_api.request_uri("GET", np).try_into().unwrap())
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

                    next_page = get_next_page(res.headers().clone());

                    let mut response: Vec<Value> = res
                        .json()
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

                    responses.append(&mut response);
                }

                T::deserialize(Value::Array(responses))
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))
            }
        }
    }
}

#[async_trait]
impl ZoteroApiAsyncExecutor for http::Request<Bytes> {
    async fn execute<'a, T: serde::Deserialize<'a>, Z: crate::ZoteroApi<'a> + std::marker::Sync>(
        self,
        zotero_api: &Z,
    ) -> Result<T, crate::ZoteroApiError> {
        let client = reqwest::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .await
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().await.unwrap_or("".to_string()),
            ));
        }

        let mut next_page = get_next_page(res.headers().clone());

        let response = res
            .json::<Value>()
            .await
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

        match next_page {
            None => {
                return T::deserialize(response)
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))
            }
            Some(_) => {
                let mut responses: Vec<Value> = vec![];
                responses.push(response);

                // follow pagination if any
                while let Some(np) = next_page {
                    let res = client
                        .execute(zotero_api.request_uri("GET", np).try_into().unwrap())
                        .await
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

                    next_page = get_next_page(res.headers().clone());

                    let mut response: Vec<Value> = res
                        .json()
                        .await
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

                    responses.append(&mut response);
                }

                T::deserialize(Value::Array(responses))
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))
            }
        }
    }
}

fn get_next_page(headers: reqwest::header::HeaderMap) -> Option<String> {
    headers.get(reqwest::header::LINK).and_then(|link_header| {
        link_header
            .to_str()
            .unwrap_or("")
            .parse()
            .ok()
            .and_then(|link: Link| {
                link.values()
                    .iter()
                    .find(|link_value| {
                        link_value
                            .rel()
                            .map_or(false, |rel| rel.contains(&RelationType::Next))
                    })
                    .map(|link_value| link_value.link().to_string())
            })
    })
}
