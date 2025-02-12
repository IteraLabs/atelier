//! # atelier-visuals
//!
//! Data visualization components.
//!
/// Orderbook static and dynamic visualization.
///

#[derive(Debug, Copy, Clone)]
pub struct Data {
    
}

#[derive(Debug, Copy, Clone)]
pub enum ChartType {
    Scatter,
    Line,
    Bar,
}

pub struct Plot {
    data: Option<(Vec<f32>, Vec<f32>)>,
    chart_type: Option<ChartType>,
}

impl Plot {
    
    pub fn new() -> Plot {

        Plot{
            data: None,
            chart_type: None,
        }
    }

    pub fn data(mut self, data: (Vec<f32>, Vec<f32>)) -> Self {
        self.data = Some(data);
        self
    }

    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = Some(chart_type);
        self
    }
}

