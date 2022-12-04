# GoatDB
## _This is a joke database_

##### Example:
&nbsp;
```rust
use goatdb::GoatDb;
fn main() {
    let mut db = GoatDb::new("example.db");
    db.set("hello", &String::from("world"));
    let value = db.get("hello").unwrap();
    println!("result: {}", value);
}
```

### Output:

```shell
result: "world"
```
