pub struct Table<'a> {
    str_arr: &'a Vec<Vec<String>>,
    num_columns: usize,
    num_rows: usize,
    column_width: Vec<usize>,
    num_chars: usize
}

impl<'a> Table<'a> {
    pub fn new(str_arr: &'a Vec<Vec<String>>) -> Table<'a> {
        let num_rows: usize = str_arr.len();
        let num_columns: usize = str_arr[0].len();
        let mut column_width: Vec<usize> = vec![0; num_columns];
        let mut table_width: usize = 0;
        let padding_len: usize = 2;
        let num_lines: usize = 3;

        for col in 0..num_columns {
            // column width is longest string in that column
            for row in 0..num_rows {
                let str_len: usize = str_arr[row][col].len();
                column_width[col] = std::cmp::max(column_width[col], str_len);
            }
            table_width += column_width[col];
        }
        // account for table formatting characters
        table_width += num_columns + 1; // borders
        table_width += padding_len * num_columns; // padding
        table_width += 1; // newlines
        let num_chars: usize = table_width * (num_rows + num_lines);

        Table {
            str_arr,
            num_rows,
            num_columns,
            column_width,
            num_chars
        }
    }

    pub fn to_string(&mut self) -> String {
        let padding_len: usize = 3;
        let border: &str = "|";
        let edge: &str = "+";
        let line: &str = "-";
        let space: &str = " ";
        let newline: &str = "\n";
        let mut table_str: String = String::with_capacity(self.num_chars);


        // top border
        table_str += edge;
        for col in 0..self.num_columns {
            for _ in 0..self.column_width[col] + padding_len - 1 {
                table_str += line;
            }
            table_str += edge;
        }
        table_str += newline;

        // column headers
        table_str += border;
        table_str += space;
        for col in 0..self.num_columns {
            table_str += &self.str_arr[0][col];
            let str_len: usize = self.str_arr[0][col].len();
            for _ in str_len..self.column_width[col] {
                table_str += space;
            }
            // add separators
            if col != self.num_columns - 1 {
                table_str += space;
                table_str += border;
                table_str += space;
            }
        }
        table_str += space;
        table_str += border;
        table_str += newline;

        // middle line
        table_str += edge;
        for col in 0..self.num_columns {
            for _ in 0..self.column_width[col] + padding_len - 1 {
                table_str += line;
            }
            table_str += edge;
        }

        // values
        table_str += newline;
        for row in 1..self.num_rows {
            table_str += border;
            table_str += space;
            for col in 0..self.num_columns {
                table_str += &self.str_arr[row][col];
                // add padding
                let str_len: usize = self.str_arr[row][col].len();
                for _ in str_len..self.column_width[col] {
                    table_str += space;
                }
                // add seperator
                if col != self.num_columns - 1 {
                    table_str += space;
                    table_str += border;
                    table_str += space;
                }
            }
            table_str += space;
            table_str += border;
            table_str += newline;
        }

        // bottom border
        table_str += edge;
        for col in 0..self.num_columns {
            for _ in 0..self.column_width[col] + padding_len - 1 {
                table_str += line;
            }
            table_str += edge;
        }

        table_str
    }
}