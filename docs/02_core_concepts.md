## Building blocks

- Variables are called _bindings_ in Rust
- New bindings are declared using `let`

```rust
let deck: Deck = Deck { cards: vec![] };
```

- Type annotations are optional
- `vec![]` creates an empty vector, can also be done using `Vec::new()`

## Structs

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}
```

- Name of the struct is always capitalized
- Contains list of fields (data) that this struct will wrap up
- `#[derive(Debug)]` defines attributes for the struct (gives compiler extra instructions)
- "Derive attribute" (`derive`) specifies which trais to automatically implement for a given struct
- "Debug" trait (`Debug`) is a set of functions

## Arrays vs vectors

- Arrays have fixed lengths
- Vectors are like arrays that can grow/shrink in size

## Mutable vs immutable bindings

- Bindings are immutable by default
- Use `let mut` (instead of `let`) to create a mutable binding

## Implementations and methods

- _Inherent implementation_ is used to define methods and associated functions for a struct ("add a function to a struct")
- Uses the `impl` keyword identifier followed by the struct type name

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    // In inherent implementations, we can also use -> Self instead of -> Type
    fn new() -> Deck {
        let suits: [&str; 4] = ["Hearts", "Spades", "Diamnonds", "Clubs"];
        let values: [&str; 13] = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
        
        let mut cards: Vec<String> = vec![];

        for suit in suits {
            for value in values {
                let card: String = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        let deck: Deck = Deck { cards };
        return deck;
    }
}
```

- `fn new() -> Deck {}` is a return type annotation, required by compiler
- _Associated functions_ are tied to the struct definition (lacks `&self`)
- _Methods_ operate on a specific instance of a struct, signified by providing `&self` as the first argument

## Implicit returns

```rust
impl Deck {
    fn new() -> Self {
        // ...

        let deck: Deck = Deck { cards };
        return deck;
    }
}
```

```rust
impl Deck {
    fn new() -> Self {
        // ...

        Deck { cards }
    }
}
```

- Implicit return: Rust automatically returns the last evaluated expression _as long as it doesn't have the semicolon_
- Drop the semicolon (!)

```rust
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
```

- Another example:

```rust
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}
```

## Installing external crates

- Rust Standard Library included with every project without any additional install
- [https://docs.rust-lang/std](https://docs.rust-lang/std)
- **External crates** have to be installed into a project with `cargo add`
- Crate listing at [https://crates.io](https://crates.io), docs at [https://docs.rs](https://docs.rs)

## Using crates

- Code in all crates, programs is organized into modules
- Every crate has a _root_ module, and might have some additional submodules
- Within a module, there are functions, structs, etc.
- We can also create submodules in our own project to better organize our code base
- Using something out of an external crate is different from using something out of an internal module:

```rust
// Internal modules have to be loaded
// using 'mod' statements
mod games;

fn main() {
    // Use external crate simply by referencing it
    let rng = rand::thrad_rng();

    let deck = games::Deck::new();
}
```

- Crate functionality can also be "imported" using `use` statements
- This pulls functionality into the scope of this file

```rust
fn main() {
    let rng = rand::threat_rng();
    let rand_number = rand::random();
}
```

```rust
use rand::threat_rng;
use rand::random;

fn main() {
    let rng = threat_rng();
    let rand_number = random();
}
```

```rust
use rand::{threat_rng, random};

fn main() {
    let rng = threat_rng();
    let rand_number = random();
}
```

## Shuffling a slice
## Splitting a vector

## Credits

- [https://www.udemy.com/course/rust-the-complete-developers-guide/](https://www.udemy.com/course/rust-the-complete-developers-guide/)
