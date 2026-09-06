use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Foo {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct ListFoo(pub Vec<Foo>);

#[derive(Debug)]
pub struct CreateFoo {
  pub name: String,
  pub description: Option<String>,
}
