# atelier-agents

# Solana Virtual Machine

## Set-up

```shell
solana --version
```

## Scaling & Precision Strategy

Using a global scale factor in the order of `10^p`, where `p` expresses the precision decimals to convert weights trained offchain, which are `f64`, into `i64` to support Solana's integer arithmetic constraints. 

Also, within intermediate calculations, in order to avoid lossing precision, values where
casted tu `u128` only for the operation, then reverted back to `u64`. Multiplication was done 
before division to minimize precision loss. 

To maximize convenience, the `SolanaModel` structure stores separate scale factors for 
weights, inputs, and outputs.

The `forward_precise` uses the `spl_math::PreciseNumber` which provides up to 12 decimal 
places of precision.

## References

- [Solana Arithmetic: Best Practices for Building Financial Apps, Helius.dev](https://www.helius.dev/blog/solana-arithmetic)
