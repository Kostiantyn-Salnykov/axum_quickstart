use crate::errors::ServiceError;
use crate::search::query::SearchQuery;
use crate::search::result::SearchPageResult;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait SearchRepositoryPort<Q, R>: Send + Sync {
    async fn search(&self, query: SearchQuery<Q>) -> Result<SearchPageResult<R>, ServiceError>;
    async fn stream(
        &self,
        query: SearchQuery<Q>,
    ) -> Result<Receiver<Result<R, ServiceError>>, ServiceError>;
}
