////////////////////////////////////
// sqlx-exec - module for the sqlx query executor
////

use crate::SqlBuilder;
use sqlx::{sqlite::SqliteArguments, Execute, Executor, FromRow, Sqlite};

/// Build a sqlx::query_as for the D (Data) generic type, binds the values, and does a .fetch_one and returns E
pub async fn fetch_as_one<'e, 'q, DB, D, Q>(db_pool: DB, sb: &'q Q) -> Result<D, sqlx::Error>
where
	DB: Executor<'e, Database = Sqlite>,
	D: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Unpin + Send,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();

	// build temp query for binding
	let mut query = sqlx::query::<sqlx::Sqlite>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	// create the QueryAs
	let query = sqlx::query_as_with::<sqlx::Sqlite, D, SqliteArguments>(&sql, query.take_arguments().unwrap());

	// exec and return
	let r = query.fetch_one(db_pool).await?;
	Ok(r)
}

/// Build a sqlx::query_as for the D (Data) generic type, binds the values, and does a .fetch_one and returns E
pub async fn fetch_as_optional<'e, 'q, DB, D, Q>(db_pool: DB, sb: &'q Q) -> Result<Option<D>, sqlx::Error>
where
	DB: Executor<'e, Database = Sqlite>,
	D: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Unpin + Send,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();

	// build temp query for binding
	let mut query = sqlx::query::<sqlx::Sqlite>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	// create the QueryAs
	let query = sqlx::query_as_with::<sqlx::Sqlite, D, SqliteArguments>(&sql, query.take_arguments().unwrap());

	// exec and return
	let r = query.fetch_optional(db_pool).await?;
	Ok(r)
}

/// Build a sqlx::query_as for the D (Data) generic type, binds the values, and does a .fetch_all and returns Vec<E>
pub async fn fetch_as_all<'e, 'q, DB, D, Q>(db_pool: DB, sb: &'q Q) -> Result<Vec<D>, sqlx::Error>
where
	DB: Executor<'e, Database = Sqlite>,
	D: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Unpin + Send,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();

	// build temp query for binding
	let mut query = sqlx::query::<sqlx::Sqlite>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	// create the QueryAs
	let query = sqlx::query_as_with::<sqlx::Sqlite, D, SqliteArguments>(&sql, query.take_arguments().unwrap());

	// exec and return
	let r = query.fetch_all(db_pool).await?;
	Ok(r)
}

pub async fn exec<'e, 'q, DB, Q>(db_pool: DB, sb: &'q Q) -> Result<u64, sqlx::Error>
where
	DB: Executor<'e, Database = Sqlite>,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let mut query = sqlx::query::<sqlx::Sqlite>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	let r = query.execute(db_pool).await?.rows_affected();

	Ok(r)
}
