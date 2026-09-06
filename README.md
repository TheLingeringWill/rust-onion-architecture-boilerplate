# foo-service

Onion-architecture Rust service boilerplate: Axum + `diesel-async` (bb8) + AWS S3/SQS, with one of
everything — one domain entity, one repository, one use case, one gateway, one command bus, one
worker, one controller.

Forked from the conventions of `post-service`. Rename `Foo`/`foo` throughout to start a real
service, then drop the `clippy::disallowed_names` allow at the top of `src/main.rs`.

## Layers

```
presentation → app → domain
         ↘         ↓        ↗
              infra
```

Dependencies point inward only. `domain` has zero dependencies. `infra` implements the ports
declared in `domain` and `app`. `presentation` never imports `infra` except to build the
DI container in `state.rs` and wire `main.rs`.

## Layout

| Layer | What lives there |
|---|---|
| `domain/` | `model/` entities, `repository/` port traits, `param/` query params, `error.rs`, `result.rs` |
| `app/` | `use_case/` orchestration traits, `gateway/` outbound ports, `dto/` input/output pairs, `bus/`, `command/`, `worker.rs` |
| `infra/` | `persistence/pg/` (`schema.rs`, `error.rs`, `model/`, `repository/`), plus `use_case/`, `gateway/`, `bus/`, `worker/`, `config/` |
| `presentation/` | `controller/`, `dto/` request/response pairs, `extractor/`, `state.rs`, `response.rs`, `error.rs` |

## Conventions

- No `mod.rs` — a module is `foo.rs` plus a sibling `foo/` directory.
- Persistence adapters live under `infra/persistence/<backend>/` — Postgres is `persistence/pg/`
  with `schema.rs`, `error.rs`, `model.rs` + `model/`, `repository.rs` + `repository/`. Their types
  carry the backend prefix — `PgFoo`, `PgCreateFoo`, `PgFooRepository` — so they never collide with
  the domain names they map to, and no import alias is needed on either side.
- Async traits use `#[trait_variant::make(Send)]`. Never `async_trait`, never `Box<dyn Trait>` —
  static dispatch through generics only.
- Result aliases are named for their layer: `DomainResult<T>`, `ApplicationResult<T>`,
  `PresentationResult<T>`.
- `Ok(…)`/`Err(…)` are a last resort. Build results with `.map`, `.map_err`, `.and_then` or `?`.
- One line beats several: a method body is a single combinator chain wherever it can be.
- One entity per file; one `<operation>_dto.rs` per operation, holding that operation's pair.
- A repository owns its `Pool<AsyncPgConnection>` handle. Use cases hold repositories, never
  connections.
- Errors flow `DomainError → ApplicationError → PresentationError → JSON`. Raw diesel/bb8/AWS
  errors never escape `infra`.

## Database access

The **repository** owns the pool. A use case holds repositories and nothing else — it never sees a
connection, and no diesel type leaves `infra`.

`self.pool` is diesel-async's own `Pool<AsyncPgConnection>` — there is no wrapper type over it. One
shape covers every body: check a connection out, then query it.

```rust
// one query
let mut conn = self.pool.get().await?;

foo::table
  .find(id)
  .first::<PgFoo>(&mut conn)
  .await
  .map_err(Into::into)
  .map(Into::into)

// several queries on one connection — same shape, nothing extra
let mut conn = self.pool.get().await?;
let foo = foo::table.find(id).first::<PgFoo>(&mut conn).await?;
bar::table.filter(bar::foo_id.eq(foo.id)).get_results::<PgBar>(&mut conn).await
```

`get()` hands back a `PooledConnection`, which returns itself to the pool when it drops at the end of
the method. It goes straight into a DSL call as `&mut conn` — diesel-async blankets `AsyncConnection`
over anything that derefs to one. The connection never reaches a signature, so the port stays clean:

```rust
#[trait_variant::make(Send)]
pub trait FooRepository: Sync + Clone + 'static {
  async fn find_by_id(&self, id: i64) -> DomainResult<Foo>;
  …
}
```

