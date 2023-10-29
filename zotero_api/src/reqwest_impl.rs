use async_trait::async_trait;
use bytes::Bytes;
use http::header;
use hyperx::header::{Link, LinkValue, RelationType};
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
            return Err(ZoteroApiError::AuthenticationError(res.text().unwrap()));
        }

        match &res.headers().get(header::LINK) {
            None => Ok(T::deserialize(
                res.json::<Value>()
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?,
            )
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?),
            Some(v) => {
                let link: Link = v.to_str().unwrap().parse().unwrap();
                let mut next: Option<LinkValue> = None;
                for l in link.values() {
                    match l.rel() {
                        None => {}
                        Some(reltypes) => {
                            if reltypes.contains(&RelationType::Next) {
                                next = Some(l.clone());
                            }
                        }
                    }
                }
                let mut chain_responses: Vec<Value> = res
                    .json()
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;
                while next.is_some() {
                    let next_link = next.unwrap();
                    next = None;
                    let uri: String = next_link.link().to_string();
                    let res = client
                        .execute(zotero_api.request_uri("GET", uri).try_into().unwrap())
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;
                    match &res.headers().get(header::LINK) {
                        None => {
                            next = None;
                        }
                        Some(v) => {
                            let link: Link = v.to_str().unwrap().parse().unwrap();
                            for l in link.values() {
                                match l.rel() {
                                    None => {}
                                    Some(reltypes) => {
                                        if reltypes.contains(&RelationType::Next) {
                                            next = Some(l.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let mut v: Vec<Value> = res
                        .json()
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;
                    chain_responses.append(&mut v);
                }
                Ok(T::deserialize(Value::Array(chain_responses))
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?)
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
            return Err(ZoteroApiError::AuthenticationError(res.text().await.unwrap()));
        }

        match &res.headers().get(header::LINK) {
            None => Ok(T::deserialize(
                res.json::<Value>()
                    .await
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?,
            )
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?),
            Some(v) => {
                let link: Link = v.to_str().unwrap().parse().unwrap();
                let mut next: Option<LinkValue> = None;
                for l in link.values() {
                    match l.rel() {
                        None => {}
                        Some(reltypes) => {
                            if reltypes.contains(&RelationType::Next) {
                                next = Some(l.clone());
                            }
                        }
                    }
                }
                let mut chain_responses: Vec<Value> = res
                    .json()
                    .await
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;
                while next.is_some() {
                    let next_link = next.unwrap();
                    next = None;
                    let uri: String = next_link.link().to_string();
                    let res = client
                        .execute(zotero_api.request_uri("GET", uri).try_into().unwrap())
                        .await
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;
                    match &res.headers().get(header::LINK) {
                        None => {
                            next = None;
                        }
                        Some(v) => {
                            let link: Link = v.to_str().unwrap().parse().unwrap();
                            for l in link.values() {
                                match l.rel() {
                                    None => {}
                                    Some(reltypes) => {
                                        if reltypes.contains(&RelationType::Next) {
                                            next = Some(l.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let mut v: Vec<Value> = res
                        .json()
                        .await
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;
                    chain_responses.append(&mut v);
                }
                Ok(T::deserialize(Value::Array(chain_responses))
                    .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?)
            }
        }
    }
}
