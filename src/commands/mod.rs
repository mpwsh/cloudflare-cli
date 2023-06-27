use tabular::{Row, Table};

pub mod accounts;
pub mod cache;
pub mod config;
pub mod dns;
pub mod zones;

fn table_from_cols(columns: Vec<&str>) -> Table {
    let cols: Vec<&str> = columns.iter().map(|_| "{:<}").collect();
    let spec: &str = &cols.join("    ");
    let mut table = Table::new(spec);

    let mut header = Row::new();
    for c in &columns {
        header.add_cell(c);
    }

    table.add_row(header);
    table
}
