import itertools
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns


class Visualization:

    def __init__(self, dataframe):
        self.cdf = dataframe

    def plot(self):
        sns.set_style('whitegrid')

        mdf = pd.melt(self.cdf, id_vars=['Implementation'], var_name=[
                      'Letter'], value_name='Execution Time (ms)')

        ax = sns.barplot(x="Implementation", y="Execution Time (ms)",
                         hue="Letter", data=mdf, errwidth=0)

        num_locations = len(mdf.Implementation.unique())
        hatches = itertools.cycle(
            ['/', '//', '+', '-', 'x', '\\', '*', 'o', 'O', '.'])
        for i, bar in enumerate(ax.patches):
            if i % num_locations == 0:
                hatch = next(hatches)
            bar.set_hatch(hatch)

        ax.legend(loc='upper center', bbox_to_anchor=(
            0.5, 1.1), ncol=3, fancybox=True, shadow=True)

        plt.show()
