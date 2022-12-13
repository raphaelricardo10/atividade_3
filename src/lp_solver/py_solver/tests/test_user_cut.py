from docplex.mp.model import Model

from solver import generate_model
from user_cut import GreedyCutCallback
from tests.fixtures.edges import edges
from tests.fixtures.graph import graph

from graph import Graph

def test_can_run_user_cut(graph: Graph):
    model, x, w = generate_model(graph.num_vertices, graph.num_edges, graph.edges)

    cb: GreedyCutCallback = model.register_callback(GreedyCutCallback)  
    cb.add_variables_info(x, w)
    cb.add_graph_info(graph.num_vertices, graph.num_edges, graph.edges)

    model.solve()

    assert cb._callback_triggered