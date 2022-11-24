// advanced features - not going to in depth on this
fn main() {
    /*
    unsafe rust:
    how to opt out of some of Rust's guarantees for the rare times when you need to manually
    unsafe blocks:
    - allow you to call or implement unsafe functions
    - allow you to access or modify a mutable static variable
    - allow you to implement an unsafe trait

    advanced traits:
    associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
    specifying placeholder types in trait definitions with associated types
    - allows for a trait to use some other type without needing to know exactly what that type is until the trait is implemented
    - associated types are a way of declaring that a type implements a trait as long as another type also implements that trait
    - associated types are declared with the 'type' keyword
    - associated types are useful when a trait needs to use a type that hasn't been specified yet
    default generic type parameters and operator overloading
    - allows you to reduce the amount of code you have to write by using default generic type parameters to extract functionality from a concrete type
    - allows you to customize the behavior of operators or specific methods in situations your types or values might need
    - allows you to overload the + operator to add two Point instances together
    fully qualified syntax for disambiguation: calling methods with the same name
    - allows you to specify which implementation of a trait you want to use if a type implements the trait multiple times
    - allows you to call methods with the same name from different traits

    advanced types:
    more about the newtype pattern, type aliases, the never type, and dynamically sized types
    using the newtype pattern to implement external traits on external types
    - allows you to create a new type in a tuple struct without being concerned about runtime performance
    - allows you to implement external traits on external types
    creating type synonyms with type aliases
    - allows you to give a type a name that is different from its original name
    - allows you to reduce repetition
    never that never returns - the '!' type
    - allows you to indicate that a function will never return
    - allows you to express the concept of "this function never returns" in a type system
    dynamically sized types and the sized trait
    - allows you to use types that we can't know the size of at compile time
    - allows you to use types that might be sized or not sized

    advanced functions and closures:
    function pointers and returning closures
    function pointers
    - allows you to store functions in variables
    - allows you to pass functions as arguments to other functions
    - allows you to return functions as the values of other functions
    returning closures
    - allows you to return closures from functions
    - allows you to use the 'impl Trait' syntax to return a closure without writing out its full type

    macros:
    ways to define code that defines more code at compile time
    macros and functions
    - allows you to define code that defines more code at compile time
    - allows you to define macros that look like functions
    - allows you to define macros that take a variable number of arguments of different types
    declarative macros with 'macro_rules!'
    - allows you to define macros for general metaprogramming
    */
}
