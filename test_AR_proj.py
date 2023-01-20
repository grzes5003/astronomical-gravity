#!/usr/bin/env python
import subprocess


def test_equal(_exec: str, _input: str, output: str):
    sums = []
    lengths = []
    for i in [2, 4, 6]:
        open(output, 'w').close()
        test = subprocess.call(["mpiexec", "-np", str(i), _exec, "-f", _input, "-s"])
        with open(output, mode='r') as file:
            content = [float(num) for num in file.read().replace('\n', ',').split(',') if num != '']
            lengths.append(len(content))
            sums.append(sum(content))
    print(sums)
    print(lengths)


if __name__ == '__main__':
    input_filepath = 'resources/result.csv'
    output_filepath = 'output.csv'
    exec_filepath = 'target/debug/AR_proj'
    test_equal(exec_filepath, input_filepath, output_filepath)
