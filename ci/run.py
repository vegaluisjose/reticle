import subprocess as sp
import pathlib
import os

ci_dir = pathlib.Path(__file__).parent.absolute()
rust_manifest_dir = os.path.abspath(os.path.join(ci_dir, ".."))
user = sp.run(["id", "-u"], check=True, stdout=sp.PIPE)
group = sp.run(["id", "-g"], check=True, stdout=sp.PIPE)

docker_rust_workdir = "/usr/src/myapp"
docker_user_opt = "{}:{}".format(
    user.stdout.decode("utf-8").strip("\n"),
    group.stdout.decode("utf-8").strip("\n"),
)
docker_rust_mount_opt = "{}:{}".format(
    rust_manifest_dir, docker_rust_workdir
)


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


def test_reticle_fmt(docker):
    cmd = ["cargo", "fmt", "--", "--check"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_clippy(docker):
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


def test_reticle_interpreter(docker):
    cmd = ["cargo", "test"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_compiler_build(docker):
    cmd = ["cargo", "build", "--release"]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_examples_isa_scalar_register(docker):
    cmd = [
        "./target/release/reticle",
        "examples/isa/scalar/register.ret",
        "-b",
        "verilog",
        "-o",
        "ci/register.v",
    ]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_examples_basic_fsm(docker):
    cmd = [
        "./target/release/reticle",
        "examples/basic/fsm.ret",
        "-b",
        "verilog",
        "-o",
        "ci/fsm.v",
    ]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)


def test_reticle_examples_basic_vadd_const(docker):
    cmd = [
        "./target/release/reticle",
        "examples/basic/vadd_const.ret",
        "-b",
        "verilog",
        "-o",
        "ci/vadd_const.v",
    ]
    if docker:
        cmd = docker_rust_cmd + cmd
    sp.run(cmd, check=True)
