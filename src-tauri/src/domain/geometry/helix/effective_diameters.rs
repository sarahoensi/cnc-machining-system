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
        tool: Diameter
    ) -> Result<Self, HelixError> {
        let d_nom = nominal.mm_value();
        let d_tool = tool.mm_value();

        let value = match mode {
            HelixMode::Inner => {
                if d_tool >= d_nom {
                    return Err(HelixError::ToolTooLarge {
                        tool_diameter: d_tool,
                        nominal_diameter: d_nom,
                    });
                }
                d_nom - d_tool
            }
            HelixMode::Outer => d_nom + d_tool,
        };

        let diameter = Diameter::mm(value)
            .map_err(|_| HelixError::EffectiveDiameterNotPositive { value })?;

        Ok(Self(diameter))
    }

    pub fn diameter(self) -> Diameter {
        self.0
    }
}
