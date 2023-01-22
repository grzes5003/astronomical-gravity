import numpy as np


def line():
    # (x, y, z, vx, vy, vz, mass, radius)
    return (*(np.random.rand(1, 3) * 10_000).ravel().tolist(),
            *(np.random.rand(1, 3) * 1000).ravel().tolist(),
            np.random.uniform(1000, 10_000), np.random.uniform(10, 10_000))


def write_stars(path: str, rows: int):
    with open(path, 'w') as file:
        file.write('x,y,z,vx,vy,vz,mass,radius\n')
        for _ in range(rows):
            file.write((str(line())[1:-1]).replace(' ', '') + '\n')


if __name__ == '__main__':
    np.random.seed(42)
    # max for 2 procs 182 -> max send 92
    input_size = 182

    write_path = f'resources/result_{input_size}.csv'
    write_stars(write_path, input_size)
