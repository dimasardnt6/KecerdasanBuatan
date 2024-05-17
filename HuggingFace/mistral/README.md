# mistral

Run as Server

CUDA support : Compute Capability >= 8.0

```powershell
$env:RUST_BACKTRACE = 1
cargo run --features cuda -- --prompt "Here is a sample quick sort implementation in rust " -n 400
```

<https://github.com/huggingface/candle/issues/353>
This is likely because your gpu is not recent enough to support bf16 and mistral is a bf16 model.

We require a compute_cap of at least 8.0 to enable the bf16 support but the RTX 2080 only has support for compute cap 7.5 so you will need a more recent GPU to run the bf16 based models.
