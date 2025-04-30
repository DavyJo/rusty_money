# Potential Improvements for rusty-money

This document outlines potential areas for improvement in the `rusty-money` crate, based on an analysis of the codebase as of version 0.4.22.

## Documentation

### Solution Approach

The following strategy should be implemented to address the current documentation gaps:

#### 1. Error Conditions Documentation

Each public function/method that can return a `MoneyError` should be annotated with a clear "# Errors" section in its documentation. This should:

- List **every** possible error variant that might be returned
- Explain the specific conditions that trigger each error type
- Add cross-references to related error-handling examples

Implementation approach:
```rust
/// Function description...
///
/// # Errors
///
/// Returns `Err(MoneyError::CurrencyMismatch)` if the currencies of the two Money objects differ.
/// Returns `Err(MoneyError::InvalidCurrency)` if the provided currency code is not recognized.
```

#### 2. Panic Documentation

All functions that can panic should be explicitly documented with a "# Panics" section that:

- Clearly describes the exact conditions that will cause a panic
- Suggests non-panicking alternatives if available
- Explains why the panic exists (e.g., trait implementation constraints)

Implementation approach:
```rust
/// Function description...
///
/// # Panics
///
/// Panics if the currencies differ when using operator overloading traits that cannot
/// return Result types (e.g., AddAssign, SubAssign, Ord). Consider using the 
/// Result-returning methods instead for fallible operations.
```

#### 3. Rounding Behavior Documentation

Clearly document the rounding strategies:

- Update the `Money::round` method docs to specifically explain each available `RoundingStrategy` option
- In `Formatter`, clearly document that `MidpointNearestEven` (banker's rounding) is used by default
- Add cross-references between rounding-related components
- Include visual examples of different rounding behaviors with the same input value

#### 4. Usage Examples

Add comprehensive examples that:

- Demonstrate proper error handling patterns for fallible operations
- Show advanced formatting capabilities with different locales and parameters
- Cover common use cases like currency conversion, allocation, and comparison
- Illustrate how to avoid common pitfalls (e.g., currency mismatch errors, unexpected rounding)

Each example should be concise but complete, showing both the success and error paths where applicable.

### Priority 

These documentation improvements should be implemented in this order:
1. Error conditions (highest impact for API safety)
2. Panic conditions (critical for reliability)
3. Rounding behavior (important for correctness)
4. Examples (enhances usability)
