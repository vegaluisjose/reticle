import subprocess as sp
import pathlib
import os
import re
import pytest

ci_dir = pathlib.Path(__file__).parent.absolute()
out_dir = os.path.abspath(os.path.join(ci_dir, "out"))
rust_manifest_dir = os.path.abspath(os.path.join(ci_dir, ".."))

user = sp.run(["id", "-u"], check=True, stdout=sp.PIPE)
group = sp.run(["id", "-g"], check=True, stdout=sp.PIPE)

docker_rust_workdir = "/usr/src/myapp"
docker_vivado_workdir = "/home/vivado/workspace"
docker_vivado_outdir = "/home/vivado/output"

docker_user_opt = "{}:{}".format(
    user.stdout.decode("utf-8").strip("\n"),
    group.stdout.decode("utf-8").strip("\n"),
)

docker_rust_mount_opt = "{}:{}".format(rust_manifest_dir, docker_rust_workdir)

docker_vivado_mount_opt = "{}:{}".format(ci_dir, docker_vivado_workdir)

docker_rust_cmd = [
    "docker",
    "run",
    "--rm",
    "--pid=host",
    "--user",
    docker_user_opt,
    "-v",
    docker_rust_mount_opt,
    "-w",
    docker_rust_workdir,
    "reticle-rust",
]

docker_vivado_cmd = [
    "docker",
    "run",
    "--rm",
    "--pid=host",
    "--user",
    docker_user_opt,
    "-v",
    docker_vivado_mount_opt,
    "-w",
    docker_vivado_workdir,
    "vivado",
    "bash",
    "--login",
]

vivado_fail_pattern = re.compile(".*~~FAIL~~.*", re.DOTALL)

reticle_examples = [
    pytest.param(
        "examples/isa/scalar/register.ret",
        "ci/register.v",
        id="register",
    ),
    pytest.param(
        "examples/basic/fsm.ret",
        "ci/fsm.v",
        id="fsm",
    ),
    pytest.param(
        "examples/basic/vadd_const.ret",
        "ci/vadd_const.v",
        id="vadd_const",
    ),
]

vivado_sim_tests = ["register", "fsm", "vadd_const"]


def test_reticle_fmt(docker: bool):
    cmd = ["cargo", "fmt", "--", "--check"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_clippy(docker: bool):
    cmd = [
        "cargo",
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_interpreter(docker: bool):
    cmd = ["cargo", "test"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_compiler_build(docker):
    cmd = ["cargo", "build", "--release"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


@pytest.mark.parametrize("inp,out", reticle_examples)
def test_reticle_example(docker: bool, inp: str, out: str):
    cmd = [
        "./target/release/reticle",
        inp,
        "-b",
        "verilog",
        "-o",
        out,
    ]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


@pytest.mark.parametrize("name", vivado_sim_tests)
def test_reticle_verilog(docker: bool, name: str):
    wd = docker_vivado_workdir if docker else str(ci_dir)
    od = docker_vivado_outdir if docker else str(out_dir)
    test_name = "test_{}".format(name)
    test_file = "{}/{}.v".format(wd, test_name)
    dut_file = "{}/{}.v".format(wd, name)
    cmd = [
        "vivado_sim.sh",
        test_name,
        test_file,
        dut_file,
        od,
    ]
    if docker:
        cmd = docker_vivado_cmd + cmd
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    stdout = cp.stdout.decode("utf-8")
    if vivado_fail_pattern.match(stdout):
        print(stdout)
        assert 0
