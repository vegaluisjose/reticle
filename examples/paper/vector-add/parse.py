import re


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


def process(data):
    out = {}
    out["lut"] = count(data, ["lut{}".format(i) for i in range(1, 7)])
    out["reg"] = count(data, ["fdre", "fdse"])
    out["dsp"] = count(data, ["dsp"])
    out["carry"] = count(data, ["carry"])
    return out


if __name__ == "__main__":
    filename = "results/vadd1024_gen_util.txt"
    data = {}
    with open(filename, "r") as file:
        for f in file:
            for k, pat in build_re_dict().items():
                m = re.search(pat, f)
                if m is not None:
                    data[k] = int(m.group(1))
    file.close()
    print(data)
    print(process(data))
