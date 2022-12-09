use crate::domain::graph::Graph;

pub(crate) struct GraphFileReader<'a> {
    filename: &'a str,
}

impl<'a> GraphFileReader<'a> {
    pub(crate) fn new(filename: &'a str) -> Self {
        Self { filename }
    }

    pub(crate) fn read_file(&self) -> Graph {
        todo!()
    }
}
