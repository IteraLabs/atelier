# atelier-dcml

# Overview

Distributed Convex Machine Learning for the atelier-rs engine.

Includes necessary definitions and tooling to conduct variations of a predictive modeling
process for High Frequency Data, Convex Linear Models, in singular and distributed learning
formulations.

## Dataset

- `atelier-data/data`: Hosts the `Dataset` struct 

## Models

- `atelier-dcml/models.rs`: hosts the `LogisticClassifier`, a convex linear model to perform binary classification.

- Attributes: `id`, `weights` 
- Methods: `compute_gradient`, `forward`

## Loss Functions

- `functions`: hosts the `CrossEntropy`, the loss function to track learning for the binary 
`LogisticClassifier`. In the same script the `Regularized` trait is defined, which will require to have a regularization operation named `RegType`, which has the *L1*, *L1*, and, *Elasticnet* variants, all compatible with loss functions and parameters used in convex linear methods.

- Attributes: 'weights', `y`, `y_hat`, `epsilon`
- Methods: 'regularize'

## Optimizers

- `optimizers.rs`: Includes the `GradientDescent`, the fundamental learning algorithm, which complies with the `Optimizer` trait also defined in this script.  

- Attributes: 'id', 'learning_rate'
- Methods: 'step', 'reset'

Hierarchical Concepts: 

- **Optimizer**: The interface that defines how model parameters are updated
during a training process. Any optimization algorithm implements this trait.

- **Gradient Descent**: An optimization algorithm. Iteratively updates a model's weights values by substracting to them the opposite sign of the calculated gradient.

- **Adam**: Extended version of the Gradient Descent. Incorporates adaptive learning rates and momentum, it mantains exponentially decaying averages of past gradients (first moment) and
past squared gradients (second moment) to adapt learning rates for each parameter 
individually.

## Metrics (WIP)

- **Classification**: Accuracy, F1, Recall, Precission, Confussion Matrix, AuC, Roc.

## Experiments (WIP)

- **Data-Modeling-Metrics** Triad with the complete cluster of modeling elements 
in order to conduct a proper scientific experiment (which by itself will require the
scientific method of Hypothesis-experiment-results-conclusion).

Separation of concerns between Data, Model, Loss, Optimizer. All are independente components thata are orchestrated by a trainer in the following logic: 

Dataset contains both features and targets, sharing the same index. 

Model: Linear model architectures only, contains its weights

## Processes

- Train: Iterative execution of optmizers - models - loss functions
- Infer: Input data parsing, models output generation
- Explain: Model's architecture, Model's weights studies.

# workspace

These are the other published crates members of the workspace: 

- [atelier-core](https://crates.io/crates/atelier-core): Core data structures and I/O tools.
- [atelier-dcml](https://crates.io/crates/atelier-dcml): Distributed Convex Machine Learning. 
- [atelier-generators](https://crates.io/crates/atelier-generators): Probabilistic generators and events simulation.
- [atelier-results](https://crates.io/crates/atelier-results): Standardized results, errors and successes.
- [atelier-synth](https://crates.io/crates/atelier-synth): Synthetic Data Generation for the atelier-rs engine.

Github hosted:

- [benches](https://github.com/IteraLabs/atelier-rs/tree/main/benches)
- [examples](https://github.com/IteraLabs/atelier-rs/tree/main/examples)
- [tests](https://github.com/IteraLabs/atelier-rs/tree/main/tests)

<br>

---
atelier-dcml is a member of the [atelier-rs](https://github.com/iteralabs/atelier-rs
) workspace
