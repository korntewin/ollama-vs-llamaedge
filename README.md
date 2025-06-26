# LlamaEdge vs. Ollama

Hey there! As I've been exploring running LLMs locally and found two interesting candidates:

1. [LlamaEdge]: Written in Rust and runs on a WebAssembly runtime. It's highly portable and offers near-native speed!
2. [Ollama]: Written in Go, has a great user experience and an intuitive CLI.

This repository contains a simple benchmark to compare the performance of [LlamaEdge] against [Ollama].
It's a quick and dirty load test to see which one performs better on my machine.

## Machine Specifications

- **Model:** MacBook Pro
- **Chip:** Apple M4 Max
- **Memory:** 36 GB
- **Operating System:** macOS Sequoia 15.5

## LLM Model

- **Model:** [Llama-3.2-3B-Instruct-Q8_0.gguf]

## The Results!

Long story short, LlamaEdge seems to be a bit faster around 1.1x - 2x.

### TL;DR

| Concurrent Users | Metric             | LlamaEdge | Ollama    | Winner      |
| :--------------- | :----------------- | :-------- | :-------- | :---------- |
| **1**            | req/s              | 1.56      | 0.75      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 642       | 1,350     | **LlamaEdge** |
|                  | 99% Latency (ms)   | 1,000     | 2,000     | **LlamaEdge** |
| **2**            | req/s              | 1.41      | 1.27      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 1,450     | 1,594     | **LlamaEdge** |
|                  | 99% Latency (ms)   | 4,000     | 3,000     | **Ollama**    |
| **3**            | req/s              | 1.32      | 1.03      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 2,343     | 3,006     | **LlamaEdge** |
|                  | 99% Latency (ms)   | 7,986     | 13,000    | **LlamaEdge** |

Here's the full data from my runs.

### 🚀 LlamaEdge

LlamaEdge handled significantly more requests and was much faster on average.

- 1 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           281 |       3 (1.1%) |     1.56 |    0.02
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      642.50 |        262 |       1,240 |        700
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    700 |    800 |   1000 |   1000 |   1000 |   1000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    3 [0], 278 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    3 [0], 278 [200]
```

- 2 concurrent users
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           253 |         1 (0%) |     1.41 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1450 |        260 |       4,348 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   2000 |   3000 |   3000 |   4000 |   4000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    252 [200], 1 [0]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    1 [0], 252 [200]
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           237 |         1 (0%) |     1.32 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        2343 |        293 |       7,986 |      2,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   2000 |   3000 |   6000 |   7986 |   7986 |   7986
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    236 [200], 1 [0]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    236 [200], 1 [0]
```

### 🐢 Ollama

Ollama's performance is detailed below.
- 1 concurrent users
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           135 |         0 (0%) |     0.75 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1350 |        874 |       2,101 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   1000 |   2000 |   2000 |   2000 |   2000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           135 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           135 [200]
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           228 |         0 (0%) |     1.27 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1594 |        792 |       3,207 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   2000 |   3000 |   3000 |   3000 |   3000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           228 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           228 [200]
```

- 3 concurrent users
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           185 |         0 (0%) |     1.03 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        3006 |      1,552 |      15,482 |      2,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   2000 |   3000 |  11000 |  13000 |  15000 |  15000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           185 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           185 [200]
```

## Wanna Run It Yourself?

This repo contains the code to run the benchmarks, which is very simple:

```bash
# on first terminal run
just run-llamaedge-server
# then on another terminal run
just user_count=2 load-test-llamaedge
```

or 

```bash
# on first terminal run
just run-ollama-server
# then on another terminal run
just user_count=2 load-test-ollama
```

**Requirements:**

1. [Cargo]: For *Goose* load testing framework which is written in Rust.
2. [Just]: Improved version of `Make` which is also written in Rust as well 😂.


---

*Disclaimer: Benchmarks can vary a lot based on hardware, the model used, and the specific workload. Your mileage may vary!*


[LlamaEdge]: https://github.com/second-state/LlamaEdge
[Ollama]: https://ollama.com
[Llama-3.2-3B-Instruct-Q8_0.gguf]: https://huggingface.co/second-state/Llama-3.2-3B-Instruct-GGUF
[Cargo]: https://www.rust-lang.org/tools/install
[Just]: https://github.com/casey/just?tab=readme-ov-file#cross-platform
