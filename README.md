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

3. Create a new `Table`:

```rust
let mut table = Table::new();
```

4. Create a row `Vec<&str>` of data that you want to display in the table:

```rust
let header = vec!["Id", "Title", "Series", "Author"]; // vector of string slices
let book = vec!["0", "Sword of Destiny", "The Witcher Series", "Andrzej Sapkowski"];
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
+----+------------------+--------------------+-------------------+----------------------------------------------------------+
| Id | Title            | Series             | Author            | Description                                              |
+----+------------------+--------------------+-------------------+----------------------------------------------------------+
| 0  | Sword of Destiny | The Witcher Series | Andrzej Sapkowski | "The Sword of Destiny" is a collection of short stories  |
|    |                  |                    |                   | that continue the adventures of Geralt of Rivia, a       |
|    |                  |                    |                   | professional monster hunter known as a Witcher. The      |
|    |                  |                    |                   | book explores Geralt's encounters with various creatures | 
|    |                  |                    |                   | and individuals, delving into his moral choices and the  |
|    |                  |                    |                   | consequences they bring. The stories in "The Sword of    |
|    |                  |                    |                   | Destiny" provide further character development for       |
|    |                  |                    |                   | Geralt and introduce important characters like Ciri, a   |
|    |                  |                    |                   | young princess with a significant role in the series.    |
| 1  | The Last Wish    | The Witcher Series | Andrzej Sapkowski | "The Last Wish" is also a collection of short stories    |
|    |                  |                    |                   | featuring Geralt of Rivia. It serves as an introduction  |
|    |                  |                    |                   | to the world and characters of "The Witcher." The book   |
|    |                  |                    |                   | follows Geralt as he takes on contracts to hunt down     |
|    |                  |                    |                   | monsters while navigating political intrigues and moral  |
|    |                  |                    |                   | dilemmas. "The Last Wish" delves into Geralt's origins,  |
|    |                  |                    |                   | his relationships, and his encounters with various       |
|    |                  |                    |                   | mythical creatures. It sets the stage for the ongoing    |
|    |                  |                    |                   | saga of Geralt and his involvement in the complex world  |
|    |                  |                    |                   | of monsters, magic, and politics.                        |
+----+------------------+--------------------+-------------------+----------------------------------------------------------+
```
