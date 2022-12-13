from pytest import fixture

from graph import Graph
from tests.fixtures.edges import edges

@fixture
def graph(edges):
    return Graph(25, 320, edges)

