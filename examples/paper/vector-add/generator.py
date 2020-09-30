import argparse


def fmt(ident, value):
    return "{}{}".format(ident, value)


def expr(ident, ty):
    return "{}:{}".format(ident, ty)


def port(ident, index, ty):
    return expr(fmt(ident, index), ty)


def reg(ident, inp, en, ty):
    e = expr(ident, ty)
    return "{} = reg[0]({}, {});".format(e, inp, en)


def add(ident, lhs, rhs, ty):
    e = expr(ident, ty)
    return "{} = add({}, {});".format(e, lhs, rhs)


def signature(name, inps, outs):
    i = ", ".join(inps)
    o = ", ".join(outs)
    signature = "def {}({})->({})".format(name, i, o)
    return signature


def prog(name, inps, outs, body):
    s = signature(name, inps, outs)
    b = "\n".join(body)
    prog = "{} {{\n{}\n}}".format(s, b)
    return prog


def emit(name, length):
    length = length // 4
    en = "en"
    inps = []
    outs = []
    body = []
    ty = "i8<4>"

    inps.append(expr(en, "bool"))
    for i in range(length):
        pa = port("a", i, ty)
        pb = port("b", i, ty)
        py = port("y", i, ty)
        a = fmt("a", i)
        b = fmt("b", i)
        y = fmt("y", i)
        t0 = fmt("t", 3 * i)
        t1 = fmt("t", 3 * i + 1)
        t2 = fmt("t", 3 * i + 2)
        inps.append(pa)
        inps.append(pb)
        outs.append(py)
        body.append(reg(t0, a, en, ty))
        body.append(reg(t1, b, en, ty))
        body.append(add(t2, t0, t1, ty))
        body.append(reg(y, t2, en, ty))

    return prog(name, inps, outs, body)


def parse_args():
    parser = argparse.ArgumentParser(description="generate vector programs")
    parser.add_argument("-n", help="function name", type=str)
    parser.add_argument("-l", help="length of vector", type=int)
    parser.add_argument("-o", help="output file", type=str)
    args = parser.parse_args()
    if not isinstance(args.n, str):
        print("Error: parsing name")
        raise argparse.ArgumentTypeError
    if not isinstance(args.l, int):
        print("Error: parsing length")
        raise argparse.ArgumentTypeError
    return args.n, args.l, args.o


def vector_add(name, length, output=None):
    if output:
        with open(output, "w") as file:
            file.write(emit(name, length))
    else:
        print(emit(name, length))


if __name__ == "__main__":
    name, length, output = parse_args()
    vector_add(name, length, output)
