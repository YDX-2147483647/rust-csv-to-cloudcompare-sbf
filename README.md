# SBF

Convert a point cloud from CSV to [SBF (simple binary file)][sbf] format.

```shell
$ sbf --input ./data/example.csv --output ./out/example.sbf
Writing to "./out/example.sbf" (text metadata) and "./out/example.sbf.data" (binary data).
Successfully read and convert 10 points.
```

## Data format

Input CSV: `(x, y, z, Re, Im)` array saved by NumPy `savetxt`.

Output SBF: See [CloudCompare doc][sbf].

## Development

```shell
watchexec --watch src -- 'rm out/example.sbf* && cargo run -- -i./data/example.csv -o ./out/example.sbf'
```

[sbf]: https://www.cloudcompare.org/doc/wiki/index.php/SBF
