# SBF

Convert a point cloud from CSV to [SBF (simple binary file)][sbf] format.

```shell
$ cat ./data/example.csv | cargo run
```

## Data format

Input CSV: `(x, y, z, Re, Im)` array saved by NumPy `savetxt`.

Output SBF: See [CloudCompare doc][sbf].

[sbf]: https://www.cloudcompare.org/doc/wiki/index.php/SBF
