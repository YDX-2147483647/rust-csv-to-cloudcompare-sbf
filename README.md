# SBF

Convert a point cloud from CSV to [SBF (simple binary file)][sbf] format.

```shell
$ cat ./data/example.csv | cargo r -- -o ./out/example.sbf
Write to "./out/example.sbf" (text metadata) and "./out/example.sbf.data" (binary data).
```

## Data format

Input CSV: `(x, y, z, Re, Im)` array saved by NumPy `savetxt`.（因有 UTF-8 / UTF-16 编码问题，使用 stdin 而非 args。）

Output SBF: See [CloudCompare doc][sbf].

[sbf]: https://www.cloudcompare.org/doc/wiki/index.php/SBF
