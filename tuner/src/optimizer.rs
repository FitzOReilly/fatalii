use std::{fs::File, io::Write};

use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::{
    error_function::ErrorFunction,
    feature_evaluator::{FeatureEvaluator, WeightVector},
    training::TrainingCoeffs,
};

const STORE_EVERY: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdamParams {
    pub batch_size: usize,
    pub validation_ratio: f64,
    pub learning_rate: f64,
    pub betas: [f64; 2],
    pub epsilon: f64,
    pub epoch: i32,
    pub t: i32,
    pub m: WeightVector,
    pub v: f64,
}

impl Default for AdamParams {
    fn default() -> Self {
        Self {
            batch_size: 32,
            validation_ratio: 0.1,
            learning_rate: 0.001,
            betas: [0.9, 0.999],
            epsilon: 1e-8,
            epoch: 0,
            t: 0,
            m: WeightVector::zeros(),
            v: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub params: AdamParams,
    pub weights: WeightVector,
    pub training_error: Option<f64>,
    pub validation_error: Option<f64>,
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            params: Default::default(),
            weights: WeightVector::zeros(),
            training_error: Default::default(),
            validation_error: Default::default(),
        }
    }
}

pub fn adam(
    weight_file_prefix: &str,
    weights: &mut WeightVector,
    error_fn: &mut ErrorFunction,
    training_coeffs: &mut [TrainingCoeffs],
    params: AdamParams,
    num_epochs: i32,
) -> std::io::Result<()> {
    let mut evaluator = FeatureEvaluator::from(&*weights);

    let mut m = params.m;
    let mut v = params.v;
    let mut t = params.t;
    let batch_size = params.batch_size;
    let validation_ratio = params.validation_ratio;
    let learning_rate = params.learning_rate;
    let betas = params.betas;
    let epsilon = params.epsilon;

    for epoch in params.epoch + 1..=params.epoch + num_epochs {
        training_coeffs.shuffle(&mut rng());
        let mut iter_batch = training_coeffs.chunks(batch_size);
        let batch_count = iter_batch.len();
        let training_batch_count = ((1.0 - validation_ratio) * batch_count as f64) as usize;
        let mut current_batch_count = 0;
        for batch in iter_batch.by_ref() {
            t += 1;
            for pos in batch {
                error_fn.add_datapoint(pos.outcome.into(), evaluator.eval(&pos.coeffs), pos.grad());
            }
            let grad = error_fn.grad();
            let grad_squared = grad.dot(&grad);

            m = betas[0] * m + (1.0 - betas[0]) * grad;
            v = betas[1] * v + (1.0 - betas[1]) * grad_squared;

            let m_hat = m / (1.0 - betas[0].powi(t));
            let v_hat = v / (1.0 - betas[1].powi(t));

            *weights -= learning_rate / (v_hat.sqrt() + epsilon) * m_hat;
            evaluator.update_weights(weights);
            error_fn.clear_batch();
            current_batch_count += 1;
            if current_batch_count >= training_batch_count {
                break;
            }
        }

        let training_pos_count = error_fn.datapoint_count_epoch();
        error_fn.clear();
        for batch in iter_batch {
            for pos in batch {
                error_fn.add_datapoint(pos.outcome.into(), evaluator.eval(&pos.coeffs), pos.grad());
            }
        }
        let validation_pos_count = error_fn.datapoint_count_epoch();
        let validation_error = error_fn.mean_squared_error_epoch();
        println!(
            "Epoch {epoch}, trained with {training_pos_count} random positions, \
            validated with {validation_pos_count} positions, validation error: {validation_error}",
        );

        if (epoch as usize).is_multiple_of(STORE_EVERY) {
            let checkpoint = Checkpoint {
                params: AdamParams {
                    epoch,
                    m,
                    v,
                    t,
                    ..params
                },
                weights: *weights,
                training_error: None,
                validation_error: Some(validation_error),
            };

            let serialized = serde_json::to_string(&checkpoint)?;
            let filename = format!("{weight_file_prefix}{epoch:04}.json");
            let mut file = File::create(filename)?;
            file.write_all(serialized.as_bytes())?;
        }

        error_fn.clear();
    }
    Ok(())
}
