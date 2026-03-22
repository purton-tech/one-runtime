# Database

Schema, migrations, and typed query workflow.

## Structure

- `migrations/` schema migrations
- `queries/` SQL files used by Clorinde
- `src/` crate entrypoints and re-exports

## Workflow

- Create migrations with `./scripts/dbmate new migration-name`.
- Apply and inspect migrations with `./scripts/dbmate ...`.
- Use `./scripts/psql ...` for direct database access.
- Regenerate typed query code with `./scripts/clorinde` after changing SQL in `queries/`.

## Core Rules

- Schema changes in `migrations/` must be reflected in `queries/`.
- Do not edit `crates/db-gen`; it is generated code.
- Never embed raw SQL in Rust. Put it in `crates/db/queries/*.sql` and call it through Clorinde-generated functions.
- Do not use `SELECT *`; always list columns explicitly.
- Group SQL files by aggregate or root entity, such as `users.sql` or `integrations.sql`.

## Migration Rules

- Use `dbmate new` so migration timestamps stay correct.
- Keep migrations in `migrations/`.
- When adding a new enum value with `ALTER TYPE ... ADD VALUE`, do not use that new value in the same migration transaction. Put dependent inserts or updates in a follow-up migration.

## Clorinde Rules

- After modifying any SQL file in `crates/db/queries`, run `./scripts/clorinde`.
- Add `--: StructName` before queries that define return types.
- Use `--! query_name` to name queries.
- Parameters are inferred automatically; do not declare them manually.
- Use `field_name?` for nullable fields when needed.
- For dynamic intervals, use `($1 || ' days')::INTERVAL`.
