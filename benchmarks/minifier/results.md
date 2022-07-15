# Benchmark results - Minifier

This is a comparison of the performance, using the [minifier](https://crates.io/crates/minifier), for parsing and minifying the first bundle for the starting `quix link` command.

> Crate used: [minifier](https://crates.io/crates/minifier)

## Tools used:

- On Windows:
  - ```powershell
    # Benchmark (Exec. Time) on Windows 11
    Measure-Command { .\quix.exe }
    ```
- On Linux:
  - ```sh
    # Benchmark (Exec. Time) on Linux
    time .\quix.exe
    ```

## Results

| Mode     | Final bundle size | First time execution | Concurrent executions |
| -------- | ----------------- | -------------------- | --------------------- |
| Raw      | **15.2** KB       | **69.18** ms         | **23.41** ms          |
| Minified | **11.7** KB       | **180.72** ms        | **39.05** ms          |

## Gains

| Mode     | Final bundle size | First time execution | Concurrent executions |
| -------- | ----------------- | -------------------- | --------------------- |
| Raw      | **0.0%**          | **0.0%**             | **0.0%**              |
| Minified | **-29.91%**       | **+161.23%**         | **+66.81%**           |

## Conclusion

Not completely sure if the gains are worth it `~30%` less bundle size, but `~160%` increase of execution time, can be worth it supposing the bigger bottleneck we would have is the VTEX IO Link endpoint, in this case probably reducing `~30%` in bigger projects will for sure save more then the time parsing the files.

But in the other scenario, where the API handles bigger bundle sizes with no worries, this minification will not provide results worth adding this complexity layer.

## Final thoughts

Would love to receive feedback on this benchmark, feel free to issue or PR if you have any suggestions, if you are willing to help with the benchmarks, please do so!

### Output samples

- [Bundles](.)
