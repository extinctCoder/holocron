use crate::ast::{SelectItem, View};
use crate::catalog::CatalogColumn;
use crate::error::HolocronError;
use crate::resolve::scope::Scope;

/// Resolve a view's select list into catalog columns, flowing each source
/// column's type up to the view column.
pub(crate) fn resolve_columns(
    view: &View,
    scope: &Scope,
) -> Result<Vec<CatalogColumn>, HolocronError> {
    let mut columns = Vec::with_capacity(view.select.len());
    for item in &view.select {
        columns.push(resolve_item(view, item, scope)?);
    }
    Ok(columns)
}

fn resolve_item(
    view: &View,
    item: &SelectItem,
    scope: &Scope,
) -> Result<CatalogColumn, HolocronError> {
    let select = match item {
        SelectItem::Column(select) => select,
        SelectItem::Expression(_) => {
            return Err(HolocronError::unsupported(
                "expression select items are not yet supported",
            ));
        }
    };

    // Resolve the source relation: the named alias, or the sole source if omitted.
    let relation = match &select.from {
        Some(alias) => scope
            .relation(alias)
            .ok_or_else(|| HolocronError::unknown_alias(&view.name, alias))?,
        None => scope
            .sole_relation()
            .ok_or_else(|| HolocronError::ambiguous_source(&view.name, &select.column))?,
    };

    let source = relation
        .column(&select.column)
        .ok_or_else(|| HolocronError::unknown_column(&relation.name, &select.column))?;

    let name = select.r#as.clone().unwrap_or_else(|| select.column.clone());
    Ok(CatalogColumn {
        name,
        data_type: source.data_type.clone(),
        nullable: source.nullable,
        // The view declares the per-column query policy explicitly; defaults
        // come from the AST (filterable=true, searchable=false).
        filterable: select.filterable,
        searchable: select.searchable,
    })
}
