use std::sync::Arc;

use crate::repository::Repository;

pub type RepositoryExtension = Arc<dyn Repository + Sync + Send>;
