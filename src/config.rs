use tracing::{error, warn};

/// The type used for most global calculations. Should probably either be `f32` or `f64`.
pub type Float = f32;
pub static FLOAT_ERR: Float = 0.00001;

pub trait SignCheckable {
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_zero(&self) -> bool;
}

impl SignCheckable for Float {
    fn is_negative(&self) -> bool {
        self < &((-1 as Float) * FLOAT_ERR)
    }

    fn is_positive(&self) -> bool {
        self > &FLOAT_ERR
    }

    fn is_zero(&self) -> bool {
        !(self.is_negative()) && !(self.is_positive())
    }
}

static MIRTH_CONFIG: Config = Config {
    acceleration_structure: AccelerationStructure {
        kind: AccStructureKind::BBH,
        axis_selection_method: AccStructureAxisSelectionMethod::Random,
    },
};

pub fn validate_config() {
    if (matches!(
        MIRTH_CONFIG.acceleration_structure.kind,
        AccStructureKind::BBH
    ) && !matches!(
        MIRTH_CONFIG.acceleration_structure.axis_selection_method,
        AccStructureAxisSelectionMethod::LargestExtent
    )) {
        warn!("using 'BBH' acceleration structure with suboptimal axis determination method ('LargestExtent' is the intended method)");
    }
}

struct Config {
    acceleration_structure: AccelerationStructure,
}

struct AccelerationStructure {
    kind: AccStructureKind,
    axis_selection_method: AccStructureAxisSelectionMethod,
}

enum AccStructureKind {
    Nothing,
    BBH,
}

enum AccStructureAxisSelectionMethod {
    Random,
    Alternating,
    LargestExtent,
}