No `type Connection`, no `&mut` parameter, no diesel type anywhere in `domain`. Error mapping lives in
`infra/persistence/pg/error.rs`: `From<diesel::result::Error>` turns a `QueryResult` into a `DomainResult` via
`.map_err(Into::into)`, and `From<RunError>` does the same for a failed checkout — which is what lets
`self.pool.get().await?` sit in a body that returns `DomainResult`.

### Transactions

Nothing here opens one: `create` is a single `INSERT`, which Postgres already runs atomically. When
you need one, bind a connection **inside the repository** and use diesel-async's own method:

```rust
let mut conn = self.pool.get().await?;

conn
  .transaction(async |conn| {
    let foo = diesel::insert_into(foo::table)
      .values(PgCreateFoo::from(foo_to_create))
      .get_result::<PgFoo>(conn)
      .await?;

    diesel::insert_into(bar::table)
      .values(PgBar::for_foo(foo.id))
      .execute(conn)
      .await
      .map(|_| foo.into())
  })
  .await
```

That covers one repository's own queries. **A transaction cannot span two repositories** — each holds
its own pool handle and would check out its own connection, and two connections cannot share a
transaction. That is the price of taking `&mut Self::Connection` off the port, and it is paid
deliberately: the common case is one repository, one query, and it should not carry a connection
parameter it never uses. When a unit of work really is atomic across tables, it is one repository
method, not two.

## Setup

```bash
# Postgres
docker run -d --name foo-db \
  -e POSTGRES_PASSWORD=root -e POSTGRES_USER=foo -e POSTGRES_DB=foo \
  -p 5432:5432 postgres:17

psql postgres://foo:root@localhost:5432/foo -c "
CREATE TABLE foo (
  id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name        VARCHAR(255) NOT NULL,
  description TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ
);"

# S3 + SQS
docker run -d --name foo-aws -p 4566:4566 localstack/localstack

export AWS_ENDPOINT_URL=http://localhost:4566 AWS_ACCESS_KEY_ID=test \
       AWS_SECRET_ACCESS_KEY=test AWS_REGION=eu-west-3
aws --endpoint-url=$AWS_ENDPOINT_URL s3 mb s3://foo-dev-upload
aws --endpoint-url=$AWS_ENDPOINT_URL sqs create-queue --queue-name foo-command

cp .env.example .env   # then set AWS_SQS_QUEUE_URL to the URL just returned
cargo run
```

There is no `diesel.toml` or `migrations/` directory — `src/infra/persistence/pg/schema.rs` is hand-written to
match the `CREATE TABLE` above. Add the diesel CLI back when the schema starts moving.

## Endpoints

| Method | Path | Notes |
|---|---|---|
| `GET` | `/` | health string |
| `POST` | `/foo` | creates a foo, then publishes a `FooCommand` to SQS |
| `GET` | `/foo` | `?limit=&offset=` |
| `GET` | `/foo/{id}` | |
| `GET` | `/docs` | Swagger UI, when `OPENAPI_ENABLED=true` |

Every response is wrapped: `{"message": …, "statusCode": 201}`. Errors use
`{"message": …, "error": …, "statusCode": …}`.

## The async flow

`POST /foo` → `FooUseCase::create` → repository insert → `FooCommandBus.send` publishes JSON to SQS
→ the poll loop in `presentation/utils/worker.rs` long-polls, calls `FooWorker::execute`, which
uploads `foos/{id}.txt` through `AwsMediaGateway` → the message is acked **only on success**, so a
failure leaves it for redelivery after the visibility timeout.

Nothing is swallowed with `let _ = …`: `Worker::execute` returns `ApplicationResult<()>` and the
poll loop is the single place errors stop and get logged.

> The insert commits before the SQS publish, so a publish failure returns 500 with the row already
> written. That is accepted here for simplicity; the real fix is a transactional outbox.

## Commands

```bash
cargo build
cargo clippy --all-targets -- -D warnings
cargo +nightly fmt        # rustfmt.toml uses unstable options
```
