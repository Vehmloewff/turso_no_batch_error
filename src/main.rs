use libsql::Builder;
use std::env::var;

#[tokio::main]
async fn main() {
    let connection = Builder::new_remote(var("DB_URL").unwrap(), var("DB_TOKEN").unwrap())
        .build()
        .await
        .unwrap()
        .connect()
        .unwrap();

    let mut batch = connection
        .execute_batch(
            "
                select 1;

                CREATE TABLE foo (
                    bar text,
                );
            ",
        )
        .await
        .unwrap();

    loop {
        match batch.next_stmt_row() {
            Some(Some(mut rows)) => loop {
                match rows.next().await.unwrap() {
                    Some(row) => dbg!(row),
                    None => break,
                };
            },
            _ => break,
        }
    }
}
