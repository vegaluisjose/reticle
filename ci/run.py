import subprocess as sp


def test_rust_fmt():
    sp.run(["cargo", "fmt", "--", "--check"], check=True)
