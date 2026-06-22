//! The resolve phase: turn parsed views into resolved catalog relations.

mod scope;
mod select;
mod views;

pub use views::resolve_views;
