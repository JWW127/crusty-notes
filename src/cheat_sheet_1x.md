## struct
```rust
struct S{}                  // define a struct
    struct S {x : T}       // define struct field x of type T
    struct S (T);           // define 'tupled' struct with field .0 of type T
    struct S;               // define zero size 'unit' struct. takes no space
```

## enum
```rust
enum E {}                   // define an enum
    enum E {A, B(), C{}}    // define enum with variants A-unit, B-tuple, C-struct like
    enum E {A = 1}          // if 'unit' you can assign a discriminant(ID)
    enum E {}               // enums without variants cant be instantiated 'never'
```
## variables
```rust
static X: T = T();          // Global variable with 'static lifetime, singe mem location
const X: T = T();           // Const must include type, must be copied to use
let x: T;                   // Allocate T bytes on stack bound as x. Assignable 1x, not mutable
let mut x: T;               // 'let' but with mutability
x = y;                      // 'Move' value of y to x, invalidating y if not a copy
```

bound varibles live on the stack for synchronous code, for async they become part of the
async's state machine, possibly reside on the heap.

## creating and accessing data structures
```rust
S {x:y}     //create struct S {} use 'ed enum E::S {} with field x set to y
S {x}       // Same, but use local variable x for field x == S{x:x}
S {..s}     // fill remaining fields from s. useful with Default::default()
S {0: x}    // like 'S (x)' below, but set field .0 with struct syntax.
S (x)       // create struct S(T) or use 'ed enum E::S() with field .0 set to x
S           // if is a 'unit' struct S; or 'ed enum E::S create value of S
E::C {x:y}  // alternate method to Create enum variant C
()          // empty tuple both literal and 'unit' type
(x)         // parenthesized expression.
(x,)        // single element tuple expression
(S,)        // single element tuple type
[S]         // array type of unspecified length, slice. CANNOT live on stack
[S; n]      // array type whose length(n) is fixed and of type S
[x; n ]     // array instance witn 'n' copies of 'x'
[x, y]      // array instance with given elements x and y
x[0]        // collection indexing, usize. implementable with Index, IndexMut
    x[..]   // same as above via range 
a..b        // create right-exclusive range e.g. 1..3 == 1,2
..b         // create right-exclusive range without a starting point
a..=b       // create inclusive range e.g. 1..3 == 1,2,3
..=b        // create inclusive range without a starting point
a..         // range from a without an ending point
..          // full range, typically used to get whole collection
s.x         // named field accessing. might deref x if not part of type s
s.0         // numbered field accessing, used in tuple types
```

