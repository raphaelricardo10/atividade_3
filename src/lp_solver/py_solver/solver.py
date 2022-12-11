from docplex.mp.model import Model

def solve_lp_graph_problem(num_vertices, num_edges, edges):
    m = Model(name='Graph Coloring Problem with UserCut')
    x = m.continuous_var(name="x", lb=0)
    c1 = m.add_constraint(x >= 2, ctname="const1")
    m.set_objective("min", 3*x)
    m.print_information()
    m.solve()
    m.print_solution()