from z3 import *
import argparse
import sys


def parse(program):
    luts = {}
    dsps = {}
    for line in program.split("\n"):
        if not line:
            continue
        line = line.strip().split(",")
        N = int(line[0])
        tile = line[1]
        if len(line) > 2:
            depends = int(line[2])
        else:
            depends = None

        if tile == "lut":
            luts[N] = depends
        elif tile == "dsp":
            dsps[N] = depends
        else:
            raise Exception()

    return luts, dsps


def generateconstraints(
    solver, namebase, varnum, columns, rows, maxrow, dependency
):
    # Declare variables
    xloc = Int("{}_{}_x".format(namebase, varnum))
    yloc = Int("{}_{}_y".format(namebase, varnum))
    # Row must be between 0 and max row
    solver.add(yloc >= 0)
    solver.add(yloc < maxrow)
    # Column must be an available column
    solver.add(Or(*[xloc == v for v in columns]))

    if dependency is not None:  # we have a dependency
        # get variables
        depxloc = Int("{}_{}_x".format(namebase, dependency))
        depyloc = Int("{}_{}_y".format(namebase, dependency))
        solver.add(xloc == depxloc)  # must be same column
        solver.add(yloc == depyloc + 1)  # row should be + 1
    else:  # no dependency: set row constraints
        solver.add(Or(*[yloc == v for v in rows]))

    return xloc, yloc


def placeonce(inputs, name, columns, rows):
    total_row = max(rows)

    s = Solver()
    s.push()

    locvars = {}
    for k, v in inputs.items():
        locvars[k] = generateconstraints(
            s, name, k, columns, rows, total_row, v
        )
    for k1, v1 in locvars.items():
        for k2, v2 in locvars.items():
            if k1 == k2:
                continue
            s.add(Or(v1[0] != v2[0], v1[1] != v2[1]))

    check = s.check()
    if check == unsat:
        return None
    m = s.model()

    locs = {
        k: (m[v[0]].as_long(), m[v[1]].as_long()) for k, v in locvars.items()
    }

    s.pop()
    return locs


def parse_args():
    parser = argparse.ArgumentParser(description="place")
    parser.add_argument("inp", help="input file", type=str)
    parser.add_argument("-o", dest="out", help="output file", type=str)
    parser.add_argument("-p", dest="prim", help="primitive", type=str)
    args = parser.parse_args()
    if not isinstance(args.inp, str):
        print("Error: missing input file")
        parser.print_help(sys.stderr)
        sys.exit(1)
    if not isinstance(args.prim, str):
        print("Error: missing primitive")
        parser.print_help(sys.stderr)
        sys.exit(1)
    assert args.prim == "dsp" or args.prim == "lut"
    return args.inp, args.out, args.prim


if __name__ == "__main__":
    inp, out, prim = parse_args()
    dsp_col = [0, 1, 2, 3, 4]
    lut_col = [0, 1, 2]
    dsp_col_length = 72
    lut_col_length = 72
    dsp_row = list(range(0, dsp_col_length))
    lut_row = list(range(0, lut_col_length))
    with open(inp, "r") as f:
        luts, dsps = parse(f.read())
        inputs = dsps if prim == "dsp" else luts
        col = dsp_col if prim == "dsp" else lut_col
        row = dsp_row if prim == "dsp" else lut_row
        locs = placeonce(inputs, prim, col, row)
        for k, v in locs.items():
            print("{},{},{}".format(k, v[0], v[1]))
