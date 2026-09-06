diesel::table! {
  foo (id) {
    id -> Int8,
    #[max_length = 255]
    name -> Varchar,
    description -> Nullable<Text>,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>,
  }
}
