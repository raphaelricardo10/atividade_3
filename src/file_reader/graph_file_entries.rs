use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct Header {
    p: String,
    edge: String,
    pub(crate) num_vertex: u32,
    pub(crate) num_edges: u32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct Record {
    e: String,
    pub(crate) from: u32,
    pub(crate) to: u32,
}
