# Atelier - Distributed Convex Machine Learning

Includes necessary definitions and tooling to conduct variations of a predictive modeling
process for High Frequency Data, Convex Linear Models, in singular and distributed learning
formulations.

## Models

- `models.rs`: hosts the `LogisticClassifier`, a convex linear model to perform binary classification. 

## Loss Functions

- `functions`: hosts the `CrossEntropy`, the loss function to track learning for the binary 
`LogisticClassifier`. In the same script the `Regularized` trait is defined, which will require to have a regularization operation named `RegType`, which has the *L1*, *L1*, and, *Elasticnet* variants, all compatible with loss functions and parameters used in convex linear methods. 

## Optimizers

- `optimizers.rs`: Includes the `GradientDescent`, the fundamental learning algorithm, which complies with the `Optimizer` trait also defined in this script.  

Hierarchical Concepts: 

- **Optimizer**: The interface that defines how model parameters are updated
during a training process. Any optimization algorithm implements this trait.

- **Gradient Descent**: An optimization algorithm. Iteratively updates a model's weights values by substracting to them the opposite sign of the calculated gradient.

- **Adam**: Extended version of the Gradient Descent. Incorporates adaptive learning rates and momentum, it mantains exponentially decaying averages of past gradients (first moment) and
past squared gradients (second moment) to adapt learning rates for each parameter 
individually.

## Metrics (WIP)

- **Classification**: Those metrics  related to classification problems ...

## Experiments (WIP)

- **Data-Modeling-Metrics** Triad with the complete cluster of modeling elements 
in order to conduct a proper scientific experiment (which by itself will require the
scientific method of Hypothesis-experiment-results-conclusion).

## Processes (WIP)

- Train: Iterative execution of optmizers - models - loss functions
- Infer: Input data parsing, models output generation
- Explain: Model's architecture, Model's weights studies.

