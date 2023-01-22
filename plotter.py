import itertools
from dataclasses import dataclass

import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns


@dataclass(repr=True)
class Res:
    time: float
    size: int
    cores: int
    file_name: int

    @classmethod
    def from_str(cls, _input: str):
        items = _input.split(';')
        return cls(time=float(items[0][2:]),
                   size=int(items[1][2:]),
                   cores=int(items[2][2:]),
                   file_name=items[3][2:])


def read_logs(path: str) -> [Res]:
    _start_char = 't'

    with open(path, 'r') as f:
        lines = f.readlines()

    lines = list(itertools.dropwhile(lambda line: line[0] != _start_char, lines))
    lines = [line.replace(' ', '').replace('\n', '') for line in lines]

    return [Res.from_str(line) for line in lines]


def obj2df(results: [Res]) -> pd.DataFrame:
    record = []
    for item in results:
        record.append([item.time, item.size, item.cores, item.file_name])
    return pd.DataFrame(record, columns=['time', 'size', 'cores', 'file_name'])


def plot_eff(df: pd.DataFrame):
    t1 = df[df['cores'] == 1].groupby('size').mean()
    df['speedup'] = t1.loc[df['size']].reset_index()['time'] / df['time']
    df['eff'] = df['speedup'] / df['cores']

    sns.set_theme(style="darkgrid")
    ax = sns.lineplot(x=range(0, 7), y=np.repeat(1, 7), linestyle='--', lw=1)
    sns.pointplot(x='cores', y='eff', data=df, hue='size', errorbar='sd', capsize=.2, ax=ax)

    ax.set(ylabel='Efficiency')
    ax.set_title('Efficiency based on used cores')
    ax.set(xlabel='Number of cores')
    ax.legend(title='Size of problem [n]')

    plt.show()

    sns.set_theme(style="darkgrid")
    ax = sns.pointplot(x="cores", y='speedup', data=df, hue='size', errorbar='sd')
    plt.plot([0, 3], [1, 6], linestyle='--', lw=1)

    ax.set(ylabel='Speedup')
    ax.set_title('Speedup based on used cores')
    ax.set(xlabel='Number of cores')
    ax.legend(title='Size of problem [n]')

    plt.show()


if __name__ == '__main__':
    # path = 'results/result.npy'
    # plot(path)

    path_perf = 'logs/log_22_22_25.log'
    res = read_logs(path_perf)
    df = obj2df(res)

    plot_eff(df)
    ...
