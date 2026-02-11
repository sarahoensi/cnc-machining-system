// application/finishing/register_finishing_measurement_use_case.rs

use crate::application::shared::AppResult;

use crate::application::finishing::dto::FinishingExecutionOutput;
use crate::application::finishing::finishing_output_mapper::to_output;

use crate::domain::{
    Diameter,
    FinishingExecutionId,
};

use crate::domain::FinishingExecutionRepository;

pub struct RegisterFinishingMeasurementUseCase<R: FinishingExecutionRepository> {
    repo: R,
}


impl<R: FinishingExecutionRepository> RegisterFinishingMeasurementUseCase<R> {

    pub fn new(repo: R) -> Self {
        Self { repo }
    }


    pub fn execute(
    &self,
    id: FinishingExecutionId,
    step_number: u32,
    measurement_mm: f64,
) -> AppResult<FinishingExecutionOutput> {

    let mut execution = self.repo.get(id)?;

    execution.register_measurement(
        step_number,
        Diameter::mm(measurement_mm)?,
    )?;

    self.repo.save(execution.clone())?;

    Ok(to_output(&execution))
}

}
