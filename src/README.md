## CLI Tables
This is a simple Rust library for generating ASCII tables in a CLI application. The `Table` struct takes in a `Vec<Vec<String>>` of data and generates an ASCII table from it.

### Usage
1. Add `cli-tables` to your `Cargo.toml` file:

```rust
[dependencies]
cli-tables = "0.1.0"
```
2. Import the Table struct:

```rust
use cli_tables::Table;
```

3. Create a `Vec<Vec<String>>` of data that you want to display in a table:

```rust
let table_arr: Vec<Vec<String>> = vec![
    vec!["#".to_string(), "First Name".to_string(), "Last Name".to_string(), "Date of Birth".to_string(), "TV Show".to_string()],
    vec!["0".to_string(), "Pedro".to_string(), "Pascal".to_string(), "1996-07-28".to_string(), "The Last of Us".to_string()],
    vec!["1".to_string(), "Belle".to_string(), "Ramsey".to_string(), "1991-09-17".to_string(), "The Last of Us".to_string()],
    vec!["3".to_string(), "Scott".to_string(), "Shepherd".to_string(), "1990-04-20".to_string(), "The Last of Us".to_string()],
    vec!["4".to_string(), "Nick".to_string(), "Offerman".to_string(), "1970-06-26".to_string(), "The Last of Us".to_string()]
];
```

4. Create a `Table` struct from the data:

```rust
let mut table = Table::new(&data);
```

5. Generate the ASCII table as a string using the `to_string()` method:

```rust
let table_str = table.to_string();
```

6. Print the table string to the console:

```rust
println!("{}", table_str);
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