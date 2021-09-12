use super::add_to_where;
use crate::{
	sql_where_items,
	val::{Field, SqlxBindable},
	x_name, OrderItem, SqlBuilder, WhereItem,
};

pub fn select(table: &str) -> SqlSelectBuilder {
	SqlSelectBuilder {
		table: table.to_string(),
		columns: None,
		and_wheres: Vec::new(),
		order_bys: None,
	}
}

pub struct SqlSelectBuilder<'a> {
	table: String,
	columns: Option<Vec<String>>,
	// TODO: needs to support full condition (and/or)
	and_wheres: Vec<WhereItem<'a>>,
	order_bys: Option<Vec<OrderItem>>,
}

impl<'a> SqlSelectBuilder<'a> {
	pub fn columns(mut self, names: &[&str]) -> Self {
		self.columns = Some(names.into_iter().map(|s| s.to_string()).collect());
		self
	}

	pub fn and_where<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, op: &'static str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, op, val);
		self
	}

	pub fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, "=", val);
		self
	}

	pub fn order_bys(mut self, odrs: &[&str]) -> Self {
		self.order_bys = Some(odrs.to_vec().into_iter().map(|o| o.into()).collect());
		self
	}

	pub fn order_by(mut self, odr: &str) -> Self {
		self.order_bys = Some(vec![odr.into()]);
		self
	}
}

impl<'a> SqlBuilder<'a> for SqlSelectBuilder<'a> {
	fn sql(&self) -> String {
		// SELECT name1, name2 FROM table_name WHERE w1 < r1, w2 = r2

		// SQL: SELECT
		let mut sql = String::from("SELECT ");

		// SQL: name1, name2,
		// For now, if no column, will do a "*"
		match &self.columns {
			Some(columns) => {
				let names = columns.iter().map(|c| x_name(c)).collect::<Vec<String>>().join(", ");
				sql.push_str(&format!("{} ", names));
			}
			None => sql.push_str(&format!("{} ", "*")),
		};

		// SQL: FROM table_name
		sql.push_str(&format!("FROM {} ", x_name(&self.table)));

		// SQL: WHERE w1 < $1, ...
		if self.and_wheres.len() > 0 {
			let sql_where = sql_where_items(&self.and_wheres, 1);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		}

		// SQL: ORDER BY
		if let Some(order_bys) = &self.order_bys {
			let sql_order_bys = order_bys.iter().map::<String, _>(|o| o.into()).collect::<Vec<String>>().join(", ");
			sql.push_str(&format!("ORDER BY {} ", sql_order_bys))
		}

		sql
	}

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.and_wheres.iter().map(|wi| &wi.val);
		Box::new(iter)
	}
}
