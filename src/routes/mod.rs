use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod index;
pub mod query;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/query"]
    Query,
    #[to = "/"]
    Index,
}
