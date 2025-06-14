use atelier_data::orderbooks::Orderbook;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum TargetsOutput {
    Values,
    // HashMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderbookTargets {
    ReturnSign,
}

impl OrderbookTargets {
    pub fn name(&self) -> &'static str {
        match self {
            OrderbookTargets::ReturnSign => "return_sign",
        }
    }

    pub fn compute(
        &self,
        ob: &Vec<Orderbook>,
    ) -> Result<Vec<f64>, Box<(dyn Error + 'static)>> {
        match self {
            OrderbookTargets::ReturnSign => compute_return_sign(&ob),
        }
    }

    pub fn from_name(name: &str) -> Option<OrderbookTargets> {
        match name {
            "return_sign" => Some(OrderbookTargets::ReturnSign),
            _ => None,
        }
    }

    pub fn all_targets() -> Vec<OrderbookTargets> {
        vec![OrderbookTargets::ReturnSign]
    }

    pub fn list_targets() -> Vec<&'static str> {
        Self::all_targets().iter().map(|f| f.name()).collect()
    }
}

#[derive(Debug)]
pub struct TargetSelector {
    selected_targets: Vec<OrderbookTargets>,
}

impl TargetSelector {
    pub fn new(targets_names: &[&str]) -> Result<Self, String> {
        let mut targets = Vec::new();
        for name in targets_names {
            match OrderbookTargets::from_name(name) {
                Some(target) => targets.push(target),
                None => {
                    return Err(format!(
                        "Unknown target: {}, the ones available are: {:?}",
                        name,
                        OrderbookTargets::list_targets()
                    ));
                }
            }
        }

        Ok(TargetSelector {
            selected_targets: targets,
        })
    }

    /// Select Features by Enum Variant
    pub fn from_targets(targets: Vec<OrderbookTargets>) -> Self {
        TargetSelector {
            selected_targets: targets,
        }
    }

    /// Compute all values
    pub fn compute_values(
        &self,
        ob: &Vec<Orderbook>,
    ) -> Result<Vec<f64>, Box<dyn Error>> {
        let results: Result<Vec<Vec<f64>>, _> = self
            .selected_targets
            .iter()
            .map(|target| target.compute(ob))
            .collect();

        Ok(results?.into_iter().flatten().collect())
    }

    /// Get all features names
    pub fn target_names(&self) -> Vec<&'static str> {
        self.selected_targets.iter().map(|f| f.name()).collect()
    }
}

pub fn compute_targets(
    orderbooks: &Vec<Orderbook>,
    targets_names: &[&str],
    output_format: TargetsOutput,
) -> Result<Vec<f64>, Box<(dyn Error + 'static)>> {
    let selector = TargetSelector::new(targets_names)?;

    match output_format {
        TargetsOutput::Values => selector.compute_values(orderbooks),
    }
}

// --- Different Targets Computations --- //

/// Return's Sign
///
/// The directional price movement (midprice) from t to t+1 is represented
/// as the sign of the corresponding return.
///
pub fn compute_return_sign(
    orderbooks: &Vec<Orderbook>,
) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    // Compute midprices
    let mid_prices: Vec<f64> = orderbooks
        .iter()
        .map(|x| (x.asks[0].price + x.bids[0].price) / 2.0)
        .collect();

    // Compute up indicator: 1.0 if midprice increases, 0.0 otherwise
    let mut up_indicator = Vec::with_capacity(mid_prices.len());
    up_indicator.push(0.0); // First value, no previous to compare

    for i in 1..mid_prices.len() {
        if mid_prices[i] > mid_prices[i - 1] {
            up_indicator.push(1.0);
        } else {
            up_indicator.push(0.0);
        }
    }

    Ok(up_indicator)
}
