use crate::orderbooks::Orderbook;
use std::{error::Error, fs, io::Write, io::BufReader};
use csv::{Writer, Reader};
use toml;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub index: Vec<u32>,
    pub features: Vec<Vec<f64>>,
    pub target: Vec<f64>,
}

#[derive(Debug)]
pub struct DatasetBuilder {
    index: Option<Vec<u32>>,
    features: Option<Vec<Vec<f64>>>,
    target: Option<Vec<f64>>,
    auto_index: bool,
}

impl DatasetBuilder {

    pub fn new() -> Self {
        DatasetBuilder {
            index: None,
            features: None,
            target: None,
            auto_index: true
        }
    }

    pub fn index(mut self, index: Vec<u32>) -> Self {
        self.index = Some(index);
        self
    }

    pub fn features(mut self, features: Vec<Vec<f64>>) -> Self {
        self.features = Some(features);
        self
    }

    pub fn target(mut self, target: Vec<f64>) -> Self {
        self.target = Some(target);
        self
    }

    pub fn disable_auto_index(mut self) -> Self {
        self.auto_index = false;
        self
    }

    pub fn build(self) -> Result<Dataset, String> {
    
        let features = self.features.ok_or("Missing features")?;
        let target = self.target.ok_or("Missing target")?;

        // Validate features and target have the same length
        
        if features.len() != target.len() {
            return Err(format!(
                "features and target length mismatch: {:?} vs {:?}",
                features.len(),
                target.len()));
        }
            
        // Validate all feature vectors have the same length

        if !features.is_empty() {
            // First feature is the expectation criteria 
            let expected_feature_len = features[0].len();

            for (i, feature_vec) in features.iter().enumerate() {
                if feature_vec.len() != expected_feature_len {
                    return Err(format!(
                        "feature vector at index {:?} has length {:?}, expected {:?}",
                    i, feature_vec.len(), expected_feature_len));
                }
            }
        }

        // If index is not provided, create one
        let index = match self.index {
            Some(idx) => {
                if idx.len() != features.len() {
                    return Err(format!(
                        "Index length {:?} doesn't match data length {:?}",
                        idx.len(), features.len()));
                }
                idx
            }
            None => {
                if self.auto_index {
                    (0..features.len() as u32).collect()
                } else {
                    Vec::new()
                }
            }
        };
       
        Ok(Dataset {
            index,
            features,
            target,
        })
    }
}

impl Dataset {

    pub fn new() -> DatasetBuilder {
        DatasetBuilder::new()
    }

    pub fn from_vectors(
        features: Vec<Vec<f64>>,
        target: Vec<f64>
    ) -> Result<Self, String> {
        
        Self::new().features(features).target(target).build()

    }

    pub fn from_pairs(&self) -> Vec<(Vec<f64>, f64)> {
        self.features
            .iter()
            .zip(self.target.iter())
            .map(|(features, &target)| (features.clone(), target))
            .collect()
    }

    pub fn get_pairs(&self) -> Vec<(Vec<f64>, f64)> {
        self.features
            .iter()
            .zip(self.target.iter())
            .map(|(features, &target)| (features.clone(), target))
            .collect()
    }

    pub fn get_parirs_ref(&self) -> Vec<(&Vec<f64>, f64)> {
        self.features
            .iter()
            .zip(self.target.iter())
            .map(|(features, &target)| (features, target))
            .collect()
    }


    pub fn len(&self) -> usize {
        self.features.len()
    }

     pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    pub fn feature_count(&self) -> usize {
        self.features.first().map_or(0, |f| f.len())
    }

    pub fn get_features(&self) -> &Vec<Vec<f64>> {
        &self.features
    }

    pub fn get_target(&self) -> &Vec<f64> {
        &self.target
    }

    pub fn get_index(&self) -> &Vec<u32> {
        &self.index
    }

    pub fn get_sample(&self, idx: usize) -> Option<(&Vec<f64>, f64)> {
        if idx < self.len() {
            Some((&self.features[idx], self.target[idx]))
        } else {
            None
        }
    }

    pub fn get_sample_by_index(&self, index_value: u32) -> Option<(&Vec<f64>, f64)> {
        self.index
            .iter()
            .position(|&idx| idx == index_value)
            .and_then(|pos| self.get_sample(pos))
    }

    pub fn to_tensor_format(&self) -> (Vec<Vec<f64>>, Vec<f64>) {
        (self.features.clone(), self.target.clone())
    }

    pub fn shift_features(&self) -> Dataset {
        if self.features.len() < 2 {
            return Dataset {
                index: Vec::new(),
                features: Vec::new(),
                target: Vec::new(),
            };
        }

        // Shift features forward: drop first feature vector
        let shifted_features = self.features[1..].to_vec();
        
        // Keep targets but drop the last one to align with shifted features
        let aligned_targets = self.target[..self.target.len() - 1].to_vec();
        
        // Create new index for the aligned data
        let shifted_index = (0..shifted_features.len() as u32).collect();

        Dataset {
            index: shifted_index,
            features: shifted_features,
            target: aligned_targets,
        }
    }

}

/// Truncate decimals on a f64
pub fn truncate_to_decimal(num: f64, decimal_places: u32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places as i32);
    (num * multiplier).trunc() / multiplier
}

/// Load from TOML file
pub fn load_from_toml(file_route: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_route)?;
    let loaded_content = toml::from_str(&contents)?;
    Ok(loaded_content)
}

/// Load from JSON file
pub fn load_from_json(file_route: &str) -> Result<Vec<Orderbook>, Box<dyn Error>> {
    let file = fs::File::open(file_route)?;
    let reader = BufReader::new(file);
    let v_orderbook: Vec<Orderbook> = serde_json::from_reader(reader)?;
    Ok(v_orderbook)
}

/// Write to JSON file
pub fn write_to_json(ob_data: &Vec<Orderbook>, file_route: &str) {
    let ob_json = serde_json::to_string(&ob_data).unwrap();
    let mut file = fs::File::create(&file_route).unwrap();
    file.write_all(ob_json.as_bytes()).unwrap();
}

/// Load from CSV
pub fn load_from_csv(file_route: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    
    let mut rdr = Reader::from_path(file_route)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let float_row: Result<Vec<f64>, _> = record
            .iter()
            .skip(1)
            .map(|field| field.parse::<f64>())
            .collect();

        data.push(float_row?);
    }
    Ok(data)

}

/// Write Dataset to CSV file
pub fn write_to_csv(
    data: &Dataset,
    file_route: &str,
    ) {

    let mut wtr = Writer::from_path(file_route).unwrap();

    // Write the header
    // Header should be based on the number of features, not the index
    if !data.features.is_empty() {
        let mut header = vec!["index".to_string()];
        
        // Add feature column names based on the number of features per sample
        for i in 0..data.features[0].len() {
            header.push(format!("feature_{}", i));
        }
        
        // Add target column
        header.push("target".to_string());
        
        wtr.write_record(&header).unwrap();
    }

    // Write the data rows
    for i in 0..data.features.len() {
        let mut csv_row = Vec::new();
        
        // Add index
        csv_row.push(data.index[i].to_string());
        
        // Add all features for this sample
        for feature_value in &data.features[i] {
            csv_row.push(feature_value.to_string());
        }
        
        // Add target value
        csv_row.push(data.target[i].to_string());
        
        wtr.write_record(&csv_row).unwrap();

    }
    
    wtr.flush().unwrap();
}

