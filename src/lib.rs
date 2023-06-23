pub fn table_from(table_arr: &Vec<Vec<&str>>) -> String {
    if table_arr.is_empty() {
        return String::from("+----------------+\n| Table is empty |\n+----------------+");
    }
    else if table_arr[0].is_empty() {
        return String::from("+----------------+\n| Table is empty |\n+----------------+");
    }
    
    let mut table_str = String::new();
    let num_rows = table_arr.len();
    let num_columns = table_arr[0].len();
    let mut table_width = 0;
    let mut column_width = vec![0; num_columns];
    let mut column_length = vec![
        vec![0; num_columns];
        num_rows
    ];
    let terminal_width;
    match terminal_size::terminal_size() {
        Some((terminal_size::Width(width), _)) => {
            terminal_width = width.into()
        }
        None => {
            panic!("failed to retrieve terminal size")
        }
    }

    // determine the column length and width for each row
    for row in 0..num_rows {
        if num_columns != table_arr[row].len() {
            panic!("Inconsistent number of columns in each row")
        }
        if row == 0 {
            for col in 0..num_columns {
                column_length[row][col] = table_arr[row][col].len();
                column_width[col] = column_length[row][col];
            }
        } else {
            for col in 0..num_columns {
                column_length[row][col] = table_arr[row][col].len();
                // otherwise, compare widths to find max
                column_width[col] = std::cmp::max(
                    column_width[col],
                    column_length[row][col]
                )
            }
        }
    }
    table_width += column_width.iter().sum::<usize>();

    // account for formatting for the table width
    let padding_length: usize = 2;
    let num_borders = num_columns + 1;
    let num_spaces = num_columns * padding_length;
    table_width += num_borders + num_spaces;

    // determine field widths
    while table_width > terminal_width {
        let mut max_column_width = 0;
        let mut widest_column = 0;
        for col in 0..num_columns {
            if column_width[col] > max_column_width {
                widest_column = col;
                max_column_width = column_width[col];
            }
        }
        column_width[widest_column] -= 1;
        table_width -= 1;
    }

    // determine row heights for text-wrapping
    let mut row_height = Vec::new();
    let mut is_wrapped = vec![
        vec![false; num_columns]; 
        num_rows
    ];
    for row in 0..num_rows {
        let mut max_row_height = 0;
        for col in 0..num_columns {
            if column_length[row][col] % column_width[col] != 0 {
                max_row_height = std::cmp::max(
                    max_row_height,
                    (column_length[row][col] / column_width[col]) + 1
                );
            }
            else {
                max_row_height = std::cmp::max(
                    max_row_height,
                    column_length[row][col] / column_width[col]
                );
            }
            is_wrapped[row][col] = column_length[row][col] > column_width[col];
        }
        row_height.push(max_row_height);
    }

    // table characters
    let newline = '\n';
    let border = '|';
    let padding = ' ';
    let edge = '+';
    let line = '-';
    
    // add top border
    table_str.push(edge);
    for col in 0..num_columns {
        for _ in 0..column_width[col] + padding_length {
            table_str.push(line);
        }
        table_str.push(edge);
    }
    table_str.push(newline);

    // add column headers
    if row_height[0] > 1 {
        let mut remaining = vec![""; num_columns];
        for line in 0..row_height[0] {
            table_str.push(border);
            table_str.push(padding);
            for col in 0..num_columns {
                if is_wrapped[0][col] {
                    let slice; 
                    let mut slice_length = column_width[col];
                    if line == 0 {
                        slice = &table_arr[0][col][..column_width[col]];
                        remaining[col] = &table_arr[0][col][column_width[col]..];
                    } else {
                        slice_length = std::cmp::min(
                            slice_length,
                            remaining[col].len()
                        );
                        slice = &remaining[col][..slice_length];
                        remaining[col] = &remaining[col][slice_length..];
                    }
                    table_str.push_str(slice);
                    if slice_length < column_width[col] {
                        for _ in slice_length..column_width[col] {
                            table_str.push(padding);
                        }
                    } 
                } else {
                    if line == 0 {
                        table_str.push_str(&table_arr[0][col]);
                        // add padding
                        for _ in column_length[0][col]..column_width[col] {
                            table_str.push(padding);
                        }
                    } else {
                        // add padding
                        for _ in 0..column_width[col] {
                            table_str.push(padding);
                        }
                    }
                }
                // add separators
                if col != num_columns - 1 {
                    table_str.push(padding);
                    table_str.push(border);
                    table_str.push(padding);
                }
            }
            table_str.push(padding);
            table_str.push(border);
            table_str.push(newline);
        }
    } else {
        table_str.push(border);
        table_str.push(padding);
        for col in 0..num_columns {
            table_str.push_str(&table_arr[0][col]);
            for _ in column_length[0][col]..column_width[col] {
                table_str.push(padding);
            }
            // add separators
            if col != num_columns - 1 {
                table_str.push(padding);
                table_str.push(border);
                table_str.push(padding);
            }
        }
        table_str.push(padding);
        table_str.push(border);
        table_str.push(newline);
    }

    // add middle line
    table_str.push(edge);
    for col in 0..num_columns {
        for _ in 0..column_width[col] + padding_length {
            table_str.push(line);
        }
        table_str.push(edge);
    }
    table_str.push(newline);

    // add values
    for row in 1..num_rows {
        if row_height[row] > 1 {
            let mut remaining = vec![""; num_columns];
            for line in 0..row_height[row] {
                table_str.push(border);
                table_str.push(padding);
                for col in 0..num_columns {
                    // add truncated value and store remaining
                    if is_wrapped[row][col] {
                        let slice;
                        let mut slice_length = column_width[col];
                        if line == 0 {
                            slice = &table_arr[row][col][..column_width[col]];
                            remaining[col] = &table_arr[row][col][column_width[col]..];
                        } else {
                            slice_length = std::cmp::min(
                                slice_length, 
                                remaining[col].len()
                            );
                            slice = &remaining[col][..slice_length];
                            remaining[col] = &remaining[col][slice_length..];
                        }
                        table_str.push_str(slice);
                        if slice_length < column_width[col] {
                            for _ in slice_length..column_width[col] {
                                table_str.push(padding);
                            }
                        }
                    } else {
                        if line == 0 {
                            table_str.push_str(&table_arr[row][col]);
                            // add padding
                            for _ in column_length[row][col]..column_width[col] {
                                table_str.push(padding);
                            }
                        } else {
                            // add padding
                            for _ in 0..column_width[col] {
                                table_str.push(padding);
                            }
                        }
                    }
                    // add separators
                    if col != num_columns - 1 {
                        table_str.push(padding);
                        table_str.push(border);
                        table_str.push(padding);
                    }
                }
                table_str.push(padding);
                table_str.push(border);
                table_str.push(newline);
            }
        } else {
            table_str.push(border);
            table_str.push(padding);
            for col in 0..num_columns {
                // add value
                table_str.push_str(&table_arr[row][col]);
                // add padding
                for _ in column_length[row][col]..column_width[col] {
                    table_str.push(padding);
                }
                // add separators
                if col != num_columns - 1 {
                    table_str.push(padding);
                    table_str.push(border);
                    table_str.push(padding);
                }
            }
            table_str.push(padding);
            table_str.push(border);
            table_str.push(newline);
        }
    }

    // add bottom border
    table_str.push(edge);
    for col in 0..num_columns {
        for _ in 0..column_width[col] + padding_length {
            table_str.push(line);
        }
        table_str.push(edge);
    }

    table_str
}