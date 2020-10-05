from os import path, makedirs
from generator import vector_add
from vivado import run_vivado, docker_vivado_workdir
from reticle import (
    build_reticle,
    compile_generic,
    compile_reticle,
)
from util import get_curr_dir
from time import perf_counter
import pandas as pd


def make_dir(value):
    value = path.join(value)
    if not path.isdir(value):
        makedirs(value)


def update_frame(frame, backend, length, time):
    if frame:
        frame["backend"].append(backend)
        frame["length"].append(length)
        frame["time"].append(time)
    else:
        frame["backend"] = [backend]
        frame["length"] = [length]
        frame["time"] = [time]
    return frame


def bench(name, dirname, lengths, backends):
    result_dir = path.join(get_curr_dir(), dirname)
    vivado_dir = path.join(docker_vivado_workdir(), dirname)
    make_dir(result_dir)
    build_reticle()
    runtime = {}
    for l in lengths:
        pname = "{}{}".format(name, l)
        prog = path.join(result_dir, "{}.ret".format(pname))
        vector_add(pname, l, prog)
        for b in backends:
            use_dsp = True if b == "dsp" else False
            vname = "{}_{}".format(pname, b)
            vfile = path.join(result_dir, "{}.v".format(vname))
            if b == "ret":
                start = perf_counter()
                compile_reticle(prog, vfile)
                elapsed = perf_counter() - start
            else:
                compile_generic(prog, vfile, use_dsp)
                start = perf_counter()
                run_vivado(["vivado.sh", "synth.tcl", vname, vivado_dir, pname])
                elapsed = perf_counter() - start
            runtime = update_frame(runtime, b, l, elapsed)
    return runtime


if __name__ == "__main__":
    lengths = [8, 16, 32, 64, 128, 256, 512, 1024]
    backends = ["gen", "dsp", "ret"]
    dirname = "bench_compiler"
    name = "vadd"
    runtime = bench(name, dirname, lengths, backends)
    df = pd.DataFrame.from_dict(runtime)
    df.to_csv("compiler.csv", index=False)
