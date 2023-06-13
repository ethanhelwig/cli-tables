#[cfg(test)]
mod tests {
    use cli_tables::{Table, TableError};

    #[test]
    fn test_table_creation() {
        let table = Table::new();
        assert_eq!(table.num_records(), 0);
        assert_eq!(table.num_fields(), 0);
        assert_eq!(format!("{}", table), "([], 0, 0)".to_string());
    }

    #[test]
    fn test_push_valid_record() {
        let mut table = Table::new();
        let record = vec![
            "Dr. Gregory House", 
            "Diagnostician", 
            "Princeton-Plainsboro Teaching Hospital"
        ];
        assert_eq!(table.push_row_str(&record), Ok(()));
        assert_eq!(table.num_records(), 1);
        assert_eq!(table.num_fields(), 3);
        assert_eq!(
            format!("{}", table), 
            "([[\"Dr. Gregory House\", \"Diagnostician\", \"Princeton-Plainsboro Teaching Hospital\"]], 1, 3)"
        );
    }

    #[test]
    fn test_push_invalid_record() {
        let mut table = Table::new();
        let record = vec![
            "Dr. Gregory House", 
            "Diagnostician", 
            "Princeton-Plainsboro Teaching Hospital"
        ];
        assert_eq!(table.push_row_str(&record), Ok(()));
        let record = vec![
            "Dr. Lisa Cuddy",
            "Dean of Medicine"
        ];
        assert_eq!(
            table.push_row_str(&record), 
            Err(TableError {
                message: "Invalid number of fields in record. Found 2, but expected 3.".to_string() 
            })
        );
        assert_eq!(table.num_records(), 1);
        assert_eq!(table.num_fields(), 3);
        assert_eq!(
            format!("{}", table), 
            "([[\"Dr. Gregory House\", \"Diagnostician\", \"Princeton-Plainsboro Teaching Hospital\"]], 1, 3)"
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
        let record = vec!["a"; 100];
        for _ in 0..100 {
            assert_eq!(table.push_row_str(&record), Ok(()));
        }
        assert_eq!(table.num_records(), 100);
        assert_eq!(table.num_fields(), 100);
    }

    #[test]
    fn test_push_100_fields() {
        let mut table = Table::new();
        let mut record = Vec::new();
        for _ in 0..100 {
            record.push("a");
        }
        assert_eq!(table.push_row_str(&record), Ok(()));
        assert_eq!(table.num_records(), 1);
        assert_eq!(table.num_fields(), 100);
    }

    #[test]
    fn test_delete_record() {
        let mut table = Table::new();
        let record = vec!["value1", "value2"];
        assert_eq!(table.push_row_str(&record), Ok(()));
        let record = vec!["value3".to_string(), "value4".to_string()];
        assert_eq!(table.push_row_string(&record), Ok(()));
        assert_eq!(table.delete_record(0), Ok(()));
        assert_eq!(table.num_records(), 1);
        let row = table.get_row(0).unwrap();
        assert_eq!(row, record)
    }

    #[test]
    fn test_set_table() {
        let mut table = Table::new();
        let table_str = vec![
            vec!["value1", "value2"],
            vec!["value3", "value4"]
        ];
        let expected_num_fields = 2;
        let expected_num_records = 2;
        assert_eq!(table.set_table_str(&table_str), Ok(()));
        assert_eq!(table.num_fields(), expected_num_fields);
        assert_eq!(table.num_records(), expected_num_records);
    }
}