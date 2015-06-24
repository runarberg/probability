//! Reexports of probability distributions and accompanying trails.

pub use distribution::Distribution;
pub use distribution::Bernoulli;
pub use distribution::Beta;
pub use distribution::Binomial;
pub use distribution::Categorical;
pub use distribution::Exponential;
pub use distribution::Gamma;
pub use distribution::Gaussian;
pub use distribution::Uniform;

pub use generator::Generator;
pub use generator::default as generator;

pub use sampler::Sampler;
