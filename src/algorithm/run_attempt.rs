use super::measure::{run_measure::Measure, treatment_measure::core::TreatmentMeasure};

pub enum RunAttempt<M, E>
where
    M: Measure,
    E: TreatmentMeasure<M = M>,
{
    Succeeded(E),
    NotCompleted(String),
}

impl<M, E> From<E> for RunAttempt<M, E>
where
    M: Measure,
    E: TreatmentMeasure<M = M>,
{
    fn from(value: E) -> Self {
        RunAttempt::Succeeded(value)
    }
}
