use crate::error::Error;
use serde::de::{DeserializeOwned};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackendExecResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

#[async_trait]
pub trait Frontend {
    /// fetch result(row sql)
    async fn fetch<Arg, T>(&self, context_id: &str, method: &str, arg: &Arg) -> Result<T, Error> where T: DeserializeOwned, Arg: Serialize;

    /// exec sql(row sql)
    async fn exec<Arg>(&self, context_id: &str, method: &str, arg: &Arg) -> Result<BackendExecResult, Error> where Arg: Serialize;

    /// exec sql(prepare sql)
    async fn exec_prepare<Arg>(&self, context_id: &str, method: &str, arg: &Arg) -> Result<BackendExecResult, Error> where Arg: Serialize;

    /// fetch result(prepare sql)
    async fn fetch_prepare<Arg, T>(&self, context_id: &str, method: &str, arg: &Arg) -> Result<T, Error> where T: DeserializeOwned, Arg: Serialize;
}

#[async_trait]
pub trait Backend {
    /// fetch result(row sql)
    async fn fetch<T>(&self, context_id: &str, sql: &str) -> Result<T, Error> where T: DeserializeOwned;

    /// exec sql(row sql)
    async fn exec(&self, context_id: &str, sql: &str) -> Result<BackendExecResult, Error>;

    /// exec sql(prepare sql)
    async fn exec_prepare(&self, context_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<BackendExecResult, Error>;

    /// fetch result(prepare sql)
    async fn fetch_prepare<T>(&self, context_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned;
}