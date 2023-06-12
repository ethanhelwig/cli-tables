## CLI Tables
This is a simple Rust library for generating ASCII tables in a CLI application.

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

3. Create a new `Table`:

```rust
let mut table = Table::new();
```

4. Create the row that you want to display in the table:

```rust
let header = vec!["#", "First Name", "Last Name", "Date of Birth", "TV Show"];
```

5. Add rows with the `push_row` function:

```rust
table.push_row(&header);
table.push_row(&book);
```

6. Print the table with the `to_string` function to the interface:

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
