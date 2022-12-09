use dotenv::dotenv;
use rstest::fixture;

use crate::file_reader::graph_file_reader::GraphFileReader;

#[fixture]
pub fn filename() -> String {
    dotenv().ok();

    format!(
        "{}/TPI_BC_COL_0.txt",
        dotenv::var("INSTANCES_PATH").unwrap()
    )
}

#[fixture]
pub(crate) fn graph_file_reader(filename: String) -> GraphFileReader {
    GraphFileReader::new(filename)
}
