from graph import Edges
from user_cut import GreedyCutCallback

from docplex.mp.model import Model

def generate_model(num_vertices: int, num_edges: int, edges: Edges):
    model = Model(name='Graph Coloring Problem with UserCut')

    # Tweak some CPLEX parameters so that CPLEX has a harder time to
    # solve the model and our cut separators can actually kick in.
    params = model.parameters
    params.threads = 1
    params.mip.strategy.heuristicfreq = -1
    params.mip.cuts.mircut = -1
    params.mip.cuts.implied = -1
    params.mip.cuts.gomory = -1
    params.mip.cuts.flowcovers = -1
    params.mip.cuts.pathcut = -1
    params.mip.cuts.liftproj = -1
    params.mip.cuts.zerohalfcut = -1
    params.mip.cuts.cliques = -1
    params.mip.cuts.covers = -1
    params.preprocessing.presolve = 0

    # Variáveis
    w = {j: model.binary_var(name=f"w_{j + 1}") for j in range(num_vertices)}
    x = {(i,j): model.binary_var(name=f"x_{i + 1}_{j + 1}") for i in range(num_vertices) for j in range(num_vertices)}

    # Todo vértice deve ter (somente) uma cor
    for i in range(num_vertices):
        sum = model.sum(x[(i,j)] for j in range(num_vertices))
        model.add_constraint(sum == 1)

    # Cada vértice de uma aresta deve ter uma cor diferente
    for j in range(num_vertices):
        for edge in edges:
            i = edge[0] - 1
            k = edge[1] - 1

            model.add_constraint(x[(i,j)] + x[(k,j)] <= w[j])

    # Quebra de simetria
    for j in range(num_vertices - 1):
        model.add_constraint(w[j] >= w[j + 1])

    # Restrição de fortalecimento
    for j in range(num_vertices):
        sum = model.sum(x[(i,j)] for i in range(num_vertices))
        model.add_constraint(w[j] <= sum)

    fo = model.sum(w[j] for j in range(num_vertices))
    model.minimize(fo)

    return model, x, w

def solve_lp_graph_problem(num_vertices: int, num_edges: int, edges: Edges):
    model, x, w = generate_model(num_vertices, num_edges, edges)

    greedy_cut_cb: GreedyCutCallback = model.register_callback(GreedyCutCallback)
    greedy_cut_cb.add_variables_info(x, w)
    greedy_cut_cb.add_graph_info(num_vertices, num_edges, edges)

    solution = model.solve()