use arrow::csv::reader;

#[allow(dead_code)]
pub fn file_reader(f: &[String]) {
    let schema = reader::infer_schema_from_files(f, ',' as u8, None, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_handler() {
        let file = file_reader(&["C:/Users/bhlie/Desktop/Rust code/gfc/Members 062821.csv".to_owned()]);
        println!("{:?}", file);
    }
}