import subprocess as sp
import os
import re
import pytest


examples = [
    "examples/isa/scalar/dsp_add_i8_i8_i8.ret",
    "examples/isa/scalar/dsp_sub_i8_i8_i8.ret",
    "examples/isa/scalar/dsp_and_i8_i8_i8.ret",
    "examples/isa/scalar/dsp_or_i8_i8_i8.ret",
    "examples/isa/scalar/lut_add_i8_i8_i8.ret",
    "examples/isa/scalar/lut_sub_i8_i8_i8.ret",
    "examples/isa/scalar/lut_and_i8_i8_i8.ret",
    "examples/isa/scalar/lut_or_i8_i8_i8.ret",
    "examples/isa/scalar/add_i8_i8_i8.ret",
    "examples/isa/scalar/sub_i8_i8_i8.ret",
    "examples/isa/scalar/mul_i8_i8_i8.ret",
    "examples/isa/scalar/add_mul_i8_i8_i8_i8.ret",
    "examples/isa/scalar/add_reg_mul_i8_i8_i8_b_i8.ret",
    "examples/isa/scalar/and_i8_i8_i8.ret",
    "examples/isa/scalar/and_b_b_b.ret",
    "examples/isa/scalar/or_i8_i8_i8.ret",
    "examples/isa/scalar/or_b_b_b.ret",
    "examples/isa/scalar/xor_i8_i8_i8.ret",
    "examples/isa/scalar/xor_b_b_b.ret",
    "examples/isa/scalar/nand_i8_i8_i8.ret",
    "examples/isa/scalar/nand_b_b_b.ret",
    "examples/isa/scalar/nor_i8_i8_i8.ret",
    "examples/isa/scalar/nor_b_b_b.ret",
    "examples/isa/scalar/xnor_i8_i8_i8.ret",
    "examples/isa/scalar/xnor_b_b_b.ret",
    "examples/isa/scalar/eq_b_i8_i8.ret",
    "examples/isa/scalar/eq_b_b_b.ret",
    "examples/isa/scalar/neq_b_i8_i8.ret",
    "examples/isa/scalar/neq_b_b_b.ret",
    "examples/isa/scalar/id_i8_i8.ret",
    "examples/isa/scalar/id_b_b.ret",
    "examples/isa/scalar/const_i8.ret",
    "examples/isa/scalar/mux_i8_b_i8_i8.ret",
    "examples/isa/scalar/mux_b_b_b_b.ret",
    "examples/isa/scalar/not_i8_i8.ret",
    "examples/isa/scalar/not_b_b.ret",
    "examples/isa/scalar/reg_i8_i8_b.ret",
    "examples/isa/vector/add_i8v4_i8v4_i8v4.ret",
    "examples/isa/vector/sub_i8v4_i8v4_i8v4.ret",
    "examples/basic/dot.ret",
    "examples/basic/pipeline.ret",
    "examples/basic/fsm.ret",
    "examples/basic/vadd_const.ret",
]


def get_ci_dir():
    return os.path.dirname(os.path.abspath(os.path.expanduser(__file__)))


def get_root_dir():
    return os.path.abspath(os.path.join(get_ci_dir(), ".."))


def get_registry_dir():
    return os.path.join(get_root_dir(), ".cargo/registry")


def get_docker_vivado_dir():
    return os.path.join("/home/vivado/workspace")


def get_docker_rust_dir():
    return os.path.join("/usr/src/myapp")


def get_docker_rust_registry_dir():
    return os.path.join("/usr/local/cargo/registry")


def get_rust_path(docker: bool, path: str):
    basedir = get_docker_rust_dir() if docker is True else get_root_dir()
    return os.path.join(basedir, path)


def get_vivado_path(docker: bool, path: str):
    basedir = get_docker_vivado_dir() if docker is True else get_root_dir()
    return os.path.join(basedir, path)


