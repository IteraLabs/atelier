use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub len: i32,
    pub min: f32,
    pub max: f32,
    pub median: f32,
    pub mean: f32,
    pub variance: f32,
    pub skew: f32,
    pub kurtosis: f32,
}

impl Stats {
    pub fn new(data: &[f32]) -> Option<Self> {
        if data.is_empty() || data.len() < 2 {
            return None;
        }

        let n = data.len() as f32;
        let len = n as i32;
        let mean = data.iter().sum::<f32>() / n;
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let s_len = sorted_data.len();
        let min = sorted_data[0].clone();
        let max = sorted_data[s_len - 1].clone();
        let median = if sorted_data.len() % 2 == 0 {
            (sorted_data[s_len / 2 - 1] + sorted_data[s_len / 2]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        };

        let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (n - 1.0);

        let std_dev = variance.sqrt();

        // Calculate moments for skewness and kurtosis
        let m3 = data.iter().map(|x| (x - mean).powi(3)).sum::<f32>() / n;

        let m4 = data.iter().map(|x| (x - mean).powi(4)).sum::<f32>() / n;

        let skew = if std_dev == 0.0 {
            0.0
        } else {
            m3 / std_dev.powi(3)
        };

        let kurtosis = if std_dev == 0.0 {
            0.0
        } else {
            m4 / std_dev.powi(4) - 3.0
        };

        Some(Self {
            len,
            min,
            max,
            median,
            mean,
            variance,
            skew,
            kurtosis,
        })
    }
}
