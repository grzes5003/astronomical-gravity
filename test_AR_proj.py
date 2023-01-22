#!/usr/bin/env python
import subprocess


def test_equal(_exec: str, _input: str, output: str):
    """
    Output for result_100.csv
    t=0.004671937;s=100;p=1;f=result_100.csv
    t=0.00329687;s=100;p=2;f=result_100.csv
    t=0.002429254;s=100;p=4;f=result_100.csv
    t=0.002323776;s=100;p=6;f=result_100.csv
    [1475902.8668500006, 1475902.8668500006, 1475902.8668500006, 1475902.8668500006]
    [300, 300, 300, 300]
    """
    sums = []
    lengths = []
    for i in [1, 2, 4, 6]:
        open(output, 'w').close()
        test = subprocess.call(["mpiexec", "-np", str(i), _exec, "-f", _input, "-s"])
        with open(output, mode='r') as file:
            content = [float(num) for num in file.read().replace('\n', ',').split(',') if num != '']
            lengths.append(len(content))
            sums.append(sum(content))
    print(sums)
    print(lengths)
    assert sums.count(sums[0]) == len(sums)
    assert lengths.count(lengths[0]) == len(lengths)


if __name__ == '__main__':
    input_filepath = 'resources/result_100.csv'
    output_filepath = 'output.csv'
    exec_filepath = 'target/debug/AR_proj'
    test_equal(exec_filepath, input_filepath, output_filepath)
