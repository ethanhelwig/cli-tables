### CLI Tables
This is a simple Rust library for generating ASCII tables in a CLI application.

### Functions
| Name          | Description                                                 |
|---------------|-------------------------------------------------------------|
| `push`        | pushes a record (or *row*) to the `Table` object.           |
| `delete`      | deletes a record from the `Table` object.        |
| `get`         | gets a record from the `Table` object.           |
| `set`         | sets multiple records as the `Table` object.    |
| `to_string`   | returns a `String` containing the formatted `Table` object. |
| `num_records` | returns the number of records.                              |
| `num_fields`  | returns the number of fields.                               |
### Usage
1. Add `cli-tables` to your `Cargo.toml` file:
```rust
[dependencies]
cli-tables = "0.2.1"
```

2. Import the `Table` struct:
```rust
use cli_tables::Table;
```

3. Create a new `Table`:
```rust
let mut table = Table::new();
```

4. Create a record that you want to display in the table:
```rust
let header = vec!["#", "First Name", "Last Name", "Date of Birth", "TV Show"];
table.push_row(&header);
```

5. Or create multiple records at once:
```rust
let values = vec![
    vec!["0", "Pedro", "Pascal", "1996-07-28", "The Last of Us"],
    vec!["1", "Belle", "Ramsey", "1991-09-17", "The Last of Us"],
    vec!["2", "Scott", "Shepherd", "1990-04-20", "The Last of Us"],
    vec!["3", "Nick", "Offerman", "1970-06-26", "The Last of Us"]
];
table.push_rows(&values);
```

6. Print the table with the `to_string` function:
```rust
println!("{}", table.to_string());
```

7. The table will look like this:
```
+---+------------+-----------+---------------+----------------+
| # | First Name | Last Name | Date of Birth | TV Show        |
+---+------------+-----------+---------------+----------------+
| 0 | Pedro      | Pascal    | 1996-07-28    | The Last of Us |
| 1 | Belle      | Ramsey    | 1991-09-17    | The Last of Us |
| 3 | Scott      | Shepherd  | 1990-04-20    | The Last of Us |
| 4 | Nick       | Offerman  | 1970-06-26    | The Last of Us |
+---+------------+-----------+---------------+----------------+
```
