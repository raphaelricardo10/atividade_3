import os
import ctypes

from pytest import fixture

from graph import Graph
from tests.fixtures.edges import edges
from tests.fixtures.graph import graph
from tests.fixtures.partial_sol import partial_solution
from callback_library import HeuristicsCallbackLibrary as HeuristicsCallbackLibrary

@fixture
def lib_path():
    return f"{os.getenv('WORKSPACE_FOLDER')}/target/debug/libgraph_col_solver.so"


class HeuristicsCallbackLibraryTest(HeuristicsCallbackLibrary):
    def __init__(self, path: str) -> None:
        super().__init__(path, Graph(0,0,[]))

    def run_greedy_solver(self, partial_solution: 'dict[tuple[int, int], float]'):
        args = self._greedy_solver_config(self.lib.test_greedy_solver_callback, partial_solution)
        self.lib.restype = ctypes.c_bool

        return self.lib.test_greedy_solver_callback(*args)

def test_can_call_library(lib_path, partial_solution):
    lib = HeuristicsCallbackLibraryTest(lib_path)

    assert lib.run_greedy_solver(partial_solution)


def test_can_generate_weighted_graph(lib_path, partial_solution, graph):
    lib = HeuristicsCallbackLibrary(lib_path, graph)

    lib.run_greedy_solver(partial_solution)