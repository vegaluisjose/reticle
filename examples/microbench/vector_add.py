# def dsp_add_i8v4_r1_r1_r1(a: i8<4>, b: i8<4>, en:bool) -> (y: i8<4>) {
#     t0: i8<4> = reg[0](a, en);
#     t1: i8<4> = reg[0](b, en);
#     t2: i8<4> = add(t0, t1);
#     y: i8<4> = reg[0](t2, en);
# }


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
    i = ",\n".join(inps)
    o = ",\n".join(outs)
    signature = "def {}(\n{})\n->\n({})".format(name, i, o)
    return signature


def prog(name, inps, outs, body):
    s = signature(name, inps, outs)
    b = "\n".join(body)
    prog = "{} {{\n{}\n}}".format(s, b)
    return prog


def emit(name, size):
    en = "en"
    inps = []
    outs = []
    body = []
    ty = "i8<4>"

    inps.append(expr(en, "bool"))
    for i in range(size):
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


if __name__ == "__main__":
    name = "vector_add"
    size = 512
    with open("{}.ret".format(name), "w") as file:
        file.write(emit(name, size))