use crate::{
    eval_coeffs::EvalCoeffs,
    feature_evaluator::{EvalType, WeightVector},
};

pub struct ErrorFunction {
    k: EvalType,
    sum_of_squared_errors_epoch: f64,
    datapoint_count_epoch: usize,
    sum_of_squared_errors_batch: f64,
    datapoint_count_batch: usize,
    grad: WeightVector,
}

impl ErrorFunction {
    pub fn new(k: EvalType) -> Self {
        Self {
            k,
            sum_of_squared_errors_epoch: 0.0,
            datapoint_count_epoch: 0,
            sum_of_squared_errors_batch: 0.0,
            datapoint_count_batch: 0,
            grad: WeightVector::from_element(0.0),
        }
    }

    pub fn clear(&mut self) {
        self.sum_of_squared_errors_epoch = 0.0;
        self.datapoint_count_epoch = 0;
        self.clear_batch();
    }

    pub fn clear_batch(&mut self) {
        self.sum_of_squared_errors_batch = 0.0;
        self.datapoint_count_batch = 0;
        for g in self.grad.iter_mut() {
            *g = 0.0;
        }
    }

    pub fn add_datapoint(&mut self, outcome: EvalType, eval: EvalType, grad_coeffs: &EvalCoeffs) {
        let sigmoid = self.sigmoid(eval);
        let squared_error = (outcome - sigmoid).powi(2);
        self.sum_of_squared_errors_epoch += squared_error;
        self.datapoint_count_epoch += 1;
        self.sum_of_squared_errors_batch += squared_error;
        self.datapoint_count_batch += 1;
        let grad_sigmoid = self.k * sigmoid * (1.0 - sigmoid);
        let outer_grad = (outcome - sigmoid) * grad_sigmoid;
        for (_row, col, coeff) in grad_coeffs.coeff_vec.triplet_iter() {
            self.grad[(2 * col, 0)] += outer_grad * grad_coeffs.mg_factor * *coeff as EvalType;
            self.grad[(2 * col + 1, 0)] += outer_grad * grad_coeffs.eg_factor * *coeff as EvalType;
        }
    }

    pub fn datapoint_count_epoch(&self) -> usize {
        self.datapoint_count_epoch
    }

    pub fn mean_squared_error_epoch(&self) -> f64 {
        self.sum_of_squared_errors_epoch / self.datapoint_count_epoch as f64
    }

    pub fn datapoint_count_batch(&self) -> usize {
        self.datapoint_count_batch
    }

    pub fn mean_squared_error_batch(&self) -> f64 {
        self.sum_of_squared_errors_batch / self.datapoint_count_batch as f64
    }

    pub fn grad(&self) -> WeightVector {
        -2.0 * self.grad / self.datapoint_count_batch as EvalType
    }

    fn sigmoid(&self, e: EvalType) -> EvalType {
        1.0 / (1.0 + (-self.k * e).exp())
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorFunction;

    #[test]
    fn sigmoid_within_range_and_increasing() {
        let err_fn = ErrorFunction::new(1.0);
        let mut prev = -1.0;
        let mut s;
        for x in -10..=10 {
            s = err_fn.sigmoid(x as f64);
            assert!(s >= 0.0);
            assert!(s <= 1.0);
            assert!(s > prev);
            prev = s;
        }
    }
}
