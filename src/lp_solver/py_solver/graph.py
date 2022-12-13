from dataclasses import dataclass


@dataclass
class Graph:
    num_vertices: int
    num_edges: int
    edges: set[tuple[int, int]]