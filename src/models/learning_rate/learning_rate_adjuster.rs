use crate::models::learning_rate::training_context::TrainingContext;
use crate::models::learning_rate::learning_rate_adjuster_enum::LearningRateAdjusterEnum;
pub trait LearningRateAdjuster: ConvertToLearningRateAdjusterEnum{
    fn adjust(&mut self, context: TrainingContext);
    fn get_learning_rate(&self) -> f64;
}

pub trait ConvertToLearningRateAdjusterEnum{
    fn convert_to_adjuster_enum(&self) -> LearningRateAdjusterEnum;
}
