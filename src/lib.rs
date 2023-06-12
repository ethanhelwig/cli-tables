use std::{
    fmt,
    error,
    cmp::{
        max,
        min
    }
};
use terminal_size::{
    Width, 
    terminal_size
};

#[derive(Debug, PartialEq)]
pub struct PushError {
    pub message: String,
}

impl fmt::Display for PushError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for PushError {}

#[derive(Debug)]
pub struct Table {
    table_vec: Vec<Vec<String>>,
    num_records: usize,
    num_fields: usize
}

impl Table {
    pub fn new() -> Self {
        Table {
            table_vec: Vec::new(),
            num_records: 0,
            num_fields: 0
        }
    }

    /// ### Description
    /// Pushes a new row/record to the table.
    ///
    /// ### Arguments
    ///
    /// * `new_record` - An immutable reference to a vector of string slices representing the new record.
    /// In Rust, the &str type represents a string slice. It is an immutable reference to a string data 
    /// stored elsewhere, typically UTF-8 encoded. String slices are commonly used to efficiently work 
    /// with string data without taking ownership of it.
    ///
    /// ### Errors
    ///
    /// Returns an error if the number of fields is invalid.
    ///
    /// ### Examples
    /// ```
    /// use cli_tables::{Table, PushError};
    /// 
    /// let mut table = Table::new(); // creates a new table
    ///
    /// let header = vec!["Id", "Title", "Series", "Author"]; // add rows using a vector of string slices
    /// assert_eq!(table.push_row(&header), Ok(())); // handle errors
    /// 
    /// let book = vec!["0", "Sword of Destiny", "The Witcher Series", "Andrzej Sapkowski"];
    /// assert_eq!(table.push_row(&book), Ok(()));
    ///
    /// let invalid_record = vec!["1", "The Last Wish", "The Witcher Series"];
    /// assert_eq!(
    ///     table.push_row(&invalid_record),
    ///     Err(PushError {
    ///         message: "Invalid number of fields in record. Found 3, but expected 4.".to_string()
    ///     })
    /// );
    /// ```
    pub fn push_row(&mut self, new_record: &Vec<&str>) -> Result<(), PushError> {
        let new_record: Vec<String> = new_record.iter().map(|&field| field.to_string()).collect();
        let num_fields = new_record.len();

        if num_fields == self.num_fields && self.num_records != 0 {
            self.table_vec.push(new_record.to_vec());
            self.num_records += 1;
            Ok(())
        }
        else if self.num_records == 0 {
            self.table_vec.push(new_record.to_vec());
            self.num_records = 1;
            self.num_fields = num_fields;
            Ok(())
        }
        else {
            let msg = format!("Invalid number of fields in record. Found {}, but expected {}.", num_fields, self.num_fields);
            Err(PushError {
                message: msg,
            })
        }
    }

