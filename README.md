# UI Lang

> Somewhere to sketch my ideas for what a ui language to supplant react might look like

## Ideas

- duck typed
- immutable by default
- cross platform
- first class testing suport
- curried
- built in support for (de)serialising to strings and json
- compiled (eventually, lets start with an interpreter?)
- multiple modules per repo

## Possible Ideal

- lifetimes: they're very cool, but are they too complicated?

## Types

- primitives
  - boolean
  - number
  - string
  - date
  - identifier (string hashed at compile time with module path)
- collecitons
  - struct ( keyed and tuple )
  - array
  - set
  - map
- excutables (each of these are a different type )
  - function
  - component
  - hook
  - regex (special syntax for creating `(s: String) => RegexMatch`)
  - module
- monads
  - Option
  - Result
- other
  - enum (algebraic, like ocaml or rust)
  - semver (first class suport for semver objects)
  - types are a first class runtime concepts
  - compilerSymbol (a way to extend the language using compile time conversions from strings to types)

### Would it be possible to define typesafe runtime conversions between types?
