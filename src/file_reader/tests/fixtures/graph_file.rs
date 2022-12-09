use rstest::fixture;

use crate::file_reader::graph_file_reader::GraphFileReader;

#[fixture]
pub fn filename() -> String {
    format!("{}/TPI_BC_COL_0.txt", env!("INSTANCES_PATH"))
}

#[fixture]
pub(crate) fn graph_file_reader(filename: String) -> GraphFileReader {
    GraphFileReader::new(filename)
}
