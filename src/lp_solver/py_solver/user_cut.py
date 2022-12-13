from graph import Graph, Edges
from cplex.callbacks import UserCutCallback

from docplex.mp.solution import SolveSolution
from docplex.mp.callbacks.cb_mixin import ConstraintCallbackMixin, print_called


class GreedyCutCallback(ConstraintCallbackMixin, UserCutCallback):
    # Callback constructor. Model object is set after registration.
    def __init__(self, env):
        UserCutCallback.__init__(self, env)
        ConstraintCallbackMixin.__init__(self)
        self.eps = 1e-6
        self.nb_cuts = 0
        self.verbose = False
        self._callback_triggered = False

    def add_cut_constraint(self, ct):
        self.register_constraint(ct)

    def add_graph_info(self, num_vertices: int, num_edges: int, edges: Edges):
        self.graph = Graph(num_vertices, num_edges, edges)

    def add_variables_info(self, x, w):
        self.x = x
        self.w = w

    def verify_variables(self):
        try:
            self.x
            self.w
            self.graph

        except AttributeError:
            raise AttributeError("Please, register the graph and the variables before starting the solver")

    def pretty_print_x(self, solution: SolveSolution):
        i = 0

        for v in solution.get_value_dict(self.x).items():
            if i % 3 == 0:
                print(f"\n{v}",end="\t\t\t\t")

            else:
                print(f"{v}", end="\t\t\t\t")

            i += 1

    @print_called("--> custom cut callback called: #{0}")
    def __call__(self):
        self.verify_variables()
        self._callback_triggered = True

        # fetch variable solution values at this point.
        sol: SolveSolution = self.make_complete_solution()

        if self.verbose:
            print(sol.get_value_dict(self.w))
            self.pretty_print_x(sol)
            print("\n\n\n\n")

        # fetch those constraints which are not satisfied.
        unsats = self.get_cpx_unsatisfied_cts(self.cts, sol, self.eps)

        for ct, cut, sense, rhs in unsats:
            # Method add() here is CPLEX's CutCallback.add()
            print(ct, cut, sense, rhs)
            # self.add(cut, sense, rhs)
            self.nb_cuts += 1
            print('-- add new cut[{0}]: [{1!s}]'.format(self.nb_cuts, ct))