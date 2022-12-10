use std::{collections::HashSet, error::Error, fs::File};

use crate::domain::graph::Graph;
use csv;

use super::graph_file_entries::{Header, Record};

pub struct GraphFileReader {
    filename: String,
}

impl GraphFileReader {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }

    pub(crate) fn make_reader(&self) -> Result<csv::Reader<File>, csv::Error> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .flexible(true)
            .from_path(&self.filename)
    }

    pub fn read_file(&self) -> Result<Graph, Box<dyn Error>> {
        let mut reader = self.make_reader().unwrap();

        let header = Self::read_header(&mut reader);
        let edges = Self::read_records(&mut reader);

        Ok(Graph::new(header.num_edges, header.num_vertex, edges))
    }

    pub(crate) fn read_header(reader: &mut csv::Reader<File>) -> Header {
        let headers = csv::StringRecord::from(vec!["p", "edge", "num_vertex", "num_edges"]);

        reader
            .records()
            .map(|record| record.unwrap().deserialize(Some(&headers)).unwrap())
            .next()
            .unwrap()
    }

    pub(crate) fn read_records(reader: &mut csv::Reader<File>) -> HashSet<(u32, u32)> {
        let headers = csv::StringRecord::from(vec!["e", "from", "to"]);

        reader
            .records()
            .skip(1)
            .map(|line| {
                let record: Record = line.unwrap().deserialize(Some(&headers)).unwrap();

                (record.from, record.to)
            })
            .collect()
    }
}
