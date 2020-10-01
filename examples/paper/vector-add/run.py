from os import path, makedirs
from generator import vector_add
from vivado import run_vivado
from reticle import (
    build_reticle,
    compile_generic,
    compile_reticle,
)
from util import get_curr_dir


def make_dir(value):
    value = path.join(value)
    if not path.isdir(value):
        makedirs(value)


if __name__ == "__main__":
    lengths = [8, 16]
    backends = ["gen", "dsp", "ret"]
    result_dir = path.join(get_curr_dir(), "results")
    make_dir(result_dir)
    build_reticle()
    for l in lengths:
        name = "vadd{}".format(l)
        prog = path.join(result_dir, "{}.ret".format(name))
        vector_add(name, l, prog)
        for b in backends:
            vlog = path.join(result_dir, "{}_{}.v".format(name, b))
            if b == "dsp":
                compile_generic(prog, vlog, use_dsp=True)
            elif b == "ret":
                compile_reticle(prog, vlog)
            else:
                compile_generic(prog, vlog, use_dsp=False)
