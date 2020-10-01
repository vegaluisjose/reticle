from os import path, makedirs
from generator import vector_add
from vivado import run_vivado, docker_vivado_workdir
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
    dirname = "results"
    result_dir = path.join(get_curr_dir(), dirname)
    vivado_dir = path.join(docker_vivado_workdir(), dirname)
    make_dir(result_dir)
    build_reticle()
    for l in lengths:
        name = "vadd{}".format(l)
        prog = path.join(result_dir, "{}.ret".format(name))
        vector_add(name, l, prog)
        for b in backends:
            use_dsp = True if b == "dsp" else False
            vname = "{}_{}".format(name, b)
            vfile = path.join(result_dir, "{}.v".format(vname))
            if b == "ret":
                compile_reticle(prog, vfile)
                run_vivado(
                    ["vivado.sh", "generic.tcl", vname, vivado_dir, name]
                )
            else:
                compile_generic(prog, vfile, use_dsp)
                run_vivado(
                    ["vivado.sh", "generic.tcl", vname, vivado_dir, name]
                )
