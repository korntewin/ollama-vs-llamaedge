# LlamaEdge vs. Ollama

Hey there! As I've been exploring running LLMs locally and found two interesting candidates:

1. [LlamaEdge]: Written in Rust and runs on a WebAssembly runtime. It's highly portable and offers near-native speed!
2. [Ollama]: Written in Go, has a great user experience and an intuitive CLI.
3. [LMStudio]: Written in Typescript, has the best UX and eas of useout of these three! 

This repository contains a simple benchmark to compare the performance for [LlamaEdge], [Ollama], and [LMStudio].
It's a quick and dirty load test to see which one performs better on my machine.

## Machine Specifications

- **Model:** MacBook Pro
- **CPU/GPU:** Apple M4 Max
- **RAM:** 36 GB
- **VRAM:** 27 GB
- **Operating System:** macOS Sequoia 15.5

## LLM Model

- **Model:** [Llama-3.2-3B-Instruct-Q8_0.gguf]

## The Results!

The results are interesting! For a single user, LMStudio comes out on top. However, as concurrency increases, Ollama takes the lead, demonstrating better scaling capabilities in this test.

### TL;DR

| Concurrent Users | Metric             | LlamaEdge | Ollama | LMStudio | LMStudio MLX | Winner      |
| :--------------- | :----------------- | :-------- | :-------- | :--- | :--- |:---------- |
| **1**            | req/s              | 1.47      | 1.54      | 1.61 | 1.75 | **LMStudio MLX** |
|                  | Avg Latency (ms)   | 683       | 654       | 623  | 575 | **LMStudio MLX** |
|                  | 99% Latency (ms)   | 1,000     | 788       | 700 | 900 | **LMStudio** |
|                  | Tokens/s           | 73.08     | 78.06     | 74.88 | 86.11 | **LMStudio MLX** |
| **2**            | req/s              | 1.51      | 1.78      | 1.62 | 1.74 | **Ollama**    |
|                  | Avg Latency (ms)   | 1333      | 1130      | 1248 | 1156 | **Ollama**    |
|                  | 99% Latency (ms)   | 2800      | 1,000      | 1941 | 1,000 | **Ollama/LMStudio MLX**    |
|                  | Tokens/s           | 73.69     | 94.44     | 77.39 | 85.14 | **Ollama**    |
| **3**            | req/s              | 1.51      | 2.19      | 1.66 | 1.73 | **Ollama**    |
|                  | Avg Latency (ms)   | 2009      | 1381      | 1831 | 1757 | **Ollama**    |
|                  | 99% Latency (ms)   | 3618      | 1,879      | 2000 | 1,988 | **Ollama**    |
|                  | Tokens/s           | 75.01     | 117.22    | 82.27 | 88.33 | **Ollama**    |

Here's the full data from my runs.

### LlamaEdge

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

### Ollama

Ollama's performance is detailed below.
- 1 concurrent users
```
  === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           277 |         0 (0%) |     1.54 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      653.89 |        612 |         788 |        700
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    700 |    700 |    788 |    788 |    788 |    788
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           277 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           277 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 14050
Total time: 180.00s
Tokens per second: 78.06
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

### LMStudio

- 1 concurrent user
```
=== PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           290 |         0 (0%) |     1.61 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      622.60 |        229 |         735 |        700
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    700 |    700 |    700 |    700 |    700 |    700
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           290 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           290 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 13478
Total time: 180.00s
Tokens per second: 74.88
##################################################
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           291 |         0 (0%) |     1.62 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1248 |        564 |       1,941 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   1000 |   1941 |   1941 |   1941 |   1941
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           291 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           291 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 13930
Total time: 180.00s
Tokens per second: 77.39
##################################################
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           299 |         0 (0%) |     1.66 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1831 |      1,211 |       2,525 |      2,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   2000 |   2000 |   2000 |   2000 |   2525 |   2525
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           299 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           299 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 14809
Total time: 180.00s
Tokens per second: 82.27
##################################################
```

### LMStudio MLX model

- 1 concurrent user
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           315 |         0 (0%) |     1.75 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |      575.00 |         74 |         960 |        600
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |    600 |    600 |    900 |    900 |    960 |    960
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                           315 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                           315 [200] 

##################################################
# Summary                                        #
##################################################
Total completion tokens: 15500
Total time: 180.00s
Tokens per second: 86.11
##################################################
```

- 2 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           314 |         1 (0%) |     1.74 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1156 |        407 |       1,330 |      1,000
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1000 |   1000 |   1000 |   1000 |   1000 |   1000
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                  1 [400], 313 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                  1 [400], 313 [200] 

 === ERRORS ===
 ------------------------------------------------------------------------------
 Count       | Error
 ------------------------------------------------------------------------------
 1             POST chat: 400 Bad Request: chat
 ------------------------------------------------------------------------------

##################################################
# Summary                                        #
##################################################
Total completion tokens: 15326
Total time: 180.00s
Tokens per second: 85.14
##################################################
```

- 3 concurrent users
```
 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 POST chat                |           311 |         1 (0%) |     1.73 |    0.01
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 POST chat                |        1757 |        527 |       1,988 |      1,988
 ------------------------------------------------------------------------------
 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 ------------------------------------------------------------------------------
 POST chat                |   1988 |   1988 |   1988 |   1988 |   1988 |   1988
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes 
 ------------------------------------------------------------------------------
 POST chat                |                                  1 [400], 310 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                  310 [200], 1 [400] 

 === ERRORS ===
 ------------------------------------------------------------------------------
 Count       | Error
 ------------------------------------------------------------------------------
 1             POST chat: 400 Bad Request: chat
 ------------------------------------------------------------------------------

##################################################
# Summary                                        #
##################################################
Total completion tokens: 15900
Total time: 180.00s
Tokens per second: 88.33
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

or

```bash
# For LMStudio, the server must be run manually from LMStudio UI
just user_count=3 load-test-lmstudio
```

**Requirements:**

1. [Cargo]: For *Goose* load testing framework which is written in Rust.
2. [Just]: Improved version of `Make` which is also written in Rust as well 😂.
3. [LMStudio]: LMStudio is currently needed to be installed from official website, no CLI installation so far. After installation, we can easily spinup OpenAI API compatible server selecting `secondstate/Llama-3.2-3B-Instruct-GGUF/Llama-3.2-3B-Instruct-Q8_0.gguf` model.


---

*Disclaimer: Benchmarks can vary a lot based on hardware, the model used, and the specific workload. Your mileage may vary!*


[LlamaEdge]: https://github.com/second-state/LlamaEdge
[Ollama]: https://ollama.com
[Llama-3.2-3B-Instruct-Q8_0.gguf]: https://huggingface.co/second-state/Llama-3.2-3B-Instruct-GGUF
[Cargo]: https://www.rust-lang.org/tools/install
[Just]: https://github.com/casey/just?tab=readme-ov-file#cross-platform
[LMStudio]: https://lmstudio.ai/
