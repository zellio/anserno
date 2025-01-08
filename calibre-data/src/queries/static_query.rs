use futures_util::TryFutureExt;
use sea_orm::{ConnectionTrait, Statement};

use crate::error::{Error, Result};

pub trait StaticQuery {
    const QUERY: &str;

    fn execute(
        conn: &sea_orm::DatabaseConnection,
    ) -> impl ::std::future::Future<Output = Result<sea_orm::ExecResult>> {
        conn.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            Self::QUERY,
        ))
        .map_err(Error::from)
    }
}
