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

The results are interesting! For a single user, LlamaEdge shows a slight edge in performance. However, as concurrency increases, Ollama takes the lead, demonstrating better scaling capabilities in this test.

### TL;DR

| Concurrent Users | Metric             | LlamaEdge | Ollama    | Winner      |
| :--------------- | :----------------- | :-------- | :-------- | :---------- |
| **1**            | req/s              | 1.47      | 1.15      | **LlamaEdge** |
|                  | Avg Latency (ms)   | 683       | 871       | **LlamaEdge** |
|                  | 99% Latency (ms)   | 1,000     | 1,000     | **Tie**     |
|                  | Tokens/s           | 73.08     | 58.61     | **LlamaEdge** |
| **2**            | req/s              | 1.51      | 1.78      | **Ollama**    |
|                  | Avg Latency (ms)   | 1333      | 1130      | **Ollama**    |
|                  | 99% Latency (ms)   | 2800      | 1000      | **Ollama**    |
|                  | Tokens/s           | 73.69     | 94.44     | **Ollama**    |
| **3**            | req/s              | 1.51      | 2.19      | **Ollama**    |
|                  | Avg Latency (ms)   | 2009      | 1381      | **Ollama**    |
|                  | 99% Latency (ms)   | 3618      | 1879      | **Ollama**    |
|                  | Tokens/s           | 75.01     | 117.22    | **Ollama**    |

Here's the full data from my runs.

### 🚀 LlamaEdge

- 1 concurrent users
```
  === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           264 |         0 (0%) |     1.47 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      683.46 |        245 |       1,466 |        700
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    700 |    800 |   1000 |   1000 |   1000 |   1000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           264 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           264 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 13154
Total time: 180.00s
Tokens per second: 73.08
##################################################
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           272 |         0 (0%) |     1.51 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1333 |        604 |       2,800 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   2000 |   2000 |   2800 |   2800 |   2800
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           272 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           272 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 13265
Total time: 180.00s
Tokens per second: 73.69
##################################################
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           272 |         0 (0%) |     1.51 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        2009 |        545 |       3,618 |      2,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   2000 |   3000 |   3000 |   3618 |   3618 |   3618
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           272 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           272 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 13501
Total time: 180.00s
Tokens per second: 75.01
##################################################

```

### 🐢 Ollama

Ollama's performance is detailed below.
- 1 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           207 |         0 (0%) |     1.15 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      871.19 |        709 |       1,392 |        800
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    800 |    900 |   1000 |   1000 |   1000 |   1000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           207 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           207 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 10550
Total time: 180.00s
Tokens per second: 58.61
##################################################
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           320 |         0 (0%) |     1.78 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1130 |        982 |       1,958 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   1000 |   1000 |   1000 |   1958 |   1958
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           320 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           320 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 17000
Total time: 180.00s
Tokens per second: 94.44
##################################################
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           395 |         0 (0%) |     2.19 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1381 |      1,115 |       1,879 |      1,115
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1115 |   1879 |   1879 |   1879 |   1879 |   1879
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           395 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           395 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 21100
Total time: 180.00s
Tokens per second: 117.22
##################################################
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
