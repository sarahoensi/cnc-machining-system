// domain//geometry/helix/effective_diameters.rs

use crate::domain::units::Diameter;
use crate::domain::geometry::HelixError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HelixMode {
    Inner,
    Outer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EffectiveDiameter(Diameter);

impl EffectiveDiameter {

    pub fn new(
        mode: HelixMode,
        nominal: Diameter,
        tool: Diameter,
    ) -> Result<Self, HelixError> {

        let d_nom = nominal.mm_value();
        let d_tool = tool.mm_value();
        let tool_radius = d_tool / 2.0;

        let value = match mode {
            HelixMode::Inner => {
                if tool_radius >= d_nom {
                    return Err(HelixError::ToolTooLarge {
                        tool_diameter: d_tool,
                        nominal_diameter: d_nom,
                    });
                }

                d_nom - tool_radius
            }

            HelixMode::Outer => d_nom + tool_radius,
        };

        // Matematisk invariant:
        // Inner: tool_radius < d_nom → result > 0
        // Outer: d_nom > 0 → result > 0
        let diameter =
            Diameter::mm(value)
                .expect("Effective diameter must remain positive");

        Ok(Self(diameter))
    }

    pub fn diameter(self) -> Diameter {
        self.0
    }
}