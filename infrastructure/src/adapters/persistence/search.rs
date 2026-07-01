use application::errors::ServiceError;
use application::search::query::{
    SearchField, SearchFilterNode, SearchFilterOperator, SearchPagination, SearchQuery,
    SearchSearching, SearchSortDirection, SearchSortRule,
};
use application::search::result::{SearchPageResult, SearchPaginationResult};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde::{Deserialize, Serialize};

pub trait SeaOrmSearchSpec {
    type Entity: EntityTrait;
    type Field: SearchField;

    fn default_sorting() -> Vec<(Self::Field, Order)>;

    fn cursor_tiebreaker_field() -> Self::Field;

    fn search_column(
        field: Self::Field,
    ) -> Result<<Self::Entity as EntityTrait>::Column, ServiceError>;

    fn sort_column(field: Self::Field) -> <Self::Entity as EntityTrait>::Column;

    fn filter_condition(
        field: Self::Field,
        operator: SearchFilterOperator,
        values: &[String],
    ) -> Result<SimpleExpr, ServiceError>;

    fn cursor_order_condition(
        field: Self::Field,
        value: &str,
        order: Order,
    ) -> Result<SimpleExpr, ServiceError>;

    fn cursor_value_condition(field: Self::Field, value: &str) -> Result<SimpleExpr, ServiceError>;

    fn cursor_values(
        row: &<Self::Entity as EntityTrait>::Model,
        sorting: &[(Self::Field, Order)],
    ) -> Vec<String>;
}

pub async fn search_with_spec<S>(
    db: &DatabaseConnection,
    query: SearchQuery<S::Field>,
) -> Result<SearchPageResult<<S::Entity as EntityTrait>::Model>, ServiceError>
where
    S: SeaOrmSearchSpec,
{
    let effective_sorting = normalize_sorting::<S>(&query.sorting);
    let page_size = pagination_limit(&query.pagination)?;
    let mut select = S::Entity::find();

    if let Some(searching) = &query.searching {
        select = select.filter(build_search_condition::<S>(searching)?);
    }

    if let Some(filtration) = &query.filtration {
        select = select.filter(build_filter_condition::<S>(filtration)?);
    }

    if let Some(cursor) = cursor_filter::<S>(&query.pagination, &effective_sorting)? {
        select = select.filter(cursor);
    }

    for (field, order) in &effective_sorting {
        select = select.order_by(S::sort_column(*field), order.clone());
    }

    let fetch_limit = page_size.saturating_add(1);
    select = select.limit(fetch_limit);
    select = apply_offset(select, &query.pagination);

    let mut rows = select.all(db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to search records.");
        ServiceError::internal(e)
    })?;

    let has_more = rows.len() > page_size as usize;
    if has_more {
        rows.pop();
    }

    let next_cursor = rows.last().map(|row| {
        encode_cursor_token(
            &effective_sorting,
            S::cursor_values(row, &effective_sorting),
        )
    });
    let skip = match query.pagination {
        SearchPagination::SkipLimit { skip, .. } => Some(skip),
        _ => None,
    };
    let (page, size) = match query.pagination {
        SearchPagination::PageSize { page, size } => (Some(page), Some(size)),
        _ => (None, None),
    };

    Ok(SearchPageResult {
        items: rows,
        pagination: SearchPaginationResult {
            has_more,
            next_cursor,
            skip,
            limit: page_size,
            page,
            size,
        },
    })
}

fn build_search_condition<S: SeaOrmSearchSpec>(
    searching: &SearchSearching<S::Field>,
) -> Result<Condition, ServiceError> {
    if searching.fields.is_empty() {
        return Err(ServiceError::Validation(
            "Search fields cannot be empty.".to_string(),
        ));
    }

    let mut condition = Condition::any();
    for field in &searching.fields {
        condition = condition.add(S::search_column(*field)?.contains(&searching.value));
    }

    Ok(condition)
}

fn build_filter_condition<S: SeaOrmSearchSpec>(
    node: &SearchFilterNode<S::Field>,
) -> Result<Condition, ServiceError> {
    match node {
        SearchFilterNode::Group { combinator, items } => {
            let mut condition = match combinator {
                application::search::query::SearchFilterCombinator::And => Condition::all(),
                application::search::query::SearchFilterCombinator::Or => Condition::any(),
            };

            for item in items {
                condition = condition.add(build_filter_condition::<S>(item)?);
            }

            Ok(condition)
        }
        SearchFilterNode::Condition {
            field,
            operator,
            values,
        } => Ok(Condition::all().add(S::filter_condition(*field, *operator, values)?)),
    }
}

