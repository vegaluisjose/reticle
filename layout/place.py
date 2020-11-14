from z3 import *
import argparse
import sys

# slack factor: use lists for rows like for cols
#   careful with depends; needs to override off-limits rows

# iterative shrinking


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


def placeonce(program, lutcolumns, dspcolumns, lutrows, dsprows):
    lutspercol = max(lutrows)
    dspspercol = max(dsprows)
    luts, dsps = parse(program)

    s = Solver()
    s.push()

    lutvars = {}
    for k, v in luts.items():
        lutvars[k] = generateconstraints(
            s, "lut", k, lutcolumns, lutrows, lutspercol, v
        )
    for k1, v1 in lutvars.items():
        for k2, v2 in lutvars.items():
            if k1 == k2:
                continue
            s.add(Or(v1[0] != v2[0], v1[1] != v2[1]))

    dspvars = {}
    for k, v in dsps.items():
        dspvars[k] = generateconstraints(
            s, "dsp", k, dspcolumns, dsprows, dspspercol, v
        )
    for k1, v1 in dspvars.items():
        for k2, v2 in dspvars.items():
            if k1 == k2:
                continue
            s.add(Or(v1[0] != v2[0], v1[1] != v2[1]))

    check = s.check()
    if check == unsat:
        return None
    m = s.model()

    lutlocs = {
        k: (m[v[0]].as_long(), m[v[1]].as_long()) for k, v in lutvars.items()
    }
    dsplocs = {
        k: (m[v[0]].as_long(), m[v[1]].as_long()) for k, v in dspvars.items()
    }

    s.pop()
    return lutlocs, dsplocs


def iterate(locs, placefunc, slack):

    minx = 0
    miny = 0
    while True:

        # for k,v in locs.items():
        #    print("{}: ({}, {})".format(k, v[0], v[1]))

        # Max x,y for last placement
        maxx = max([x[0] for x in locs.values()])
        maxy = max([x[1] for x in locs.values()])

        # If we have no space left to shrink, we're done
        if (minx + 1 >= maxx) and (miny + 1 >= maxy):
            return locs

        # Get new target bound: halfway to min
        boundx = int((minx + maxx) / 2)
        boundy = int((miny + maxy) / 2)
        listx = list(range(0, boundx))
        listy = list(range(0, boundy, slack))

        # print(maxx, maxy, boundx, boundy)
        # print(listy)

        # Try placement
        res = placefunc(listx, listy)
        if not res:  # Didn't fit
            # Raise min
            minx = boundx
            miny = boundy
        else:  # it fit; iterate
            locs = res


def place(
    program,
    lutcolumns,
    dspcolumns,
    lutspercol,
    dspspercol,
    lutslack=1,
    dspslack=1,
):

    # Do an inital place
    lutrows = list(range(0, lutspercol, lutslack))
    dsprows = list(range(0, dspspercol, dspslack))
    lutlocs, dsplocs = placeonce(
        program, lutcolumns, dspcolumns, lutrows, dsprows
    )

    # iteratively shrink luts
    # print('shrink luts')
    # def placeluts(lutxs, lutys):
    #    res = placeonce(program, lutxs, dspcolumns, lutys, dsprows)
    #    if res is None:
    #        return None
    #    else:
    #        return res[0]
    # lutlocs = iterate(lutlocs, placeluts, lutslack)

    # iteratively shrink dsps
    # print('shrink dsps')
    # def placedsps(dspxs, dspys):
    #    res = placeonce(program, lutcolumns, dspxs, lutrows, dspys)
    #    if res is None:
    #        return None
    #    else:
    #        return res[1]
    # dsplocs = iterate(dsplocs, placedsps, dspslack)

    for k, v in lutlocs.items():
        print("{},{},{}".format(k, v[0], v[1]))

    for k, v in dsplocs.items():
        print("{},{},{}".format(k, v[0], v[1]))


def parse_args():
    parser = argparse.ArgumentParser(description="place")
    parser.add_argument("inp", help="input file", type=str)
    parser.add_argument("-o", dest="out", help="output file", type=str)
    args = parser.parse_args()
    if not isinstance(args.inp, str):
        print("Error: missing input file")
        parser.print_help(sys.stderr)
        sys.exit(1)
    return args.inp, args.out


if __name__ == "__main__":
    inp, out = parse_args()
    LUTCOLUMNS = []
    DSPCOLUMNS = [0, 1, 2, 3, 4]
    DSPSPERCOL = 72
    LUTSPERCOL = 1
    with open(inp, "r") as f:
        place(f.read(), LUTCOLUMNS, DSPCOLUMNS, LUTSPERCOL, DSPSPERCOL)
