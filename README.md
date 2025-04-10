# esc

esc (embedded SQL compiler) is a way to apply SQL migrations and statements such that statements are type-checked against the final schema (based on the migrations) and for any arguments and operations. Using a DSL, you define `stmt`s such that you can have provable static guarantees about the whether the statement is safe or not.

```
// PostGIS Extension Types
type Point;
type Polygon;
type Geometry = {Point, Polygon};
type ST_MakePoint(Float, Float): Point;
type ST_SetSRID(Geometry, Integer): Point;

stmt get_users(
  created_between: Option<Tuple<Integer, Integer>>,
  limit: Option<Integer>
): Array<Row> {
  `SELECT * FROM users`
  if created_between {
    `WHERE created_at BETWEEN {created_between[0]} AND {created_between[1]}`
  }
  if limit {
    `LIMIT {limit}`
  }
}

stmt bulk_add_users(usernames: Array<String>): Array<Row> {
  `INSERT INTO users (name) VALUES`
  for un in usernames; `,` {
    `({un})`
  }
  `RETURNING *`
}
```

## Roadmap

- [x] Compute final schema based on migrations
- [] Type-check statements against final schema
- [] Type-check statements for aliasing
- [] Type-check statements with operations
  - [] `AND`
  - [] `OR`
  - [] [TBD](https://www.postgresql.org/docs/9.0/functions.html)

## Getting Started

The entrypoint is a CLI that accepts the migrations directory and statements directory. Note that the migrations are processed in lexicographical order.

```sh
cargon run -- <migrations_dir_path> <statements_dir_path>
```

## Examples

These can be run in the esc directory with `cargo run --example <example_name>`.
