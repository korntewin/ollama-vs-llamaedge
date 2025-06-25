model_name := "Llama-3.2-3B-Instruct-f16.gguf"

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
        curl -Lo models/{{model_name}} https://huggingface.co/second-state/Llama-3.2-3B-Instruct-GGUF/blob/main/{{model_name}}; \
    else \
        echo "{{model_name}} already exists."; \
    fi

run-llamaedge-server:
    wasmedge --dir .:. --nn-preload default:GGML:AUTO:models/{{model_name}} \
    llamaedge/llama-api-server.wasm \
    --prompt-template llama-3-chat \
    --ctx-size 128000
