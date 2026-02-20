use crate::domain::units::Diameter;
use crate::domain::GeometryError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HelixMode {
    Inner,
    Outer,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EffectiveDiameter(Diameter);

impl EffectiveDiameter {
    pub fn new(mode: HelixMode, nominal: Diameter, tool: Diameter) -> Result<Self, GeometryError> {
        let d_nom = nominal.mm_value();
        let d_tool = tool.mm_value();

        let value = match mode {
            HelixMode::Inner => {
                if d_tool >= d_nom {
                    return Err(GeometryError::InvalidHelix);
                }
                d_nom - d_tool
            }
            HelixMode::Outer => d_nom + d_tool,
        };

        let diameter = Diameter::mm(value).map_err(|_| GeometryError::OutOfRange)?;

        Ok(Self(diameter))
    }

    pub fn diameter(self) -> Diameter {
        self.0
    }
}
