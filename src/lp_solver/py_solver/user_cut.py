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
        self.x = None
        self.w = None

    def add_cut_constraint(self, ct):
        self.register_constraint(ct)

    @print_called("--> custom cut callback called: #{0}")
    def __call__(self):
        # fetch variable solution values at this point.
        sol: SolveSolution = self.make_complete_solution()
        print(sol.get_value_dict(self.x))     
        print(sol.get_value_dict(self.w))
        # fetch those constraints which are not satisfied.
        unsats = self.get_cpx_unsatisfied_cts(self.cts, sol, self.eps)
        for ct, cut, sense, rhs in unsats:
            # Method add() here is CPLEX's CutCallback.add()
            self.add(cut, sense, rhs)
            self.nb_cuts += 1
            print('-- add new cut[{0}]: [{1!s}]'.format(self.nb_cuts, ct))