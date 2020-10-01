import subprocess as sp
import os
import re
from util import get_curr_dir


def get_id(opt: str) -> str:
    cp = sp.run(["id", opt], check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8").strip("\n")


def get_group_id() -> str:
    return get_id("-g")


def get_user_id() -> str:
    return get_id("-u")


def docker_bind_opt(src: str, dst: str) -> str:
    return "{}:{}".format(src, dst)


def docker_vivado_workdir() -> str:
    return os.path.join("/home", "vivado", "workspace")


def build_docker_vivado_cmd():
    cmd = []
    cmd.append("docker")
    cmd.append("run")
    cmd.append("--rm")
    cmd.append("--pid=host")
    cmd.append("--user")
    cmd.append(docker_bind_opt(get_user_id(), get_group_id()))
    cmd.append("-v")
    cmd.append(docker_bind_opt(get_curr_dir(), docker_vivado_workdir()))
    cmd.append("-w")
    cmd.append(docker_vivado_workdir())
    cmd.append("vivado")
    cmd.append("bash")
    cmd.append("--login")
    return cmd


def run_vivado(cmd, docker=True):
    if not isinstance(cmd, list):
        cmd = [cmd]
    if docker is True:
        cmd = build_docker_vivado_cmd() + cmd
    cp = sp.run(cmd, check=True, stdout=sp.PIPE)
    return cp.stdout.decode("utf-8")
