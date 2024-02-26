_What's cooler than being cool?_

# 🧊 Ice Code 🧊

_Alright alright alright alright alright alright alright alright_ 

## What is this?

`ice_code` provides a macro to label a particular block of Rust code as being the
cold path in the application. This information allows the compiler to prioritize the
other code in the method, leading to better inlining behavior and boosting performance 
in the common case.

## Usage

### Source
```rust
/// Returns the length of the provided string as long as it's shorter than 256 bytes.
fn short_str_len(input: &str) -> Result<u8, String> {
    if let Ok(short_len) = u8::try_from(input.len()) {
        // This path will be taken almost every time
        Ok(short_len)
    } else { 
        // Oops, the string is too big. This will almost never happen.
        ice! {
            // This can be any expression, including a block or statement.
            Err(format!("Input string was {} bytes, which is too long.", input.len()))
        }
    }
}

pub fn main() {
    let result = short_str_len("foobar".repeat(1_000).as_str());
    assert!(result.is_err());
}
```

### Compiled output
 
![cold_anonymous_fn.png](cold_anonymous_fn.png)

### Labeled cold code

If you use multiple `ice!` invocations in the same method, the mangled anonymous names produced
by the compiler may be difficult for humans to disambiguate. You can add a label to the generated
assembly by using this syntax:

```rust
cold! {
    // label          expression
    error_handler => Err(format!("..."))
}
```

This will produce assembly like the following:

![cold_named_fn.png](cold_named_fn.png)
