from dataclasses import dataclass

Edges = set[tuple[int, int]]

@dataclass
class Graph:
    num_vertices: int
    num_edges: int
    edges: Edges