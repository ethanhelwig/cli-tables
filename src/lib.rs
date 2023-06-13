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
pub struct TableError {
    pub message: String,
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for TableError {}

#[derive(Debug)]
pub struct Table {
    table_vec: Vec<Vec<String>>,
    num_records: usize,
    num_fields: usize
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {})", self.table_vec, self.num_records, self.num_fields)
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            table_vec: Vec::new(),
            num_records: 0,
            num_fields: 0
        }
    }

    pub fn num_records(&self) -> usize {
        return self.num_records;
    }

    pub fn num_fields(&self) -> usize {
        return self.num_fields;
    }

    /// ### Description
    /// Pushes a new record to the end of the `Table` object.
    ///
    /// ### Arguments
    /// * `new_record: &Vec<&str>` - An immutable reference 
    /// to string slices that represent the new record.
    ///
    /// ### Errors
    /// Returns an error if the number of fields is invalid.
    ///
    /// ### Example
    /// ```
    /// use cli_tables::Table;
    /// 
    /// let mut table = Table::new();
    ///
    /// let header = vec!["Id", "Title", "Series", "Author"];
    /// table.push_row_str(&header).unwrap();
    /// 
    /// let book = vec!["0", "Sword of Destiny", "The Witcher Series", "Andrzej Sapkowski"];
    /// table.push_row_str(&book).unwrap();
    /// ```
    pub fn push_row_str(&mut self, new_record: &Vec<&str>) -> Result<(), TableError> {
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
            Err(TableError {
                message: msg,
            })
        }
    }

    /// ### Description
    /// Pushes a new record to the end of the `Table` object.
    ///
    /// ### Arguments
    /// * `new_record: &Vec<String>` - An immutable reference
    /// to Strings that represent the new record.
    ///
    /// ### Errors
    /// Returns an error if the number of fields is invalid.
    ///
    /// ### Example
    /// ```
    /// use cli_tables::Table;
    /// 
    /// let mut table = Table::new();
    ///
    /// let header = vec![
    ///     "Id".to_string(), 
    ///     "Title".to_string(), 
    ///     "Series".to_string(), 
    ///     "Author".to_string()
    /// ];
    /// table.push_row_str(&header).unwrap();
    /// 
    /// let book = vec![
    ///     "0".to_string(), 
    ///     "Sword of Destiny".to_string(), 
    ///     "The Witcher Series".to_string(), 
    ///     "Andrzej Sapkowski".to_string()
    /// ];
    /// table.push_row_str(&book).unwrap();
    /// ```
    pub fn push_row_string(&mut self, new_record: &Vec<String>) -> Result<(), TableError> {
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
            Err(TableError {
                message: msg,
            })
        }
    }

    /// ### Description
    /// Retrieves the desired record of the `Table` object and returns it.
    ///
    /// ### Arguments
    /// * `id: usize` - represets the desired record id to be retrieved.
    ///
    /// ### Errors
    /// Returns a `TableError` if no record with the given id is found.
    ///
    /// ### Example
    /// ```
    /// use cli_tables::Table;
    /// 
    /// let mut table = Table::new();
    ///
    /// let header = vec![
    ///     "Id".to_string(), 
    ///     "Title".to_string(), 
    ///     "Series".to_string(), 
    ///     "Author".to_string()
    /// ];
    /// table.push_row_str(&header).unwrap();
    /// 
    /// table.get_row(0).unwrap();
    /// ```
    pub fn get_row(&self, id: usize) -> Result<Vec<String>, TableError> {
        for record in 0..self.num_records {
            if record == id {
                return Ok(self.table_vec[record].clone())
            }
        }
        Err(TableError {
            message: "No record with matching id".to_string()
        })
    }

    /// ### Description
    /// Deletes the desired record from the `Table` object.
    ///
    /// ### Arguments
    /// * `id` - A `usize` value that represets the desired 
    /// record id to be deleted.
    ///
    /// ### Errors
    /// Returns a `TableError` if no record with the given id is found.
    ///
    /// ### Example
    /// ```
    /// use cli_tables::Table;
    /// 
    /// let mut table = Table::new();
    ///
    /// let header = vec![
    ///     "Id".to_string(), 
    ///     "Title".to_string(), 
    ///     "Series".to_string(), 
    ///     "Author".to_string()
    /// ];
    /// table.push_row_str(&header).unwrap();
    /// 
    /// table.delete_record(0).unwrap();
    /// ```
    pub fn delete_record(&mut self, id: usize) -> Result<(), TableError> {
        for record in 0..self.num_records {
            if record == id {
                self.table_vec.remove(record);
                self.num_records -= 1;
                return Ok(())
            }
        }
        Err(TableError {
            message: "No record with matching id".to_string() 
        })
    }

    /// ### Description
    /// The `set` function takes multiple records, and adds each one 
    /// in order to the `Table` object. If the `Table` object is not 
    /// empty, it is reset with the given values.
    ///
    /// ### Arguments
    /// * `new_table` - reference to a `Vec<Vec<&str>>` object that
    /// represents multiple records.
    ///
    /// ### Errors
    /// Returns a `TableError` if the number of fields is not equal
    /// for each record.
    ///
    /// ### Example
    /// ```
    /// use cli_tables::Table;
    /// 
    /// let mut table = Table::new();
    ///
    /// let table_vec = vec![
    ///     vec!["Id", "Title", "Series", "Author"],
    ///     vec!["0", "Sword of Destiny", "The Witcher Series", "Andrzej Sapkowski"],
    ///     vec!["1", "The Last Wish", "The Witcher Series", "Andrzej Sapkowski"]
    /// ];
    /// table.set(&table_vec).unwrap();
    /// ```
    pub fn set(&mut self, new_table: &Vec<Vec<&str>>) -> Result<(), TableError> {
        if !self.table_vec.is_empty() {
            self.table_vec = Vec::new();
        }
        self.num_records = new_table.len();
        self.num_fields = new_table[0].len();
        for record in 0..new_table.len() {
            if self.num_fields == new_table[record].len() {
                self.table_vec.push(Vec::new());
                for field in 0..new_table[record].len() {
                    self.table_vec[record].push(new_table[record][field].to_string());
                }
            } else {
                return Err(TableError {
                    message: "Records have an unequal number of fields".to_string()
                })
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        if self.table_vec.is_empty() {
            return "+----------------+\n| Table is empty |\n+----------------+".to_string();
        }
        
        let mut table_str = String::new();
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