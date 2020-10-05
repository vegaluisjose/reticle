import re
from os import path
import pandas as pd


def build_util_pattern(start, end):
    return r".*{}\s+\|\s+(\b\d+\b)\s+\|\s+{}.*".format(start, end)


def build_util_pattern_map():
    pat = {}
    for i in range(1, 7):
        pat["lut{}".format(i)] = build_util_pattern("LUT{}".format(i), "CLB")
    pat["carry"] = build_util_pattern("CARRY8", "CLB")
    pat["fdre"] = build_util_pattern("FDRE", "Register")
    pat["fdse"] = build_util_pattern("FDSE", "Register")
    pat["dsp"] = build_util_pattern("DSP48E2", "Arithmetic")
    comp = {}
    for k, v in pat.items():
        comp[k] = re.compile(v)
    return comp


def build_runtime_pattern():
    return re.compile(r"Data Path Delay:\s+(\d+\.\d+).*")


def count(data, types):
    num = 0
    for t in types:
        if t in data:
            num += data[t]
    return num


def update_util_data(frame, ty, number, length, backend):
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


def update_runtime_data(frame, backend, length, time):
    if frame:
        frame["backend"].append(backend)
        frame["length"].append(length)
        frame["time"].append(time)
    else:
        frame["backend"] = [backend]
        frame["length"] = [length]
        frame["time"] = [time]
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
                    for k, pat in build_util_pattern_map().items():
                        m = re.search(pat, f)
                        if m is not None:
                            data[k] = int(m.group(1))
            num = {}
            num["lut"] = count(data, ["lut{}".format(i) for i in range(1, 7)])
            num["reg"] = count(data, ["fdre", "fdse"])
            num["dsp"] = count(data, ["dsp"])
            num["carry"] = count(data, ["carry"])
            for k, v in num.items():
                frame = update_util_data(frame, k, v, l, b)
    return frame


def parse_runtime(name, dirname, lengths, backends):
    frame = {}
    pat = build_runtime_pattern()
    for l in lengths:
        for b in backends:
            filename = "{}{}_{}_timing.txt".format(name, l, b)
            file = path.join(dirname, filename)
            data = {}
            with open(file, "r") as file:
                for f in file:
                    m = re.search(pat, f)
                    if m is not None:
                        runtime = float(m.group(1))
                        frame = update_runtime_data(frame, b, l, runtime)
    return frame


if __name__ == "__main__":
    lengths = [8, 16, 32, 64, 128, 256, 512, 1024]
    backends = ["gen", "dsp", "ret"]
    dirname = "results"
    name = "vadd"
    util = parse_util(name, dirname, lengths, backends)
    util_df = pd.DataFrame.from_dict(util)
    util_df.to_csv("util.csv", Index=False)
    runtime = parse_runtime(name, dirname, lengths, backends)
    runtime_df = pd.DataFrame.from_dict(runtime)
    runtime_df.to_csv("runtime.csv", Index=False)