    pub fn to_string(&self) -> String {
        if self.table_vec.is_empty() {
            return "+----------------+\n| Table is empty |\n+----------------+".to_string();
        }
        
        let mut table_str = String::new();
        //let num_lines = self.num_records;
        //let mut field_length = Vec::new();
        let mut field_width = vec![0; self.num_fields];
        let mut field_length = vec![
            vec![0; self.num_fields]; 
            self.num_records
        ];
        let mut terminal_width = 0;
        if let Some((Width(width), _)) = terminal_size() {
            terminal_width = width.into();
        }
        let mut table_width = 0;
        let padding_length: usize = 2;

        // table characters
        let newline = '\n';
        let border = '|';
        let padding = ' ';
        let edge = '+';
        let line = '-';

        // determine field width for each field
        for record in 0..self.num_records {
            // for first record, set width
            if record == 0 {
                for field in 0..self.num_fields {
                    field_length[record][field] = self.table_vec[record][field].len();
                    field_width[field] = field_length[record][field];
                }
            } else {
                for field in 0..self.num_fields {
                    field_length[record][field] = self.table_vec[record][field].len();
                    // otherwise, compare widths to find max
                    field_width[field] = max(
                        field_width[field],
                        field_length[record][field]
                    )
                }
            }
        }
        table_width += field_width.iter().sum::<usize>();

        // account for formatting for the table width
        let num_borders = self.num_fields + 1;
        let num_spaces = self.num_fields * 2;
        table_width += num_borders + num_spaces;

        // determine field widths
        while table_width > terminal_width {
            let mut max_field_width = 0;
            let mut widest_field = 0;
            for field in 0..self.num_fields {
                if field_width[field] > max_field_width {
                    widest_field = field;
                    max_field_width = field_width[field];
                }
            }
            field_width[widest_field] -= 1;
            table_width -= 1;
        }

        // determine record heights
        let mut record_height = Vec::new();
        let mut is_wrapped = vec![
            vec![false; self.num_fields]; 
            self.num_records
        ];
        for record in 0..self.num_records {
            let mut max_record_height = 0;
            for field in 0..self.num_fields {
                if field_length[record][field] % field_width[field] != 0 {
                    max_record_height = max(
                        max_record_height,
                        (field_length[record][field] / field_width[field]) + 1
                    );
                }
                else {
                    max_record_height = max(
                        max_record_height,
                        field_length[record][field] / field_width[field]
                    );
                }
                is_wrapped[record][field] = field_length[record][field] > field_width[field];
            }
            record_height.push(max_record_height);
        }
        
        // add top border
        table_str.push(edge);
        for field in 0..self.num_fields {
            for _ in 0..field_width[field] + padding_length {
                table_str.push(line);
            }
            table_str.push(edge);
        }
        table_str.push(newline);

        // add column headers
        if record_height[0] > 1 {
            let mut remaining = "";
            for line in 0..record_height[0] {
                table_str.push(border);
                table_str.push(padding);
                for field in 0..self.num_fields {
                    if is_wrapped[0][field] {
                        let slice; // check other comment
                        if line == 0 {
                            slice = &self.table_vec[0][field][..field_width[field]];
                            remaining = &self.table_vec[0][field][field_width[field]..];
                        } else {
                            slice = &remaining[..field_width[field]];
                            remaining = &remaining[field_width[field]..];
                        }
                        table_str.push_str(slice);
                    }
                }
                table_str.push(padding);
                table_str.push(border);
                table_str.push(newline);
            }
        } else {
            table_str.push(border);
            table_str.push(padding);
            for field in 0..self.num_fields {
                table_str.push_str(&self.table_vec[0][field]);
                for _ in field_length[0][field]..field_width[field] {
                    table_str.push(padding);
                }
                // add separators
                if field != self.num_fields - 1 {
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
        for field in 0..self.num_fields {
            for _ in 0..field_width[field] + padding_length {
                table_str.push(line);
            }
            table_str.push(edge);
        }
        table_str.push(newline);

        // add values
        for record in 1..self.num_records {
            let mut remaining = "";
            if record_height[record] > 1 {
                for line in 0..record_height[record] {
                    table_str.push(border);
                    table_str.push(padding);
                    for field in 0..self.num_fields {
                        // add truncated value and store remaining
                        if is_wrapped[record][field] {
                            let slice; // removed "" assignment and make not mutable
                            let mut slice_length = field_width[field];
                            if line == 0 {
                                slice = &self.table_vec[record][field][..field_width[field]];
                                remaining = &self.table_vec[record][field][field_width[field]..];
                            } else {
                                slice_length = min(field_width[field], remaining.len());
                                slice = &remaining[..slice_length];
                                remaining = &remaining[slice_length..];
                            }
                            table_str.push_str(slice);
                            if slice_length < field_width[field] {
                                for _ in slice_length..field_width[field] {
                                    table_str.push(padding);
                                }
                            }
                        } else {
                            if line == 0 {
                                table_str.push_str(&self.table_vec[record][field]);
                                // add padding
                                for _ in field_length[record][field]..field_width[field] {
                                    table_str.push(padding);
                                }
                            } else {
                                // add padding
                                for _ in 0..field_width[field] {
                                    table_str.push(padding);
                                }
                            }
                        }
                        // add separators
                        if field != self.num_fields - 1 {
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
                for field in 0..self.num_fields {
                    // add value
                    table_str.push_str(&self.table_vec[record][field]);
                    // add padding
                    for _ in field_length[record][field]..field_width[field] {
                        table_str.push(padding);
                    }
                    // add separators
                    if field != self.num_fields - 1 {
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
        for field in 0..self.num_fields {
            for _ in 0..field_width[field] + padding_length {
                table_str.push(line);
            }
            table_str.push(edge);
        }

        table_str
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {})", self.table_vec, self.num_records, self.num_fields)
    }
}

#[cfg(test)]
mod tests {
    use super::{Table, PushError};

    #[test]
    fn test_table_creation() {
        let table = Table::new();
        assert_eq!(table.num_records, 0);
        assert_eq!(table.num_fields, 0);
        assert_eq!(table.table_vec.len(), 0);
    }

    #[test]
    fn test_push_valid_record() {
        let mut table = Table::new();
        let record = vec!["Value1", "Value2"];
        assert_eq!(table.push_row(&record), Ok(()));
        assert_eq!(table.num_records, 1);
        assert_eq!(table.num_fields, 2);
        assert_eq!(table.table_vec.len(), 1);
        assert_eq!(table.table_vec[0][0], "Value1");
        assert_eq!(table.table_vec[0][1], "Value2");
    }

    #[test]
    fn test_push_invalid_record() {
        let mut table = Table::new();
        let record1 = vec!["Value1", "Value2"];
        let record2 = vec!["Value1"];
        assert_eq!(table.push_row(&record1), Ok(()));
        assert_eq!(
            table.push_row(&record2),
            Err(PushError {
                message: "Invalid number of fields in record. Found 1, but expected 2.".to_string(),
            })
        );
    }

    #[test]
    fn test_to_string_empty_table() {
        let table = Table::new();
        let expected = "+----------------+\n| Table is empty |\n+----------------+";
        assert_eq!(table.to_string(), expected);
    }

    #[test]
    fn test_push_100_records() {
        let mut table = Table::new();
        let record = vec!["value"; 100];
        for _ in 0..100 {
            assert_eq!(table.push_row(&record), Ok(()));
        }
        assert_eq!(table.num_records, 100);
        assert_eq!(table.num_fields, 100);
        assert_eq!(table.table_vec.len(), 100);
    }

    #[test]
    fn test_push_100_fields() {
        let mut table = Table::new();
        let mut record = Vec::new();
        for _ in 0..100 {
            record.push("value");
        }
        assert_eq!(table.push_row(&record), Ok(()));
        assert_eq!(table.num_records, 1);
        assert_eq!(table.num_fields, 100);
        assert_eq!(table.table_vec.len(), 1);
        assert_eq!(table.table_vec[0].len(), 100);
    }
}