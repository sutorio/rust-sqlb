mod utils;

use serial_test::serial;
use sqlb::{Field, HasFields, Raw};
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, TodoPatch};

#[serial]
#[tokio::test]
async fn sb_insert_todo_patch() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title = "test - title 01";
	let patch_data = TodoPatch {
		title: Some(test_title.to_string()),
	};

	// DO insert
	let sb = sqlb::insert().table("todo").data(patch_data.not_none_fields());
	let sb = sb.returning(&["id", "title"]);
	let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;
	assert_eq!(test_title, title);

	// CHECK with select all
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_insert_raw() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURE
	let test_title = "test - title 02";

	// ACTION
	let fields: Vec<Field> = vec![("ctime", Raw("now()")).into(), ("title", test_title.to_string()).into()];

	let sb = sqlb::insert().table("todo").data(fields);
	let sb = sb.returning(&["id", "title", "ctime"]);
	let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;
	assert_eq!(test_title, title);

	// CHECK
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}
