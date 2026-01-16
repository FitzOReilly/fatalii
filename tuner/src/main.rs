use std::{
    fs::{self, File},
    io::Write,
};

use clap::{Args, Parser, Subcommand};
use eval::HandCraftedEval;
use tuner::{
    error_function::ErrorFunction,
    eval_params::EvalParams,
    feature_evaluator::{default_weights, engine_weights, FeatureEvaluator, WeightVector},
    file_reader,
    optimizer::{self, AdamParams, Checkpoint},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize default evaluation parameters (material values only)
    InitDefaultWeights(InitWeightsArgs),
    /// Initialize evaluation parameters from the engine
    InitEngineWeights(InitWeightsArgs),
    /// Optimize parameter weights
    Optimize(OptimizeArgs),
    /// Print evaluation parameters
    Print(PrintArgs),
}

#[derive(Debug, Args)]
struct InitWeightsArgs {
    #[arg(short, long)]
    weight_file_prefix: String,
}

#[derive(Debug, Args)]
struct OptimizeArgs {
    #[arg(short, long)]
    training_data_file: String,
    #[arg(short, long)]
    weight_file_prefix: String,
    #[arg(short, long)]
    num_epochs: u32,
    #[arg(short, long, default_value_t = 0)]
    start_epoch: u32,
}

#[derive(Debug, Args)]
struct PrintArgs {
    #[arg(short, long)]
    weight_file: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::InitDefaultWeights(args) => {
            init_weights(&args.weight_file_prefix, default_weights())?
        }
        Commands::InitEngineWeights(args) => {
            init_weights(&args.weight_file_prefix, engine_weights())?
        }
        Commands::Optimize(args) => optimize(
            &args.training_data_file,
            &args.weight_file_prefix,
            args.start_epoch,
            args.num_epochs,
        )?,
        Commands::Print(args) => write_weights(&args.weight_file)?,
    };

    Ok(())
}

fn init_weights(weight_file_prefix: &str, weights: WeightVector) -> std::io::Result<()> {
    let checkpoint = Checkpoint {
        weights,
        ..Default::default()
    };
    let serialized = serde_json::to_string(&checkpoint)?;
    let filename = format!("{weight_file_prefix}{:04}.json", 0);
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn optimize(
    training_data_file: &str,
    weight_file_prefix: &str,
    start_epoch: u32,
    num_epochs: u32,
) -> std::io::Result<()> {
    let filename = format!("{weight_file_prefix}{start_epoch:04}.json");
    let contents = fs::read_to_string(filename)?;
    let initial_checkpoint: Checkpoint = serde_json::from_str(&contents)?;
    let mut weights = initial_checkpoint.weights;

    let adam_params = AdamParams {
        ..initial_checkpoint.params
    };

    let k = 1.0;
    let mut error_fn = ErrorFunction::new(k);

    let mut pos_evaluator = HandCraftedEval::new();
    let feature_evaluator = FeatureEvaluator::new();

    let mut training_coeffs =
        file_reader::read_training_data(training_data_file, &mut pos_evaluator, &feature_evaluator);

    optimizer::adam(
        weight_file_prefix,
        &mut weights,
        &mut error_fn,
        &mut training_coeffs,
        adam_params,
        num_epochs as i32,
    )
}

fn write_weights(weight_file: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(weight_file)?;
    let final_checkpoint: Checkpoint = serde_json::from_str(&contents)?;
    let weights = final_checkpoint.weights;
    let eval_params = EvalParams::from(&weights);
    println!("{eval_params}");
    Ok(())
}
