use crate::agents::DistributedAgent;
use tch::{Kind, Tensor};


// ----------------------------------------------------------------------------------- //

pub fn singular_training(
    agent: &mut DistributedAgent,
    num_iterations: usize,
    learning_rate: f64,
) {

    for i in 0..num_iterations {
        // 1. Compute current loss for monitoring
        let loss = agent.compute_loss();
        let metric_acc = agent.compute_accuracy(0.50);
        let bce_loss = agent.compute_bce();

        println!(
            "iteration: {:?}, loss: {:?}, acc: {:?}, bce_loss: {:?}",
            i,
            loss.to_kind(Kind::Float),
            metric_acc,
            bce_loss
        );

        // 2. Compute gradients for parameter update
        let gradients = agent.compute_gradient();

        // 3.1 Update parameters using gradients
        agent.theta = &agent.theta - &(gradients * learning_rate);

        // 3.2 Update parameters using Optimizer
        // GradientDescent::builder().learning_rate(0.05);
        // GradientDescent.step(&model.weights, &gradients)
        // agent.theta = &agent.theta - new_grad;

    }
}

// ----------------------------------------------------------------------------------- //

pub fn distributed_training(
    agents: &mut Vec<DistributedAgent>,
    num_iterations: usize,
    consensus_matrix: Tensor,
) {
    // let num_agents = agents.len() as i64;
    // let a_matrix = training::a_matrix(num_agents, "atelier-dcm/Config_01.toml");

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
        let consensus = consensus_matrix.matmul(&theta_stacked);

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
        //

        // --- 4. Update parameters
        for (i, agent) in agents.iter_mut().enumerate() {
            let consensus_theta = consensus.get(i as i64);
            let new_theta = consensus_theta - agent.eta * &gradients[i];
            let accuracy = agent.compute_accuracy(0.5);

            agent.theta = new_theta;
            agent.loss = losses[i].shallow_clone();
            agent.accuracy = accuracy;
        }
    }
}
