
**sqlb** is a simple and expressive SQLBuilder for Rust for [sqlx](https://crates.io/crates/sqlx) focused on PostgreSQL for now. 

- **Simple** - Focused on providing an expressive, composable, and reasonably typed scheme to build and execute (via sqlx for now) parameterized SQL statements. The goal is NOT to abstract SQL but to make it expressive and composable using Rust programmatic constructs.
	- **NOT** a database **executor/driver** (Will be using SQLX as sql executore)
	- **NOT** an **ORM**, although eventually, one could build an ORM on top of it. 
- **Expressive** - From arbitrary typed data in and out (list of names/values) to struct and mapping rules. 
- **Focused** 
	- **[sqlx](https://crates.io/crates/sqlx)** - The first "database executor" provided will be [sqlx](https://github.com/launchbadge/sqlx). 
	- **PostgreSQL** - First database support will be Postgres (via sqlx). Additional database support may be added based on interest and pull requests.
- **Prepared Statement ONLY!**	

> NOTE: SQL Builders are typically not used directly by application business logic, but rather to be wrapped in some Application Model Access Layer (e.g., DAOs or MCs - Model Controller -). In fact, even when using ORMs, it is often a good code design to wrap those access via some data access layers. 


Goals for first **0.x.x** releases: 

- **sqlx** - Only plan to be on top of [sqlx](https://crates.io/crates/sqlx).
- **PostgreSQL** - Focus only on the PostgreSQL.
- **Macros** - Adding macros to keep thing DRY (but they are optional, all can be implemented via trait objects)
- **limitations** - The sqlb traits can handle certain types of applications, but the trait model has limitations when it comes to external app types.


## Early API Example (just conceptual for now)

```rust
#[derive(sqlx::FromRow)] // Optional: to be able to use the sqlx_exec::fetch_as...
pub struct Todo {
    pub id: i64,
    pub title: String,
}

#[derive(sqlb::Fields)] // implements sqlb::HasFields for dynamic binding
pub struct TodoPatch {
    pub title: Option<String>,
}

let patch_data = TodoPatch {
	title: Some("Hello Title".to_string())
};

// INSERT - Insert a new Todo from a Partial todo
let sb = sqlb::insert().table("todo").data(patch_data.fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;

// SELECT - Get all todos
let sb = sqlb::select().table("todo").columns(&["id", "title"]).order_by("!id");
let todos: Vec<Todo> = sb.fetch_as_all(&db_pool).await?;
assert_eq!(1, todos.len());
```

## Latest Breaking Changes

- `0.2.0` - Changing the generic order to match `sqlx`. From `.fetch_one::<(i64, String), _>` to `.fetch_one::<_, (i64, String)>`
- `0.0.7` - `sqlb::insert().table("todo")` (in 0.0.7) rather than `sqlb::insert("toto")` (<=0.0.6) (for all SqlBuilders)


## For sqlb Dev

Start a PostgreSQL

```sh
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:14

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 - MUST run with `--test-threads=1` to avoid database access conflicts
cargo test -- --test-threads=1

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert -- --test-threads=1'
```