from docplex.mp.model import Model

def solve_lp_graph_problem(num_vertices: int, num_edges: int, edges: 'set[tuple[int, int]]'):
    for edge in edges:
        print(edge)

    model = Model(name='Graph Coloring Problem with UserCut')

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

    model.export('a.lp')

    solution = model.solve()
    solution.display()

solve_lp_graph_problem(25, 0, {(1,2), (2, 3)})