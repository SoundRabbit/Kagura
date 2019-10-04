pub struct PannerNode {
    pub cone_inner_angle: Option<f64>,
    pub cone_outer_angle: Option<f64>,
    pub cone_outer_gain: Option<f64>,
    pub distance_model: Option<DistanceModelType>,
    pub max_distance: Option<f64>,
    pub orientation: Option<(f64, f64, f64)>,
    pub panning_model: Option<PanningModelType>,
    pub position: Option<(f64, f64, f64)>,
    pub ref_distance: Option<(f64, f64, f64)>,
    pub rolloff_factor: Option<f64>,
}

pub enum DistanceModelType {
    Linear,
    Inverse,
    Exponential,
}

pub enum PanningModelType {
    Equalpower,
    Hrtf,
}

impl PannerNode {
    pub fn new() -> Self {
        Self {
            cone_inner_angle: None,
            cone_outer_angle: None,
            cone_outer_gain: None,
            distance_model: None,
            max_distance: None,
            orientation: None,
            panning_model: None,
            position: None,
            ref_distance: None,
            rolloff_factor: None,
        }
    }

    pub fn cone_inner_angle(mut self, value: f64) -> Self {
        self.cone_inner_angle = Some(value);
        self
    }

    pub fn cone_outer_angle(mut self, value: f64) -> Self {
        self.cone_outer_angle = Some(value);
        self
    }

    pub fn cone_outer_gain(mut self, value: f64) -> Self {
        self.cone_outer_gain = Some(value);
        self
    }

    pub fn distance_model(mut self, value: DistanceModelType) -> Self {
        self.distance_model = Some(value);
        self
    }

    pub fn max_distance(mut self, value: f64) -> Self {
        self.max_distance = Some(value);
        self
    }

    pub fn orientation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.orientation = Some((x, y, z));
        self
    }

    pub fn panning_model(mut self, value: PanningModelType) -> Self {
        self.panning_model = Some(value);
        self
    }

    pub fn position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.position = Some((x, y, z));
        self
    }

    pub fn ref_distance(mut self, x: f64, y: f64, z: f64) -> Self {
        self.ref_distance = Some((x, y, z));
        self
    }

    pub fn rolloff_factor(mut self, value: f64) -> Self {
        self.rolloff_factor = Some(value);
        self
    }
}
