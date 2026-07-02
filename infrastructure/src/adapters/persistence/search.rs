use application::errors::ServiceError;
use application::search::query::{
    SearchField, SearchFilterNode, SearchFilterOperator, SearchPagination, SearchProjection,
    SearchProjectionMode, SearchQuery, SearchSearching, SearchSortDirection, SearchSortRule,
};
use application::search::result::{SearchPageResult, SearchPaginationResult};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder,
    QuerySelect, Value,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct FieldCapabilities {
    pub searchable: bool,
    pub sortable: bool,
    pub projectable: bool,
    pub filter_ops: &'static [SearchFilterOperator],
}

impl FieldCapabilities {
    pub const fn new(
        searchable: bool,
        sortable: bool,
        projectable: bool,
        filter_ops: &'static [SearchFilterOperator],
    ) -> Self {
        Self {
            searchable,
            sortable,
            projectable,
            filter_ops,
        }
    }

    pub fn supports(&self, operator: SearchFilterOperator) -> bool {
        self.filter_ops.contains(&operator)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScalarKind {
    Uuid,
    Text,
    Bool,
    I64,
    DateTime,
    PgEnum { db_type: &'static str },
}

impl ScalarKind {
    fn supports_contains(self) -> bool {
        matches!(self, Self::Text)
    }
}

#[derive(Debug, Clone)]
pub struct FieldDef<F, C> {
    pub field: F,
    pub column: C,
    pub scalar: ScalarKind,
    pub capabilities: FieldCapabilities,
    pub parse: fn(&str) -> Result<Value, ServiceError>,
}

impl<F, C> FieldDef<F, C> {
    pub const fn new(
        field: F,
        column: C,
        scalar: ScalarKind,
        capabilities: FieldCapabilities,
        parse: fn(&str) -> Result<Value, ServiceError>,
    ) -> Self {
        Self {
            field,
            column,
            scalar,
            capabilities,
            parse,
        }
    }
}

#[derive(Debug)]
pub struct EntitySearchSpec<E, F>
where
    E: EntityTrait,
    F: 'static,
{
    pub fields: &'static [FieldDef<F, E::Column>],
    pub tiebreaker: F,
    pub default_sort: &'static [(F, Order)],
}

impl<E, F> EntitySearchSpec<E, F>
where
    E: EntityTrait,
    F: SearchField + 'static,
{
    pub fn field(&self, field: F) -> Option<&FieldDef<F, E::Column>> {
        self.fields
            .iter()
            .find(|candidate| candidate.field == field)
    }
}

impl<F, C> FieldDef<F, C>
where
    F: SearchField,
    C: ColumnTrait,
{
    fn build_search_condition(&self, value: &str) -> Result<SimpleExpr, ServiceError> {
        if !self.capabilities.searchable || !self.scalar.supports_contains() {
            return Err(ServiceError::Validation(format!(
                "Search field `{}` is not supported for full-text searching.",
                self.field
            )));
        }

        Ok(self.column.contains(value))
    }

    fn build_filter_condition(
        &self,
        operator: SearchFilterOperator,
        values: &[String],
    ) -> Result<SimpleExpr, ServiceError> {
        if !self.capabilities.supports(operator) {
            return Err(ServiceError::Validation(format!(
                "Filter operator `{}` is not supported for field `{}`.",
                operator, self.field
            )));
        }

        let parsed = self.parse_values(values)?;
        let value = parsed
            .first()
            .cloned()
            .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

        Ok(match operator {
            SearchFilterOperator::Gt => self.column.gt(value),
            SearchFilterOperator::Ge => self.column.gte(value),
            SearchFilterOperator::Lt => self.column.lt(value),
            SearchFilterOperator::Le => self.column.lte(value),
            SearchFilterOperator::Eq => self.column.eq(value),
            SearchFilterOperator::Ne => self.column.ne(value),
            SearchFilterOperator::Contains => {
                if !self.scalar.supports_contains() {
                    return Err(ServiceError::Validation(format!(
                        "Contains filter is only supported for text fields. Field `{}` uses {:?}.",
                        self.field, self.scalar
                    )));
                }

                let value = values
                    .first()
                    .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;
                self.column.contains(value)
            }
            SearchFilterOperator::In => self.column.is_in(parsed),
            SearchFilterOperator::Nin => self.column.is_not_in(parsed),
        })
    }

    fn build_cursor_order_condition(
        &self,
        value: &str,
        order: Order,
    ) -> Result<SimpleExpr, ServiceError> {
        let parsed = (self.parse)(value)?;

        Ok(match order {
            Order::Asc => self.column.gt(parsed),
            Order::Desc => self.column.lt(parsed),
            _ => self.column.lt(parsed),
        })
    }

    fn build_cursor_value_condition(&self, value: &str) -> Result<SimpleExpr, ServiceError> {
        Ok(self.column.eq((self.parse)(value)?))
    }

    fn parse_values(&self, values: &[String]) -> Result<Vec<Value>, ServiceError> {
        values.iter().map(|value| (self.parse)(value)).collect()
    }

    fn cursor_value(&self, row: &serde_json::Value) -> String {
        let key = self.field.to_string();
        row.get(&key)
            .and_then(|value| value.as_str().map(|value| value.to_owned()))
            .unwrap_or_default()
    }
}

pub trait SeaOrmSearchSpec {
    type Entity: EntityTrait;
    type Field: SearchField;
    type Result: DeserializeOwned;

    fn spec() -> &'static EntitySearchSpec<Self::Entity, Self::Field>;
}

pub async fn search_with_spec<S>(
    db: &DatabaseConnection,
    query: SearchQuery<S::Field>,
) -> Result<SearchPageResult<S::Result>, ServiceError>
where
    S: SeaOrmSearchSpec,
{
    let spec = S::spec();
    let effective_sorting = normalize_sorting::<S>(spec, &query.sorting)?;
    let page_size = pagination_limit(&query.pagination)?;
    let selected_fields = projection_fields::<S>(spec, &query.projection, &effective_sorting)?;
    let mut select = S::Entity::find().select_only();
    for field in &selected_fields {
        select = select.column_as(field.column, field.field.to_string());
    }

    if let Some(searching) = &query.searching {
        select = select.filter(build_search_condition::<S>(spec, searching)?);
    }

    if let Some(filtration) = &query.filtration {
        select = select.filter(build_filter_condition::<S>(spec, filtration)?);
    }

    if let Some(cursor) = cursor_filter::<S>(spec, &query.pagination, &effective_sorting)? {
        select = select.filter(cursor);
    }

    for (field, order) in &effective_sorting {
        let field_def = spec
            .field(*field)
            .ok_or_else(|| ServiceError::Validation(format!("Unknown sort field `{field}`.")))?;

        select = select.order_by(field_def.column, order.clone());
    }

    let fetch_limit = page_size.saturating_add(1);
    select = select.limit(fetch_limit);
    select = apply_offset(select, &query.pagination);

    let mut rows = select.into_json().all(db).await.map_err(|e| {
        tracing::error!(error = %e, "Failed to search records.");
        ServiceError::internal(e)
    })?;

    let has_more = rows.len() > page_size as usize;
    if has_more {
        rows.pop();
    }

    let next_cursor = if has_more {
        rows.last().map(|row| {
            let values = effective_sorting
                .iter()
                .map(|(field, _)| {
                    spec.field(*field)
                        .expect("validated sort field")
                        .cursor_value(row)
                })
                .collect();

            encode_cursor_token(&effective_sorting, values)
        })
    } else {
        None
    };

    let rows = rows
        .into_iter()
        .map(|row| serde_json::from_value(row).map_err(ServiceError::internal))
        .collect::<Result<Vec<S::Result>, ServiceError>>()?;
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

type EntityFieldDef<E, F> = FieldDef<F, <E as EntityTrait>::Column>;
type ProjectionFields<E, F> = Vec<EntityFieldDef<E, F>>;

fn projection_fields<S: SeaOrmSearchSpec>(
    spec: &EntitySearchSpec<S::Entity, S::Field>,
    projection: &Option<SearchProjection<S::Field>>,
    sorting: &[(S::Field, Order)],
) -> Result<ProjectionFields<S::Entity, S::Field>, ServiceError> {
    let mut fields = Vec::new();

    let requested_fields = match projection {
        Some(projection) => match projection.mode {
            SearchProjectionMode::Show => {
                if projection.fields.is_empty() {
                    spec.fields
                        .iter()
                        .filter(|&field| field.capabilities.projectable)
                        .cloned()
                        .collect::<Vec<_>>()
                } else {
                    projection
                        .fields
                        .iter()
                        .map(|field| {
                            let field_def = spec.field(*field).ok_or_else(|| {
                                ServiceError::Validation(format!(
                                    "Unknown projection field `{field}`."
                                ))
                            })?;
                            if !field_def.capabilities.projectable {
                                return Err(ServiceError::Validation(format!(
                                    "Field `{}` is not projectable.",
                                    field
                                )));
                            }
                            Ok(field_def.clone())
                        })
                        .collect::<Result<Vec<_>, ServiceError>>()?
                }
            }
            SearchProjectionMode::Hide => {
                let hidden = projection
                    .fields
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<_>>();
                spec.fields
                    .iter()
                    .filter(|&field| {
                        field.capabilities.projectable && !hidden.contains(&field.field)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            }
        },
        None => spec
            .fields
            .iter()
            .filter(|&field| field.capabilities.projectable)
            .cloned()
            .collect::<Vec<_>>(),
    };

    for field in requested_fields {
        push_unique(&mut fields, field);
    }

    for (field, _) in sorting {
        let field_def = spec
            .field(*field)
            .ok_or_else(|| ServiceError::Validation(format!("Unknown sort field `{field}`.")))?;
        push_unique(&mut fields, field_def.clone());
    }

    let tiebreaker = spec.field(spec.tiebreaker).ok_or_else(|| {
        ServiceError::Validation(format!("Unknown tiebreaker field `{}`.", spec.tiebreaker))
    })?;
    push_unique(&mut fields, tiebreaker.clone());

    Ok(fields)
}

fn push_unique<F, C>(fields: &mut Vec<FieldDef<F, C>>, field: FieldDef<F, C>)
where
    F: PartialEq,
{
    if !fields.iter().any(|existing| existing.field == field.field) {
        fields.push(field);
    }
}

fn build_search_condition<S: SeaOrmSearchSpec>(
    spec: &EntitySearchSpec<S::Entity, S::Field>,
    searching: &SearchSearching<S::Field>,
) -> Result<Condition, ServiceError> {
    if searching.fields.is_empty() {
        return Err(ServiceError::Validation(
            "Search fields cannot be empty.".to_string(),
        ));
    }

    let mut condition = Condition::any();
    for field in &searching.fields {
        let field_def = spec
            .field(*field)
            .ok_or_else(|| ServiceError::Validation(format!("Unknown search field `{field}`.")))?;
        condition = condition.add(field_def.build_search_condition(&searching.value)?);
    }

    Ok(condition)
}

fn build_filter_condition<S: SeaOrmSearchSpec>(
    spec: &EntitySearchSpec<S::Entity, S::Field>,
    node: &SearchFilterNode<S::Field>,
) -> Result<Condition, ServiceError> {
    match node {
        SearchFilterNode::Group { combinator, items } => {
            let mut condition = match combinator {
                application::search::query::SearchFilterCombinator::And => Condition::all(),
                application::search::query::SearchFilterCombinator::Or => Condition::any(),
            };

            for item in items {
                condition = condition.add(build_filter_condition::<S>(spec, item)?);
            }

            Ok(condition)
        }
        SearchFilterNode::Condition {
            field,
            operator,
            values,
        } => {
            let field_def = spec.field(*field).ok_or_else(|| {
                ServiceError::Validation(format!("Unknown filter field `{field}`."))
            })?;

            Ok(Condition::all().add(field_def.build_filter_condition(*operator, values)?))
        }
    }
}

fn normalize_sorting<S: SeaOrmSearchSpec>(
    spec: &EntitySearchSpec<S::Entity, S::Field>,
    sortings: &[SearchSortRule<S::Field>],
) -> Result<Vec<(S::Field, Order)>, ServiceError> {
    let mut result = if sortings.is_empty() {
        spec.default_sort.to_vec()
    } else {
        sortings
            .iter()
            .map(|rule| -> Result<_, ServiceError> {
                let field_def = spec.field(rule.field).ok_or_else(|| {
                    ServiceError::Validation(format!("Unknown sort field `{}`.", rule.field))
                })?;

                if !field_def.capabilities.sortable {
                    return Err(ServiceError::Validation(format!(
                        "Sort field `{}` is not sortable.",
                        rule.field
                    )));
                }

                Ok((
                    rule.field,
                    match rule.direction {
                        SearchSortDirection::Asc => Order::Asc,
                        SearchSortDirection::Desc => Order::Desc,
                    },
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let tiebreaker = spec.tiebreaker;
    if !result.iter().any(|(field, _)| *field == tiebreaker) {
        let order = result
            .last()
            .map(|(_, order)| order.clone())
            .unwrap_or(Order::Desc);
        result.push((tiebreaker, order));
    }

    Ok(result)
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
    spec: &EntitySearchSpec<S::Entity, S::Field>,
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
            let field_def = spec.field(*prev_field).ok_or_else(|| {
                ServiceError::Validation(format!("Unknown cursor field `{prev_field}`."))
            })?;

            branch = branch.add(field_def.build_cursor_value_condition(&values[prev_index])?);
        }

        let field_def = spec
            .field(*field)
            .ok_or_else(|| ServiceError::Validation(format!("Unknown cursor field `{field}`.")))?;

        branch = branch.add(field_def.build_cursor_order_condition(&values[index], order.clone())?);
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
