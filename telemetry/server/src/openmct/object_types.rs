use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct OpenMctObject {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
}

pub static OPEN_MCT_OBJECT_TYPES: &'static [OpenMctObject] = &[
    OpenMctObject {
        id: "temperature",
        name: "Temperature",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "thermistor",
        name: "Thermistor",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "acceleration",
        name: "Acceleration",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "pressure",
        name: "Pressure",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "hall_effect",
        name: "Hall Effect",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "displacement",
        name: "Displacement",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "velocity",
        name: "Velocity",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "status",
        name: "Status",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "keyence",
        name: "Keyence",
        icon: "icon-telemetry",
    },
    OpenMctObject {
        id: "brake_feedback",
        name: "Brake Feedback",
        icon: "icon-telemetry",
    },
];
