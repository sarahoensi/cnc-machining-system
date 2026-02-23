// domain/machining_physics/cutting_solver.rs

use crate::domain::{
    machining_physics::{
        CuttingParameters, MachiningPhysicsError, Tool, formulas::*
    },
    units::{ChipLoad, CuttingSpeed, FeedRate, Rpm},
};

pub struct MachiningSolver;

impl MachiningSolver {

    // ---------------------------------------------------------
    // Cutting speed + chip load
    // ---------------------------------------------------------
    pub fn from_speed_and_chip_load(
    cutting_speed: CuttingSpeed,
    chip_load: ChipLoad,
    tool: Tool,
) -> Result<CuttingParameters, MachiningPhysicsError> {

    let rpm =
        rpm_from_cutting_speed(
            cutting_speed,
            tool.diameter(),
        )?;

    let feed =
        feed_from_chip_load(
            chip_load,
            rpm,
            tool.teeth(),
        )?;

    Ok(CuttingParameters::new(
        cutting_speed,
        rpm,
        chip_load,
        feed,
    ))
}

    // ---------------------------------------------------------
    // RPM + feed rate
    // ---------------------------------------------------------
    pub fn from_rpm_and_feed(
        rpm: Rpm,
        feed: FeedRate,
        tool: Tool,
    ) -> Result<CuttingParameters, MachiningPhysicsError> {

        let chip =
            chip_from_feed(
                feed,
                rpm,
                tool.teeth(),
            )?;

        let cutting_speed =
            cutting_speed_from_rpm(
                rpm,
                tool.diameter(),
            )?;

        Ok(CuttingParameters::new(
            cutting_speed,
            rpm,
            chip,
            feed,
        ))
    }
}