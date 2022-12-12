use super::fixtures::graph_file::{filename, graph_file_reader};
use crate::file_reader::graph_file_reader::GraphFileReader;
use rstest::rstest;

#[rstest]
fn test_can_read_file(filename: String) {
    let graph = GraphFileReader::new(filename).read_file().unwrap();

    assert_eq!(graph.num_vertex, 25);
    assert_eq!(graph.num_edges, 320);
}

#[rstest]
fn test_can_read_csv_header(graph_file_reader: GraphFileReader) {
    let mut reader = graph_file_reader.make_reader().unwrap();

    let header = GraphFileReader::read_header(&mut reader);

    assert_eq!(header.num_vertex, 25);
    assert_eq!(header.num_edges, 320);
}

#[rstest]
fn test_can_read_csv_records(graph_file_reader: GraphFileReader) {
    let mut reader = graph_file_reader.make_reader().unwrap();
    reader.records().next();

    let records = GraphFileReader::read_records(&mut reader);

    assert_eq!(records.len(), 320);
}
