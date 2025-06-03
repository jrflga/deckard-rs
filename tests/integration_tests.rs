use postgres::{Client, NoTls};

fn url(num: &str) -> Option<String> {
    std::env::var(format!("DATABASE_URL{}", num)).ok()
}

#[test]
fn copy_row_between_dbs() {
    let src_url = match url("1") {
        Some(u) => u,
        None => {
            eprintln!("DATABASE_URL1 not set, skipping test");
            return;
        }
    };
    let dst_url = match url("2") {
        Some(u) => u,
        None => {
            eprintln!("DATABASE_URL2 not set, skipping test");
            return;
        }
    };

    let mut src = Client::connect(&src_url, NoTls).unwrap();
    let mut dst = Client::connect(&dst_url, NoTls).unwrap();

    src.batch_execute(
        "DROP TABLE IF EXISTS items; CREATE TABLE items (id SERIAL PRIMARY KEY, name TEXT); INSERT INTO items (name) VALUES ('foo');",
    )
    .unwrap();
    dst.batch_execute(
        "DROP TABLE IF EXISTS items; CREATE TABLE items (id INT PRIMARY KEY, name TEXT);",
    )
    .unwrap();

    for row in src.query("SELECT id, name FROM items", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        dst.execute(
            "INSERT INTO items (id, name) VALUES ($1, $2)",
            &[&id, &name],
        )
        .unwrap();
    }

    let count: i64 = dst
        .query_one("SELECT COUNT(*) FROM items", &[])
        .unwrap()
        .get(0);
    assert_eq!(count, 1);
}
