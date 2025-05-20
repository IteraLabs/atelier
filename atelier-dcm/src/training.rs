use crate::agents::DistributedAgent;
use tch::{Kind, Tensor};

pub fn consensus_matrix(num_agents: i64) -> Tensor {
    // Row-stochastic matrix for fully connected graph
    let val = 1.0 / num_agents as f64;

    Tensor::from_slice(&vec![val; (num_agents * num_agents) as usize])
        .reshape(&[num_agents, num_agents])
        .to_kind(Kind::Float)
}

// ----------------------------------------------------------------------------------- //

pub fn distributed_training(agents: &mut Vec<DistributedAgent>, num_iterations: usize) {
    let num_agents = agents.len() as i64;
    let a_matrix = consensus_matrix(num_agents);

    for iters in 0..num_iterations {
        println!("\n------ Iteration: {:?} ------ \n", iters);

        //println!("Pre Compute Gradients");
        // --- 1. Compute gradients
        let gradients: Vec<_> = agents.iter().map(|a| a.compute_gradient()).collect();

        //println!("Pre Compute Thetas");
        // --- 2. Collect current parameters
        let thetas: Vec<_> = agents.iter().map(|a| a.theta.shallow_clone()).collect();

        //println!("Pre Compute Consensus");
        // --- 3. Compute consensus (matrix multiplication)
        let theta_stacked = Tensor::stack(&thetas, 0);
        let consensus = a_matrix.matmul(&theta_stacked);

        //println!("Pre Compute Losses");
        // --- 4. Compute metrics --- //
        let losses: Vec<_> = agents.iter().map(|agent| agent.compute_loss()).collect();
        // println!("losses: {:?}", losses);

        //println!("Thetas Matrix: \n{}\n", theta_stacked);
        //println!("Graph Matrix: \n{}\n", a_matrix);
        //println!("Consensus: \n{}\n", consensus);

        //for i_agent in 0..gradients.len() {
            // println!("\nAgent: {:?}\n", i_agent);
            //println!("Gradients: {}", gradients[i_agent]);
            // println!("Loss: {}", losses[i_agent]);
        //}

        // --- 4. Update parameters
        for (i, agent) in agents.iter_mut().enumerate() {
            let consensus_theta = consensus.get(i as i64);
            let new_theta = consensus_theta - agent.eta * &gradients[i];
            agent.theta = new_theta;
            agent.loss = losses[i].shallow_clone();
        }
       
    }
}
