use goatdb::GoatDb;

fn main() {
    let mut db = GoatDb::new("example.json");

    for i in 0..100 {
        let key = format!("key-{}", i);

        db.set(&key, &format!("{}", i));

        let value = db.get(&key).unwrap();

        println!("result: {}", value);
    }
}
