use anyhow::Result;
use clap::{Parser, ValueEnum};

use candle_transformers::models::quantized_rwkv_v5::Model as Q5; // Import model quantized rwkv v5
use candle_transformers::models::quantized_rwkv_v6::Model as Q6; // Import model quantized rwkv v6
use candle_transformers::models::rwkv_v5::{Config, Model as M5, State, Tokenizer}; // Import model rwkv v5
use candle_transformers::models::rwkv_v6::Model as M6; // Import model rwkv v6

use candle_core::utils::{cuda_is_available, metal_is_available}; // Import fungsi untuk mengecek ketersediaan CUDA atau Metal
use candle_core::{DType, Device, Tensor}; // Import struct Device dan Tensor dari candle_core
use candle_nn::VarBuilder; // Import VarBuilder dari candle_nn
use candle_transformers::generation::LogitsProcessor; // Import LogitsProcessor untuk memproses logit
use hf_hub::{api::sync::Api, Repo, RepoType}; // Import untuk mengambil model dari Hugging Face

// Nilai token EOS untuk mengakhiri kalimat yang dihasilkan oleh model rwkv
const EOS_TOKEN_ID: u32 = 261;

// Enum untuk model rwkv yang tersedia di Hugging Face dan model rwkv yang sudah diquantized (q4k)
enum Model {
    M5(M5),
    Q5(Q5),
    M6(M6),
    Q6(Q6),
}

// Implementasi model rwkv untuk menghasilkan output dari model rwkv yang dipilih oleh pengguna
impl Model {
    fn forward(&self, xs: &Tensor, state: &mut State) -> candle_core::Result<Tensor> {
        match self {
            Self::M5(m) => m.forward(xs, state),
            Self::Q5(m) => m.forward(xs, state),
            Self::M6(m) => m.forward(xs, state),
            Self::Q6(m) => m.forward(xs, state),
        }
    }
}

// Struct TextGeneration untuk menghasilkan output dari model rwkv yang dipilih oleh pengguna
struct TextGeneration {
    model: Model,
    config: Config,
    device: Device,
    tokenizer: Tokenizer,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

// Implementasi TextGeneration untuk menghasilkan output dari model rwkv yang dipilih oleh pengguna
impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    fn new(
        model: Model,
        config: Config,
        tokenizer: Tokenizer,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        repeat_penalty: f32,
        repeat_last_n: usize,
        device: &Device,
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model,
            config,
            tokenizer,
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            device: device.clone(),
        }
    }

    // Fungsi run untuk menghasilkan output dari model rwkv yang dipilih oleh pengguna
    fn run(&mut self, prompt: &str, sample_len: usize) -> Result<()> {
        use std::io::Write;
        let mut tokens = self.tokenizer.encode(prompt)?;
        let mut generated_tokens = 0usize;
        let mut state = State::new(1, &self.config, &self.device)?;
        let mut next_logits = None;
        for &t in tokens.iter() {
            let input = Tensor::new(&[[t]], &self.device)?;
            let logits = self.model.forward(&input, &mut state)?;
            next_logits = Some(logits);
            print!("{}", self.tokenizer.decode(&[t])?)
        }
        std::io::stdout().flush()?;

        let start_gen = std::time::Instant::now();
        for _ in 0..sample_len {
            let logits = match next_logits.as_ref() {
                Some(logits) => logits,
                None => anyhow::bail!("cannot work on an empty prompt"),
            };
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };
            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;
            if next_token == EOS_TOKEN_ID || next_token == 0 {
                break;
            }
            print!("{}", self.tokenizer.decode(&[next_token])?);
            std::io::stdout().flush()?;

            let input = Tensor::new(&[[next_token]], &self.device)?;
            next_logits = Some(self.model.forward(&input, &mut state)?)
        }
        let dt = start_gen.elapsed();
        println!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(())
    }
}

// Enum untuk memilih model rwkv yang tersedia di Hugging Face
#[derive(Parser, ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
enum Which {
    Eagle7b,
    World1b5,
    World3b,
    World6_1b6,
}

// Implementasi Which untuk menampilkan model rwkv yang dipilih oleh pengguna
impl std::fmt::Display for Which {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Which {
    // Implementasi model_id untuk menampilkan model_id dari model rwkv yang dipilih oleh pengguna
    fn model_id(&self) -> &'static str {
        match self {
            Self::Eagle7b => "RWKV/v5-Eagle-7B-HF",
            Self::World1b5 => "RWKV/rwkv-5-world-1b5",
            Self::World3b => "RWKV/rwkv-5-world-3b",
            Self::World6_1b6 => "paperfun/rwkv",
        }
    }