fn normalize_sorting<S: SeaOrmSearchSpec>(
    sortings: &[SearchSortRule<S::Field>],
) -> Vec<(S::Field, Order)> {
    let mut result = if sortings.is_empty() {
        S::default_sorting()
    } else {
        sortings
            .iter()
            .map(|rule| {
                (
                    rule.field,
                    match rule.direction {
                        SearchSortDirection::Asc => Order::Asc,
                        SearchSortDirection::Desc => Order::Desc,
                    },
                )
            })
            .collect()
    };

    let tiebreaker = S::cursor_tiebreaker_field();
    if !result.iter().any(|(field, _)| *field == tiebreaker) {
        let order = result
            .last()
            .map(|(_, order)| order.clone())
            .unwrap_or(Order::Desc);
        result.push((tiebreaker, order));
    }

    result
}

fn pagination_limit(pagination: &SearchPagination) -> Result<u64, ServiceError> {
    let limit = match pagination {
        SearchPagination::SkipLimit { limit, .. } => *limit,
        SearchPagination::PageSize { size, .. } => *size,
        SearchPagination::Cursor { limit, .. } => *limit,
    };

    if limit == 0 {
        return Err(ServiceError::Validation(
            "Pagination limit must be greater than zero.".to_string(),
        ));
    }

    Ok(limit)
}

fn apply_offset<E>(select: sea_orm::Select<E>, pagination: &SearchPagination) -> sea_orm::Select<E>
where
    E: EntityTrait,
{
    match pagination {
        SearchPagination::SkipLimit { skip, .. } => select.offset(*skip),
        SearchPagination::PageSize { page, size } => {
            let offset = page.saturating_sub(1).saturating_mul(*size);
            select.offset(offset)
        }
        SearchPagination::Cursor { .. } => select,
    }
}

fn cursor_filter<S: SeaOrmSearchSpec>(
    pagination: &SearchPagination,
    sorting: &[(S::Field, Order)],
) -> Result<Option<Condition>, ServiceError> {
    let cursor = match pagination {
        SearchPagination::Cursor { cursor, .. } => cursor,
        _ => return Ok(None),
    };

    let Some(cursor) = cursor else {
        return Ok(None);
    };

    let values = parse_cursor(cursor, sorting)?;
    let mut condition = Condition::any();

    for (index, (field, order)) in sorting.iter().enumerate() {
        let mut branch = Condition::all();
        for (prev_index, (prev_field, _)) in sorting.iter().take(index).enumerate() {
            branch = branch.add(S::cursor_value_condition(*prev_field, &values[prev_index])?);
        }

        branch = branch.add(S::cursor_order_condition(
            *field,
            &values[index],
            order.clone(),
        )?);
        condition = condition.add(branch);
    }

    Ok(Some(condition))
}

fn encode_cursor_token<F: SearchField>(sorting: &[(F, Order)], values: Vec<String>) -> String {
    let payload = CursorPayload {
        version: 1,
        sorting: sorting_signature(sorting),
        values,
    };
    let bytes = serde_json::to_vec(&payload).expect("cursor payload serialization should not fail");
    URL_SAFE_NO_PAD.encode(bytes)
}

fn parse_cursor<F: SearchField>(
    cursor: &str,
    sorting: &[(F, Order)],
) -> Result<Vec<String>, ServiceError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| ServiceError::Validation("Invalid cursor value.".to_string()))?;
    let payload: CursorPayload = serde_json::from_slice(&decoded)
        .map_err(|_| ServiceError::Validation("Invalid cursor value.".to_string()))?;

    if payload.version != 1 || payload.sorting != sorting_signature(sorting) {
        return Err(ServiceError::Validation(
            "Invalid cursor value.".to_string(),
        ));
    }

    if payload.values.len() != sorting.len() {
        return Err(ServiceError::Validation(
            "Invalid cursor value.".to_string(),
        ));
    }

    Ok(payload.values)
}

fn sorting_signature<F: SearchField>(sorting: &[(F, Order)]) -> Vec<CursorSortEntry> {
    sorting
        .iter()
        .map(|(field, order)| CursorSortEntry {
            field: field.to_string(),
            direction: match order {
                Order::Asc => "asc".to_string(),
                Order::Desc => "desc".to_string(),
                _ => "desc".to_string(),
            },
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct CursorPayload {
    version: u8,
    sorting: Vec<CursorSortEntry>,
    values: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct CursorSortEntry {
    field: String,
    direction: String,
}
