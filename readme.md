- Create a turso database
- Set the `DB_URL` and `DB_TOKEN` env variables.
- Note that the second statement in the sql query is not valid (it has a trailing comma).
- `cargo run`
- It does not panic, but it should.
- Note the there is no sign of the rouge statement in the batch response. If the `execute_batch` didn't return an `Err`, I would expect to find something here.
- Delte the `select 1;` statement.
- Now it panics.
