use crate::error::Error;
use serde::de::{DeserializeOwned};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutorExecResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

#[async_trait]
pub trait Executor {
    /// fetch result(row sql)
    async fn fetch<T>(&self, context_id: &str, sql: &str) -> Result<T, Error>
        where
            T: DeserializeOwned;

    /// exec sql(row sql)
    async fn exec(&self, context_id: &str, sql: &str) -> Result<ExecutorExecResult, Error>;

    /// exec sql(prepare sql)
    async fn exec_prepare(&self, context_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<ExecutorExecResult, Error>;

    /// fetch result(prepare sql)
    async fn fetch_prepare<T>(&self, context_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned;
}