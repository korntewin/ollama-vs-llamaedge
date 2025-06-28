model_name := "Llama-3.2-3B-Instruct-Q8_0.gguf"
hatch_rate := "0.2"
run_time := "3m"
user_count := "3"

install-llamaedge:
    @if ! command -v wasmedge &> /dev/null; then \
        echo "Installing WasmEdge..."; \
        curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install_v2.sh | bash; \
    else \
        echo "WasmEdge already installed."; \
    fi
    @if [ ! -f "llamaedge/llama-api-server.wasm" ]; then \
        mkdir -p llamaedge; \
        curl -Lo llamaedge/llama-api-server.wasm https://github.com/second-state/LlamaEdge/releases/latest/download/llama-api-server.wasm; \
    else \
        echo "LlamaEdge API server already exists."; \
    fi

install-ollama:
    @if ! command -v ollama &> /dev/null; then \
        echo "Installing ollama..."; \
        if [ "$(uname -s)" = "Darwin" ]; then \
            brew install ollama; \
        else \
            curl -fsSL https://ollama.com/install.sh | sh; \
        fi; \
    else \
        echo "ollama already installed."; \
    fi

download-model:
    @if [ ! -f "models/{{model_name}}" ]; then \
        mkdir -p models; \
        curl -Lo models/{{model_name}} https://huggingface.co/second-state/Llama-3.2-3B-Instruct-GGUF/resolve/main/{{model_name}}; \
    else \
        echo "{{model_name}} already exists."; \
    fi

# Force various parameters to be the same as ollama
run-llamaedge-server: install-llamaedge download-model 
    wasmedge --dir .:. --nn-preload default:GGML:AUTO:models/{{model_name}} \
        llamaedge/llama-api-server.wasm \
        --prompt-template llama-3-chat \
        --port 11401 \
        --ctx-size 8192 \
        --batch-size 1024 \
        --ubatch-size 512 \
        --n-predict 50 \
        --threads 10 \
        --temp 1.0 \
        --top-p 1.0 \
        --model-name llama3

run-ollama-server: install-ollama download-model
    ollama create llama3 -f ollama/Modelfile;
    OLLAMA_HOST=localhost:11400 ollama serve

load-test-llamaedge: 
    MODEL=llamaedge cargo run --release -- -r {{hatch_rate}} --run-time {{run_time}} -u {{user_count}}

load-test-ollama: 
    MODEL=ollama cargo run --release -- -r {{hatch_rate}} --run-time {{run_time}} -u {{user_count}}

load-test-lmstudio: 
    MODEL=lmstudio cargo run --release -- -r {{hatch_rate}} --run-time {{run_time}} -u {{user_count}}