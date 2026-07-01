use crate::errors::ServiceError;
use crate::search::query::SearchQuery;
use crate::search::result::SearchPageResult;
use async_trait::async_trait;

#[async_trait]
pub trait SearchRepositoryPort<Q, R>: Send + Sync {
    async fn search(&self, query: SearchQuery<Q>) -> Result<SearchPageResult<R>, ServiceError>;
}
