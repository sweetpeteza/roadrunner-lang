# Recursion Fix - RESOLVED

## Problem Summary

The evaluator had a **stack overflow** when executing closures and recursive functions. This occurred in the `test_closures::case_1` test.

## Root Cause

**The issue was NOT with the environment implementation or recursive function handling.**

The actual root cause was a `debug!` macro in `extend_function_env()` that attempted to print the environment:

```rust
debug!("Extended function environment: {:?}", extended_env);
```

When closures capture their environment, they create circular references:
1. A `Function` object contains an `env: Env` (which is `Rc<RefCell<Environment>>`)
2. The `Environment` contains `outer: Option<Env>` pointing to parent environments
3. When a closure is stored in an environment and that environment is captured by another closure, a cycle is formed

The derived `Debug` implementation for `Environment` would try to print the entire chain, including any `Object::Function` values in the store, which would then try to print their captured environments... causing infinite recursion.

## Fix Applied

Simply removed the problematic debug line in `src/evaluator.rs`:

```diff
- debug!("Extended function environment: {:?}", extended_env);
```

## Tests Added

Added deep recursion tests to verify the fix:

```rust
#[case("let countdown = fn(n) { if (n == 0) { return 0; } else { countdown(n - 1); } }; countdown(50);", Object::Integer(0))]
#[case("let countdown = fn(n) { if (n == 0) { return 0; } else { countdown(n - 1); } }; countdown(100);", Object::Integer(0))]
fn test_deep_recursion(...)
```

## Results

- All 142 tests pass
- Closures work correctly (nested functions with captured environments)
- Recursion works correctly (recursive counter function)
- Deep recursion works (100 levels of recursive calls)

## Lessons Learned

1. **Be careful with `Debug` on cyclic data structures** - `#[derive(Debug)]` can cause infinite loops when there are circular references
2. **`Rc<RefCell<T>>` enables cycles** - When using shared mutable state, be aware that cycles can form
3. **Debug logging can have side effects** - Even "harmless" debug statements can cause issues in edge cases

## Future Considerations

If you need to debug environments in the future, implement a custom `Debug` that:
- Tracks visited nodes to detect cycles
- Limits recursion depth
- Only prints essential information (e.g., just the keys, not the full values)

Example safe debug approach:
```rust
impl Environment {
    pub fn debug_keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }
}
```
