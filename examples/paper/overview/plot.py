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


def plot_dsp(ax, length, baseline, optimized):
    ax.scatter(np.arange(len(length)), baseline, marker="o")
    ax.scatter(np.arange(len(length)), optimized, marker="^")
    ax.set_xticks(np.arange(len(length)))
    ax.set_xticklabels(length)
    ax.set_xlabel("Loop bound (N)")
    ax.set_ylabel("DSPs used")
    ax.grid(ls="--")
    ax.legend(["example", "optimized"])


def plot_lut(ax, length, baseline, optimized):
    ax.scatter(np.arange(len(length)), baseline, marker="o")
    ax.scatter(np.arange(len(length)), optimized, marker="^")
    ax.set_xticks(np.arange(len(length)))
    ax.set_xticklabels(length)
    ax.set_xlabel("Loop bound (N)")
    ax.set_ylabel("LUTs used")
    ax.grid(ls="--")
    ax.yaxis.set_major_formatter(formatter())
    ax.legend(["example", "optimized"])


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
    lut_optiimzed = len(lut_baseline) * [0]
    f, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 5))
    plot_dsp(ax1, dsp_length, dsp_baseline, dsp_optimized)
    plot_lut(ax2, lut_length, lut_baseline, lut_optiimzed)
    plt.tight_layout(pad=2)
    plt.savefig("overview.pdf")
