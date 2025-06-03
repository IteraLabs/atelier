/// Modeling Tools for Solana Virtual Machine
///
use spl_math::precise_number::PreciseNumber;
use solana_program::pubkey::Pubkey;

pub trait OnChainModel {
    // Weights should be convertible to a supported type 
    fn export_weights(&self) -> Vec<i32>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SolanaModel {
    pub owner: Pubkey,
    pub num_features: u8,                    // 5-20 features
    pub weights: [i64; 20],                  // Fixed-point weights (max 20)
    pub bias: i64,                           // Fixed-point bias term
    pub weight_scale_factor: u64,            // Global scaling factor for weights
    pub output_scale_factor: u64,            // Scaling factor for output
    pub input_scale_factor: u64,             // Expected input scaling
}

// Conversion utilities
impl SolanaModel {

    // Convert f64 weights to fixed-point during deployment
    pub fn from_trained_weights(
        weights: &[f64], 
        bias: f64, 
        owner: Pubkey,
        precision_decimals: u32
    ) -> Self {
        let scale_factor = 10_u64.pow(precision_decimals); // e.g., 10^9 for 9 decimal places
        
        // Find the maximum absolute weight to determine scaling
        let max_weight = weights.iter()
            .map(|w| w.abs())
            .fold(bias.abs(), f64::max);
        
        // Ensure we don't overflow i64
        let safe_scale = if max_weight * scale_factor as f64 > i64::MAX as f64 {
            ( (i64::MAX as f64) / (max_weight as f64) ) as u64
        } else {
            scale_factor
        };
        
        let mut fixed_weights = [0i64; 20];
        for (i, &weight) in weights.iter().enumerate() {
            fixed_weights[i] = (weight * safe_scale as f64).round() as i64;
        }
        
        Self {
            owner,
            num_features: weights.len() as u8,
            weight_scale_factor: safe_scale,
            output_scale_factor: safe_scale, // Same scale for output
            weights: fixed_weights,
            bias: (bias * safe_scale as f64).round() as i64,
            input_scale_factor: safe_scale,
        }
    }
    
    // Forward pass with fixed-point arithmetic
    pub fn forward(&self, inputs: &[u64]) -> Result<u64, &'static str> {
        
        if inputs.len() != self.num_features as usize {
            return Err("Input dimension mismatch");
        }
        
        // Use u128 for intermediate calculations to prevent overflow
        let mut result: i128 = self.bias as i128;
        
        // Compute weighted sum: Σ(wi * xi)
        for i in 0..self.num_features as usize {
            let weight = self.weights[i] as i128;
            let input = inputs[i] as i128;
            
            // Since both weight and input are scaled,
            // we need to divide by one scale factor
            let product = (weight * input) / self.weight_scale_factor as i128;
            result = result.checked_add(product)
                .ok_or("Arithmetic overflow in forward pass")?;
        }
        
        // Ensure result is positive for u64 return
        if result < 0 {
            return Err("Negative output not supported");
        }
        
        // Convert back to u64, maintaining the output scale
        Ok(result as u64)
    }
    
    // Convert fixed-point output back to logical value
    pub fn rescale_output(&self, fixed_output: u64) -> f64 {
        fixed_output as f64 / self.output_scale_factor as f64
    }
    
    // High-precision forward pass using spl-math for critical applications
    pub fn forward_precise(&self, inputs: &[u64]) -> Result<PreciseNumber, &'static str> {

        if inputs.len() != self.num_features as usize {
            return Err("Input dimension mismatch");
        }
        
        let scale_precise = PreciseNumber::new(self.weight_scale_factor as u128)
            .ok_or("Failed to create scale PreciseNumber")?;
        
        // Start with bias
        let mut result = PreciseNumber::new(self.bias as u128)
            .ok_or("Failed to create bias PreciseNumber")?
            .checked_div(&scale_precise)
            .ok_or("Failed to scale bias")?;
        
        // Add weighted inputs
        for i in 0..self.num_features as usize {

            let weight_precise = PreciseNumber::new(self.weights[i].abs() as u128)
                .ok_or("Failed to create weight PreciseNumber")?
                .checked_div(&scale_precise)
                .ok_or("Failed to scale weight")?;
            
            let input_precise = PreciseNumber::new(inputs[i] as u128)
                .ok_or("Failed to create input PreciseNumber")?
                .checked_div(&PreciseNumber::new(self.input_scale_factor as u128)
                    .ok_or("Failed to create input scale")?)
                .ok_or("Failed to scale input")?;
            
            let product = weight_precise
                .checked_mul(&input_precise)
                .ok_or("Multiplication overflow")?;
            
            if self.weights[i] >= 0 {
                result = result.checked_add(&product)
                    .ok_or("Addition overflow")?;
            } else {
                result = result.checked_sub(&product)
                    .ok_or("Subtraction underflow")?;
            }
        }
        
        Ok(result)
    }
}

// Usage example for deployment
impl SolanaModel {

    pub fn deploy_example() -> Self {
        // Your trained f64 weights
        let trained_weights = vec![0.342, -0.156, 0.789, -0.234, 0.567];
        let trained_bias = 0.123;
        let owner = Pubkey::default();
        
        // Convert to fixed-point with 9 decimal places of precision
        Self::from_trained_weights(&trained_weights, trained_bias, owner, 9)
    }

}
