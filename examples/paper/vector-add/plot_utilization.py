import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np


def create_plot(frame, ty):
    indices = df["type"] == ty
    data = df[indices]
    g = sns.catplot(
        data=data,
        x="length",
        hue="backend",
        y="number",
        kind="bar",
        legend_out=False,
    )
    labels = ["base", "base+hint", "reticle"]
    for t, l in zip(g._legend.texts, labels):
        t.set_text(l)
    g._legend.set_title("Backend")
    plt.savefig("{}.pdf".format(ty))


if __name__ == "__main__":
    df = pd.read_csv("util.csv")
    create_plot(df, "dsp")
    create_plot(df, "lut")
    create_plot(df, "reg")
    create_plot(df, "carry")
