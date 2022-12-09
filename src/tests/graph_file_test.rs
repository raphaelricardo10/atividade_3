use std::env;

use dotenv::dotenv;
use crate::file_reader::graph_file_reader::GraphFileReader;

#[test]
fn test_can_read_file() {
    let filename = "../TPI_BC_COL_1.txt";

    let graph = GraphFileReader::new(filename).read_file().unwrap();

    assert_eq!(graph.num_vertex, 25);
    assert_eq!(graph.num_edges, 320);
}

#[test]
fn test_can_read_csv_header() {
    dotenv().ok();
    let filename = format!("{}/TPI_BC_COL_0.txt", dotenv::var("INSTANCES_PATH").unwrap());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_path(filename).unwrap();

    let header = GraphFileReader::read_header(&mut reader);

    assert_eq!(header.num_vertex, 25);
    assert_eq!(header.num_edges, 320);
}
