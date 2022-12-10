
use tracing::{error, warn};

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
