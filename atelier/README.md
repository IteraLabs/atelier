# atelier

This is the core module, sort of the orchestrator. 

# Usage

## Local clone

Clone the repository

```shell
git clone
cd atelier
cargo run \
    -- --template "atelier-sync/templates/single_orderbook.toml" \
    --output-dir "./examples"
```

## Docker (recommended)

If you are using a mac with Apple sillicon, you just need to build with `--platform linux/amd64` in order to cross compile, within the OSx system, the linux vm in the container, otherwise just do not include it.

```shell
docker build \
    --platform linux/amd64 \
    --target runner \
    --file .Dockerfile \
    --tag atelier-torch \
    --no-cache . 
```

the `builder` stage, to compile the rust binary, and the `runner` stage to have a 
minimalistic container to expose a service provided by the binary execution.

Generating results by running the containerized atelier.

```shell
docker run \
    --platform linux/amd64 \
    atelier-torch \ 
    --template "templates/single_orderbook.toml" \
    --output-dir "."
```

