import subprocess as sp
import os
from util import get_curr_dir, get_rust_dir


def build_reticle():
    cmd = []
    cmd.append("cargo")
    cmd.append("build")
    cmd.append("--release")
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8")


def get_reticle_translate():
    return os.path.abspath(
        os.path.join(get_rust_dir(), "target", "release", "reticle-translate")
    )


def compile_generic(inp, out, use_dsp=False):
    cmd = []
    cmd.append(get_reticle_translate())
    cmd.append("-b")
    cmd.append("verilog")
    if use_dsp:
        cmd.append("--use-dsp")
    cmd.append("-o")
    cmd.append(out)
    cmd.append(inp)
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8")


def compile_reticle(inp, out):
    cmd = []
    cmd.append(get_reticle_translate())
    cmd.append("-b")
    cmd.append("reticle")
    cmd.append("-o")
    cmd.append(out)
    cmd.append(inp)
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8")
