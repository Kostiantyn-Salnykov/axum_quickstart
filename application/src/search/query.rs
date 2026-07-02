use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct SearchQuery<F> {
    pub searching: Option<SearchSearching<F>>,
    pub filtration: Option<SearchFilterNode<F>>,
    pub sorting: Vec<SearchSortRule<F>>,
    pub projection: Option<SearchProjection<F>>,
    pub pagination: SearchPagination,
}

#[derive(Debug, Clone)]
pub struct SearchSearching<F> {
    pub value: String,
    pub fields: Vec<F>,
}

#[derive(Debug, Clone)]
pub struct SearchProjection<F> {
    pub mode: SearchProjectionMode,
    pub fields: Vec<F>,
}

#[derive(Debug, Clone)]
pub enum SearchFilterNode<F> {
    Group {
        combinator: SearchFilterCombinator,
        items: Vec<SearchFilterNode<F>>,
    },
    Condition {
        field: F,
        operator: SearchFilterOperator,
        values: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchFilterCombinator {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchProjectionMode {
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchFilterOperator {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    Contains,
    In,
    Nin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchSortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct SearchSortRule<F> {
    pub field: F,
    pub direction: SearchSortDirection,
}

#[derive(Debug, Clone)]
pub enum SearchPagination {
    SkipLimit { skip: u64, limit: u64 },
    PageSize { page: u64, size: u64 },
    Cursor { cursor: Option<String>, limit: u64 },
}

pub trait SearchField:
    Copy + Clone + Eq + std::hash::Hash + Display + Send + Sync + 'static
{
}

impl<T> SearchField for T where
    T: Copy + Clone + Eq + std::hash::Hash + Display + Send + Sync + 'static
{
}

pub trait SearchableField: SearchField + Sized {
    fn search_fields() -> &'static [Self];
}

impl<F> SearchQuery<F> {
    pub fn new(
        searching: Option<SearchSearching<F>>,
        filtration: Option<SearchFilterNode<F>>,
        sorting: Vec<SearchSortRule<F>>,
        projection: Option<SearchProjection<F>>,
        pagination: SearchPagination,
    ) -> Self {
        Self {
            searching,
            filtration,
            sorting,
            projection,
            pagination,
        }
    }
}

impl<F> SearchSearching<F> {
    pub fn new(value: String, fields: Vec<F>) -> Self {
        Self { value, fields }
    }
}

impl<F> SearchProjection<F> {
    pub fn new(mode: SearchProjectionMode, fields: Vec<F>) -> Self {
        Self { mode, fields }
    }
}

impl<F> SearchSortRule<F> {
    pub fn new(field: F, direction: SearchSortDirection) -> Self {
        Self { field, direction }
    }
}

impl Display for SearchFilterCombinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => f.write_str("and"),
            Self::Or => f.write_str("or"),
        }
    }
}

impl Display for SearchFilterOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Contains => "contains",
            Self::In => "in",
            Self::Nin => "nin",
        })
    }
}

impl Display for SearchSortDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        })
    }
}