    // Implementasi revision untuk menampilkan revisi dari model rwkv yang dipilih oleh pengguna
    fn revision(&self) -> &'static str {
        match self {
            Self::Eagle7b => "refs/pr/1",
            Self::World1b5 | Self::World3b => "refs/pr/2",
            Self::World6_1b6 => "main",
        }
    }
}

// Implementasi fungsi main
fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    // Parse argumen baris perintah
    let args = Args::parse();
    let _guard = if args.tracing {
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };
    println!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle_core::utils::with_avx(),
        candle_core::utils::with_neon(),
        candle_core::utils::with_simd128(),
        candle_core::utils::with_f16c()
    );
    println!(
        "temp: {:.2} repeat-penalty: {:.2} repeat-last-n: {}",
        args.temperature.unwrap_or(0.),
        args.repeat_penalty,
        args.repeat_last_n
    );

    // Inisialisasi API dan repo
    let start = std::time::Instant::now();
    let api = Api::new()?;
    let repo = api.repo(Repo::with_revision(
        args.model_id
            .unwrap_or_else(|| args.which.model_id().to_string()),
        RepoType::Model,
        args.revision
            .unwrap_or_else(|| args.which.revision().to_string()),
    ));

    // Mendapatkan tokenizer dan konfigurasi model
    let tokenizer = match args.tokenizer {
        Some(file) => std::path::PathBuf::from(file),
        None => api
            .model("lmz/candle-rwkv".to_string())
            .get("rwkv_vocab_v20230424.json")?,
    };
    let config_filename = match args.config_file {
        Some(file) => std::path::PathBuf::from(file),
        None => repo.get("config.json")?,
    };

    // Mendapatkan file bobot model
    let filenames = match args.weight_files {
        Some(files) => files
            .split(',')
            .map(std::path::PathBuf::from)
            .collect::<Vec<_>>(),
        None => {
            if args.quantized {
                vec![match args.which {
                    Which::World1b5 => api
                        .model("lmz/candle-rwkv".to_string())
                        .get("world1b5-q4k.gguf")?,
                    Which::World3b => api
                        .model("lmz/candle-rwkv".to_string())
                        .get("world3b-q4k.gguf")?,
                    Which::Eagle7b => api
                        .model("lmz/candle-rwkv".to_string())
                        .get("eagle7b-q4k.gguf")?,
                    Which::World6_1b6 => repo.get("rwkv-6-world-1b6-q4k.gguf")?,
                }]
            } else {
                vec![match args.which {
                    Which::World1b5 | Which::World3b | Which::Eagle7b => {
                        repo.get("model.safetensors")?
                    }
                    Which::World6_1b6 => repo.get("rwkv-6-world-1b6.safetensors")?,
                }]
            }
        }
    };
    println!("retrieved the files in {:?}", start.elapsed());

    // Inisialisasi tokenizer dan model
    let tokenizer = Tokenizer::new(tokenizer)?;
    let start = std::time::Instant::now();
    let config: Config = serde_json::from_slice(&std::fs::read(config_filename)?)?;
    let device = device(args.cpu)?;
    let model = if args.quantized {
        let filename = &filenames[0];
        let vb =
            candle_transformers::quantized_var_builder::VarBuilder::from_gguf(filename, &device)?;
        match args.which {
            Which::World1b5 | Which::World3b | Which::Eagle7b => Model::Q5(Q5::new(&config, vb)?),
            Which::World6_1b6 => Model::Q6(Q6::new(&config, vb)?),
        }
    } else {
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, DType::F32, &device)? };
        match args.which {
            Which::World1b5 | Which::World3b | Which::Eagle7b => Model::M5(M5::new(&config, vb)?),
            Which::World6_1b6 => Model::M6(M6::new(&config, vb)?),
        }
    };
    println!("loaded the model in {:?}", start.elapsed());

    // Inisialisasi pipeline untuk generasi teks
    let mut pipeline = TextGeneration::new(
        model,
        config,
        tokenizer,
        args.seed,
        args.temperature,
        args.top_p,
        args.repeat_penalty,
        args.repeat_last_n,
        &device,
    );

    // Jalankan generasi teks
    pipeline.run(&args.prompt, args.sample_len)?;
    Ok(())
}