import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import matplotlib as matplt
import numpy as np


def filter(data, column, value):
    return data.loc[data[column] == value]


def tolist(data, column):
    return [v for _, v in data.to_dict()[column].items()]


def formatter():
    mkfunc = (
        lambda x, pos: "%1.1fM" % (x * 1e-6)
        if x >= 1e6
        else "%1.1fK" % (x * 1e-3)
        if x >= 1e3
        else "%1.1f" % x
    )
    return matplt.ticker.FuncFormatter(mkfunc)


def plot_dsp(length, baseline, optimized):
    plt.figure()
    ax = plt.gca()
    ax.scatter(np.arange(len(length)), baseline, marker="o", s=75)
    ax.scatter(np.arange(len(length)), optimized, marker="^", s=75)
    ax.set_xticks(np.arange(len(length)))
    ax.set_xticklabels(length)
    ax.set_xlabel("Loop bound (N)")
    ax.set_ylabel("DSPs used")
    ax.grid(ls="--")
    ax.legend(["behavioral, scalar", "structural, vectorized"])
    plt.tight_layout()
    plt.savefig("overview_dsp.pdf")


def plot_lut(length, baseline, optimized):
    plt.figure()
    ax = plt.gca()
    ax.scatter(np.arange(len(length)), baseline, marker="o", s=75)
    ax.scatter(np.arange(len(length)), optimized, marker="^", s=75)
    ax.set_xticks(np.arange(len(length)))
    ax.set_xticklabels(length)
    ax.set_xlabel("Loop bound (N)")
    ax.set_ylabel("LUTs used")
    ax.grid(ls="--")
    ax.yaxis.set_major_formatter(formatter())
    ax.legend(["behavioral, scalar", "structural, vectorized"])
    plt.tight_layout()
    plt.savefig("overview_lut.pdf")


if __name__ == "__main__":
    util = pd.read_csv("../vector-add/util.csv")
    util = filter(util, "backend", "dsp")
    dsp_util = filter(util, "type", "dsp")
    lut_util = filter(util, "type", "lut")
    dsp_length = tolist(dsp_util, "length")
    dsp_baseline = tolist(dsp_util, "number")
    dsp_optimized = [x // 4 for x in dsp_length]
    lut_length = tolist(lut_util, "length")
    lut_baseline = tolist(lut_util, "number")
    lut_optimized = len(lut_baseline) * [0]
    plot_dsp(dsp_length, dsp_baseline, dsp_optimized)
    plot_lut(lut_length, lut_baseline, lut_optimized)
