#[derive(Debug, Clone)]
pub struct SearchPaginationResult {
    pub has_more: bool,
    pub next_cursor: Option<String>,
    pub skip: Option<u64>,
    pub limit: u64,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SearchPageResult<R> {
    pub items: Vec<R>,
    pub pagination: SearchPaginationResult,
}
