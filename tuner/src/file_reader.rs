use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use eval::{eval::HasMatingMaterial, Eval};
use movegen::{fen::Fen, side::Side};

use crate::{
    feature_evaluator::FeatureEvaluator,
    position_features::EvalType,
    training::{Outcome, TrainingFeatures, TrainingPosition},
};

pub fn read_training_data(
    filename: &str,
    pos_evaluator: &mut (impl Eval + HasMatingMaterial),
    feature_evaluator: &FeatureEvaluator,
) -> Vec<TrainingFeatures> {
    let mut parsed_count = 0;
    let mut training_data = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut s = line.split(" c9 ");
            let short_fen = s.next().unwrap();
            let pos = Fen::shortened_str_to_pos(short_fen).unwrap();
            let outcome = match s.next().unwrap().split('\"').nth(1).unwrap() {
                "1-0" => Outcome::WhiteWin,
                "1/2-1/2" => Outcome::Draw,
                "0-1" => Outcome::BlackWin,
                invalid => panic!("Invalid outcome: {invalid}"),
            };
            let pos_eval = pos_evaluator.eval(&pos);
            let training_pos = TrainingPosition { pos, outcome };
            let training_features = TrainingFeatures::from(&training_pos);
            let feature_eval = feature_evaluator.eval(&training_features.features);

            // Exclude draws by insufficient material
            if pos_eval != 0
                || pos_evaluator.has_mating_material(Side::White)
                    && pos_evaluator.has_mating_material(Side::Black)
            {
                // Validate that the evaluations match
                assert!(
                    ((pos_eval as EvalType) - feature_eval).abs() < 1.0,
                    "Evaluations don't match\nPosition: {short_fen}\n\
                    Position Eval: {pos_eval}\nFeature Eval: {feature_eval}",
                );
                training_data.push(training_features);
            }
            parsed_count += 1;
        }
        let training_count = training_data.len();
        let filtered_count = parsed_count - training_count;
        println!("Positions: {parsed_count} parsed, {filtered_count} filtered out, will use {training_count} for training");
    }
    training_data
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
