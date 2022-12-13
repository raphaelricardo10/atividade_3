import ctypes

from graph import Graph, Edges
from collections import UserList

class PartialSolutionEntry(ctypes.Structure):
    _fields_ = [
        ("vertex", ctypes.c_uint32),
        ("color", ctypes.c_uint32),
        ("weight", ctypes.c_float),
    ]

class PartialSolution(UserList):
    def __init__(self, data) -> None:
        super().__init__(PartialSolution.make_entries(data))

        self.c_type = (PartialSolutionEntry * len(data))

    def make_entries(data):
        return [PartialSolutionEntry(vertex, color, weight) for (vertex, color), weight in data.items()]

class ABIEdge(ctypes.Structure):
    _fields_ = [
        ("src", ctypes.c_uint32),
        ("dest", ctypes.c_uint32),
    ]

class ABIGraph(ctypes.Structure):
    _fields_ = [
        ("num_vertices", ctypes.c_uint32),
        ("num_edges", ctypes.c_size_t),
        ("edges", ctypes.POINTER(ABIEdge)),
    ]

    def __init__(self, graph: Graph) -> None:
        super().__init__(graph.num_vertices, graph.num_edges, ABIGraph.make_edges_ptr(graph.edges))

    def make_edges_ptr(edges: 'list[(int, int)]'):
        abi_edges = [ABIEdge(*x) for x in edges]
        return (ABIEdge * len(abi_edges))(*abi_edges)


class HeuristicsCallbackLibrary:
    def __init__(self, path: str, graph: Graph) -> None:
        self.path = path
        self.graph = graph
        self.lib = ctypes.cdll.LoadLibrary(path)

    def _greedy_solver_config(self, func, partial_solution: 'dict[tuple[int, int], float]'):
        partial_solution: PartialSolution = PartialSolution(partial_solution)
        abi_graph = ABIGraph(self.graph)

        func.argtypes = [
            ABIGraph,
            partial_solution.c_type,
            ctypes.c_size_t
        ]

        return abi_graph, partial_solution.c_type(*partial_solution), len(partial_solution)

    def run_greedy_solver(self, partial_solution: 'dict[tuple[int, int], float]'):
        args = self._greedy_solver_config(self.lib.greedy_solver_callback, partial_solution)

        return self.lib.greedy_solver_callback(*args)