def get_id(opt: str) -> str:
    cp = sp.run(["id", opt], check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8").strip("\n")


def get_user_id() -> str:
    return get_id("-u")


def get_group_id() -> str:
    return get_id("-g")


def get_example_name(path: str) -> str:
    file, ext = os.path.splitext(path)
    return os.path.basename(file)


def docker_bind_opt(src: str, dst: str) -> str:
    return "{}:{}".format(src, dst)


def build_docker_rust_cmd():
    cmd = []
    cmd.append("docker")
    cmd.append("run")
    cmd.append("--rm")
    cmd.append("--pid=host")
    cmd.append("--user")
    cmd.append(docker_bind_opt(get_user_id(), get_group_id()))
    cmd.append("-v")
    cmd.append(docker_bind_opt(get_root_dir(), get_docker_rust_dir()))
    cmd.append("-v")
    cmd.append(
        docker_bind_opt(get_registry_dir(), get_docker_rust_registry_dir())
    )
    cmd.append("-w")
    cmd.append(get_docker_rust_dir())
    cmd.append("reticle-rust")
    return cmd


def build_docker_vivado_cmd():
    cmd = []
    cmd.append("docker")
    cmd.append("run")
    cmd.append("--rm")
    cmd.append("--pid=host")
    cmd.append("--user")
    cmd.append(docker_bind_opt(get_user_id(), get_group_id()))
    cmd.append("-v")
    cmd.append(docker_bind_opt(get_root_dir(), get_docker_vivado_dir()))
    cmd.append("-w")
    cmd.append(get_docker_vivado_dir())
    cmd.append("vivado")
    cmd.append("bash")
    cmd.append("--login")
    return cmd


def run_rust(docker: bool, cmd):
    if docker is True:
        cmd = build_docker_rust_cmd() + cmd
    sp.run(cmd, check=True)


def run_vivado(docker: bool, cmd):
    if docker is True:
        cmd = build_docker_vivado_cmd() + cmd
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8")


def check_vivado_fail(stdout: str):
    fail = re.compile(".*~~FAIL~~.*", re.DOTALL)
    error = re.compile(".*ERROR.*", re.DOTALL)
    if fail.match(stdout) is None and error.match(stdout) is None:
        return True
    else:
        print(stdout)
        return False


def run_vivado_sim(docker: bool, outdir: str):
    script = os.path.join("ci", "vivado_sim.sh")
    cmd = []
    cmd.append(get_vivado_path(docker, script))
    cmd.append(get_vivado_path(docker, "ci/tests"))
    cmd.append(get_vivado_path(docker, outdir))
    cmd.append(get_vivado_path(docker, "output"))
    return check_vivado_fail(run_vivado(docker, cmd))


def build_reticle(docker: bool):
    cmd = []
    cmd.append("cargo")
    cmd.append("build")
    cmd.append("--release")
    run_rust(docker, cmd)


def reticle_to_verilog(docker: bool, infile: str, outdir: str):
    build_reticle(docker)
    filename = "{}.v".format(get_example_name(infile))
    outfile = os.path.join(outdir, filename)
    cmd = []
    cmd.append(get_rust_path(docker, "target/release/reticle"))
    cmd.append(get_rust_path(docker, infile))
    cmd.append("-b")
    cmd.append("verilog")
    cmd.append("-o")
    cmd.append(get_rust_path(docker, outfile))
    run_rust(docker, cmd)


def test_reticle_fmt(docker: bool):
    cmd = []
    cmd.append("cargo")
    cmd.append("fmt")
    cmd.append("--")
    cmd.append("--check")
    run_rust(docker, cmd)


def test_reticle_clippy(docker: bool):
    cmd = []
    cmd.append("cargo")
    cmd.append("clippy")
    cmd.append("--all-targets")
    cmd.append("--all-features")
    cmd.append("--")
    cmd.append("-D")
    cmd.append("warnings")
    run_rust(docker, cmd)


def test_reticle_interpreter(docker: bool):
    cmd = []
    cmd.append("cargo")
    cmd.append("test")
    run_rust(docker, cmd)


def test_reticle_build(docker: bool):
    build_reticle(docker)


def test_reticle_compiler(docker: bool):
    wd = "ci/verilog"
    for example in examples:
        print("Compiling {}...", example)
        reticle_to_verilog(docker, example, wd)
    assert run_vivado_sim(docker, wd)
