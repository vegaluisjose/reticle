import os


def get_curr_dir():
    return os.path.dirname(os.path.abspath(os.path.expanduser(__file__)))


def get_rust_dir():
    return os.path.abspath(os.path.join(get_curr_dir(), "..", "..", ".."))
