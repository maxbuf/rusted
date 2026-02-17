## Ownership, borrowing and lifetimes

- Three closely connected systems
- Represent 90% of the difficulty of Rust
- Very different approach from other languages

**Ownership**

1. Every value is 'owned' by a single variable, struct, vector, etc. at a time
2. Reassigning the value to another variable, passing it to a function, putting it into a vector, etc. _moves_ the value (old variable can't be used anymore)

**Borrowing**

3. You can create many read-only references to a value that exist at the same time
4. You can't move a value while a ref to the value exists
5. You can make a writable (mutable) reference to a value _only if_ there are no read-only references currently in use (one mutable ref to a value can exist at a time)
6. you can't mutate a value through the owner when any ref (mutable or immutable) to the value exists
7. Some types of value are _copied_ instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)

**Lifetimes**

8. When a variable goes out of scope, the value owned by it is _dropped_ (cleaned up in memory)
9. Values can't be dropped if there are still active references to it
10. References to a value can't outlive the value they refer to

---

- When in doubt: "Rust wants to minimize unexpected updates to data"
- [https://without.boats/blog/references-are-like-jumps](https://without.boats/blog/references-are-like-jumps)

## Goal of these systems

- 

## Ownership

- 

## Owning and moving values

- 

## Borrowing

- 

## Immutable references

- 

## Mutable references

- 

## Copy-able values

- 

## Copies vs moves

- 

## Credits

- 
