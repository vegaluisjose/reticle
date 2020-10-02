import re
from os import path
import pandas as pd


def build_re(start, end):
    return r".*{}\s+\|\s+(\b\d+\b)\s+\|\s+{}.*".format(start, end)


def build_re_dict():
    pat = {}
    for i in range(1, 7):
        pat["lut{}".format(i)] = build_re("LUT{}".format(i), "CLB")
    pat["carry"] = build_re("CARRY8", "CLB")
    pat["fdre"] = build_re("FDRE", "Register")
    pat["fdse"] = build_re("FDSE", "Register")
    pat["dsp"] = build_re("DSP48E2", "Arithmetic")
    comp = {}
    for k, v in pat.items():
        comp[k] = re.compile(v)
    return comp


def count(data, types):
    num = 0
    for t in types:
        if t in data:
            num += data[t]
    return num


def update_frame(frame, ty, number, length, backend):
    if frame:
        frame["type"].append(ty)
        frame["number"].append(number)
        frame["length"].append(length)
        frame["backend"].append(backend)
    else:
        frame["type"] = [ty]
        frame["number"] = [number]
        frame["length"] = [length]
        frame["backend"] = [backend]
    return frame


def parse_util(name, dirname, lengths, backends):
    frame = {}
    for l in lengths:
        for b in backends:
            filename = "{}{}_{}_util.txt".format(name, l, b)
            file = path.join(dirname, filename)
            data = {}
            with open(file, "r") as file:
                for f in file:
                    for k, pat in build_re_dict().items():
                        m = re.search(pat, f)
                        if m is not None:
                            data[k] = int(m.group(1))
            num = {}
            num["lut"] = count(data, ["lut{}".format(i) for i in range(1, 7)])
            num["reg"] = count(data, ["fdre", "fdse"])
            num["dsp"] = count(data, ["dsp"])
            num["carry"] = count(data, ["carry"])
            for k, v in num.items():
                frame = update_frame(frame, k, v, l, b)
    return frame


if __name__ == "__main__":
    lengths = [8, 16, 32, 64, 128, 256, 512, 1024]
    backends = ["gen", "dsp", "ret"]
    dirname = "results"
    name = "vadd"
    frame = parse_util(name, dirname, lengths, backends)
    df = pd.DataFrame.from_dict(frame)
    df.to_csv("util.csv")
