use std::sync::Arc;

use crate::errors::ServiceError;
use crate::search::query::SearchQuery;
use crate::search::repository::SearchRepositoryPort;
use crate::search::result::SearchPageResult;
use crate::search::use_case::SearchUseCase;
use async_trait::async_trait;

#[derive(Clone)]
pub struct SearchService<Q, R> {
    repository: Arc<dyn SearchRepositoryPort<Q, R>>,
}

impl<Q, R> SearchService<Q, R> {
    pub fn new(repository: Arc<dyn SearchRepositoryPort<Q, R>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<Q, R> SearchUseCase<Q, R> for SearchService<Q, R>
where
    Q: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    async fn search(&self, query: SearchQuery<Q>) -> Result<SearchPageResult<R>, ServiceError> {
        self.repository.search(query).await
    }
}
