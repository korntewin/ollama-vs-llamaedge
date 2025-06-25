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

Long story short, LlamaEdge seems to be quite a bit faster.

### TL;DR

| Concurrent Users | Metric             | LlamaEdge | Ollama    | Winner      |
| :--------------- | :----------------- | :-------- | :-------- | :---------- |
| **1**            | req/s              | 1.58      | 0.07      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 635       | 16,100    | **LlamaEdge** |
|                  | 99% Latency (ms)   | 1,000     | 48,000    | **LlamaEdge** |
| **2**            | req/s              | 1.56      | 0.13      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 1,295     | 20,033    | **LlamaEdge** |
|                  | 99% Latency (ms)   | 2,000     | 156,000   | **LlamaEdge** |
| **3**            | req/s              | 1.41      | 0.07      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 2,179     | 65,266    | **LlamaEdge** |
|                  | 99% Latency (ms)   | 5,000     | 205,905   | **LlamaEdge** |

LlamaEdge has significantly higher performance. It serves responses over 10 times faster than Ollama on average.
Here's the full data from my runs.

### 🚀 LlamaEdge

LlamaEdge handled significantly more requests and was much faster on average.

- 1 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           284 |         1 (0%) |     1.58 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      635.48 |        213 |       1,225 |        700
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    700 |    800 |    900 |   1000 |   1000 |   1000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    283 [200], 1 [0]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    283 [200], 1 [0]
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           280 |         1 (0%) |     1.56 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1295 |        269 |       2,658 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   2000 |   2000 |   2000 |   2658 |   2658
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    279 [200], 1 [0]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    279 [200], 1 [0]
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           253 |         1 (0%) |     1.41 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        2179 |        368 |       5,599 |      2,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   2000 |   3000 |   4000 |   5000 |   5599 |   5599
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                    1 [0], 252 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                    1 [0], 252 [200]
```

### 🐢 Ollama

Ollama's performance is detailed below.
- 1 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |            13 |         0 (0%) |     0.07 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |       16100 |      4,564 |      48,312 |     12,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |  12000 |  16000 |  48000 |  48000 |  48000 |  48000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                            13 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                            13 [200] 
```

- 2 concurrent users
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |            23 |         0 (0%) |     0.13 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |       20033 |      2,483 |     156,467 |     11,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |  11000 |  15000 | 156000 | 156000 | 156000 | 156000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                            23 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                            23 [200]
```

- 2 concurrent users
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |            12 |         0 (0%) |     0.07 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |       65266 |      9,936 |     205,905 |     55,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |  55000 |  69000 | 205905 | 205905 | 205905 | 205905
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                            12 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                            12 [200]
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
