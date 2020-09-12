import subprocess as sp
import os
import re
import pytest


examples = [
    "examples/isa/scalar/register.ret",
    "examples/basic/fsm.ret",
    "examples/basic/vadd_const.ret",
]


def get_ci_dir():
    return os.path.dirname(os.path.abspath(os.path.expanduser(__file__)))


def get_root_dir():
    return os.path.abspath(os.path.join(get_ci_dir(), ".."))


def get_docker_vivado_dir():
    return os.path.join("/home/vivado/workspace")


def get_docker_rust_dir():
    return os.path.join("/usr/src/myapp")


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
    cmd.append(docker_bind_opt(get_ci_dir(), get_docker_vivado_dir()))
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
    pattern = re.compile(".*~~FAIL~~.*", re.DOTALL)
    if pattern.match(stdout) is None:
        return True
    else:
        print(stdout)
        return False


def run_vivado_sim(docker: bool, infile: str):
    name = get_example_name(infile)
    test_name = "test_{}".format(name)
    dut = "{}.v".format(name)
    test = "{}.v".format(test_name)
    cmd = []
    cmd.append("vivado_sim.sh")
    cmd.append(test_name)
    cmd.append(get_vivado_path(docker, test))
    cmd.append(get_vivado_path(docker, dut))
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


def test_reticle_build(docker: bool):
    build_reticle(docker)


@pytest.mark.parametrize("example", examples)
def test_reticle_compiler(docker: bool, example: str):
    reticle_to_verilog(docker, example, "ci")
    assert run_vivado_sim(docker, example)
