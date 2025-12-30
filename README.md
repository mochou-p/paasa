<!-- paasa/README.md -->

# paasa
parses your string input into a collection of tokens, based on the specified language.  
the output is one-dimensional, there is no nesting in the structure.  

## notes
this parser has strict rules, and aims to follow only what the compiler allows.  
it should **NOT** produce any/reasonable results from invalid input.  

## example
```rs
use paasa::{parse, rust::Token::{self, *}};

fn main() {
    let result      = parse::<Token>("fn hey() {}");
    let expectation = vec![Fn, FnName, ParenStart, ParenEnd, ScopeStart, ScopeEnd];

    assert_eq!(result, Ok(expectation));
}
```
> [!TIP]
> `parse` ignores whitespace, newlines and comments.  
> if you want more control over what tokens you get, use `parse_with_settings`  

## features
> [!IMPORTANT]
> each supported language is behind a cargo feature,  
> so make sure to explicitly enable what you need  

###### ✅ - considered complete, 🚧 - work in progress, ⏳ - planned

- 🚧 rust
- ⏳ c
- ⏳ toml
- ⏳ json

> [!NOTE]
> you can also enable the `full` feature,  
> which enables all other features  

