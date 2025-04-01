# benches

Benchmarks to attribute the resource utilization when running a relevant processes.

## Single order book generation

For both sides of the orderbook, these are the boundaries to the pseudo-random
generation. 

### Params

- Levels: [2, 20, 50, 100]
- Orders: [(5,10), (100, 200), (500, 700), (1000, 1300)]

e.g. There will be a case where the generated orderbook will have 2 levels per side, and a random number (Uniformly distributed) of orders from 
2 up to 10. Or, the other extreme, 100 levels for each side, and, for each level, an amount of orders that could go from 1,000 up to 1,300 orders
per level, thus, it will be measured the time it takes to generate an orderbook with up to 1,300,000 randomly generated orders in it.

To run the benchmark

```shell
cargo bench --color=never 2>&1 | tee benches/orderbook_benchmark.txt
```
and this is an example of the output, as saved in the `orderbook_benchmark_raw.txt` file

```shell

```

### Plots

Criterion.rs automatically generates detailed plots as part of its benchmarking process. Here's how to access and use them:

Using macOS.

```
open target/criterion/report/index.html
```

Using Linux.

```
xdg-open target/criterion/report/index.html
```

## PDF and SD

Solarized dark             |  Solarized Ocean
:-------------------------:|:-------------------------:
![shallow](assets/images/benches/orderbook_generation/b_l_100_b_o_Some((1000, 1300))_a_l_100_a_o_Some((1000, 1300))_1.svg)  |  
![deep](assets/images/benches/orderbook_generation/b_l_100_b_o_Some((1000, 1300))_a_l_100_a_o_Some((1000, 1300))_2.svg)

<details> 
<summary>SVG code</summary>
```
@b_l_100_b_o_Some((1000, 1300))_a_l_100_a_o_Some((1000, 1300)).svg
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="121px" height="81px" viewBox="-0.5 -0.5 121 81" style="background-color: rgb(255, 255, 255);">
    <defs/>
    <g>
        <ellipse cx="60" cy="40" rx="60" ry="40" fill="#ffffff" stroke="#000000" pointer-events="all"/>
    </g>
</svg>
@sample.svg
```

</details>


### Cases

|    Case    | Levels per side | Orders per level | Median Timen to Execute (Milliseconds) |
|:----------:|:---------------:|:----------------:|:--------------------------------------:|
| Shallowest |        2        |  (1,000 : 1,300) |                  0.002                 |
| Deepest    |       100       |  (1,000 : 1,300) |                  11.78                 |

