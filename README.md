Inspired by `https://github.com/kohsuke/wordnet-random-name`

Usage:

```rust
extern crate dictionary;

use dictionary::Dictionary;

let mut dict = Dictionary::new();

let x = dict.word();
```