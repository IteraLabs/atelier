# Contributing Guide

Thanks for the interest in contributing to this awesome project !, in order to get you up to speed in terms of sharing your contributed work, or intentions, consider the following: 

## Lints

This project, for the stable version, has implemented the following lints:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
unused_extern_crates = "warn"
```

## rustfmt.toml

Consider all defaults to be present, and, the following changed:

```toml
imports_granularity = "crate"
reorder_impl_items = true
wrap_comments = true
```

## Code format with rustfmt

For the `atelier` crate, there is a `.rustfmt.toml` config file, even though must of the values are exactly the same as the default, they were included for future-proof purposes in terms of formatting. 
 
## Reporting a Bug

In the case that you've found a bug, please make sure you are able to answer the following:

```
- What version of Rust are you using?
- What version of the crate are you using?
- What operating system are you using?
- What did you do?
- What did you expect to see?
- What did you see instead? 
```

