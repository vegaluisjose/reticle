import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np


def plot(data, ty, y):
    g = sns.catplot(
        data=data,
        x="length",
        hue="backend",
        y=y,
        kind="bar",
        legend_out=False,
    )
    labels = ["base", "base+hint", "reticle"]
    for t, l in zip(g._legend.texts, labels):
        t.set_text(l)
    g._legend.set_title("Backend")
    plt.title(ty)
    plt.tight_layout()
    plt.savefig("{}.pdf".format(ty))


def plot_util(data, ty):
    indices = data["type"] == ty
    data = data[indices]
    plot(data, ty, "number")


def plot_time(data, ty):
    plot(data, ty, "time")


if __name__ == "__main__":
    util = pd.read_csv("util.csv")
    runtime = pd.read_csv("runtime.csv")
    compiler = pd.read_csv("compiler.csv")
    plot_util(util, "dsp")
    plot_util(util, "lut")
    plot_util(util, "reg")
    plot_util(util, "carry")
    plot_time(runtime, "runtime")
    plot_time(compiler, "compiler")
