use crate::file_reader::graph_file_reader::GraphFileReader;

#[test]
fn test_can_read_file() {
    let filename = "../TPI_BC_COL_1.txt";

    let graph = GraphFileReader::new(filename).read_file();

    assert_eq!(graph.num_vertex, 25);
    assert_eq!(graph.num_edges, 320);
}
