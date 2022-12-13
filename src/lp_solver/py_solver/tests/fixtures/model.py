from pytest import fixture
from docplex.mp.model import Model

@fixture
def model():
    m = Model("Test model")
    x = m.binary_var("x")
    y = m.binary_var("y")
    z = m.binary_var("z")

    m.add_constraint(x + y + z <= 2)

    m.maximize(10*x + 20*y + 5*z)

    params = m.parameters
    params.threads = 1
    # params.mip.strategy.heuristicfreq = -1
    params.mip.cuts.mircut = -1
    params.mip.cuts.implied = -1
    params.mip.cuts.gomory = -1
    params.mip.cuts.flowcovers = -1
    params.mip.cuts.pathcut = -1
    params.mip.cuts.liftproj = -1
    params.mip.cuts.zerohalfcut = -1
    params.mip.cuts.cliques = -1
    params.mip.cuts.covers = -1

    return m