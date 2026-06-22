use indexmap::IndexMap;

use crate::ast::View;
use crate::catalog::{Catalog, CatalogRelation};
use crate::error::HolocronError;

/// The aliases visible inside one view: each `from`/`join` alias bound to its
/// relation in the catalog. Lexical scope — visible only within the view.
pub(crate) struct Scope<'catalog> {
    aliases: IndexMap<String, &'catalog CatalogRelation>,
}

impl<'catalog> Scope<'catalog> {
    /// Resolve every `from`/`join` source of a view to a catalog relation.
    ///
    /// # Errors
    /// A source referencing an unknown relation, or two sources sharing an alias.
    pub(crate) fn build(view: &View, catalog: &'catalog Catalog) -> Result<Self, HolocronError> {
        let mut aliases = IndexMap::new();
        bind(
            &mut aliases,
            &view.name,
            &view.from.r#as,
            &view.from.table,
            catalog,
        )?;
        for join in &view.join {
            bind(&mut aliases, &view.name, &join.r#as, &join.table, catalog)?;
        }
        Ok(Self { aliases })
    }

    /// The relation an alias is bound to, if declared.
    pub(crate) fn relation(&self, alias: &str) -> Option<&'catalog CatalogRelation> {
        self.aliases.get(alias).copied()
    }

    /// The sole source, when there is exactly one — used to infer an omitted `from`.
    pub(crate) fn sole_relation(&self) -> Option<&'catalog CatalogRelation> {
        if self.aliases.len() == 1 {
            self.aliases.values().next().copied()
        } else {
            None
        }
    }
}

fn bind<'catalog>(
    aliases: &mut IndexMap<String, &'catalog CatalogRelation>,
    view: &str,
    alias: &str,
    relation: &str,
    catalog: &'catalog Catalog,
) -> Result<(), HolocronError> {
    let resolved = catalog
        .relation(relation)
        .ok_or_else(|| HolocronError::unknown_source(view, alias, relation))?;
    if aliases.insert(alias.to_string(), resolved).is_some() {
        return Err(HolocronError::duplicate_alias(view, alias));
    }
    Ok(())
}
