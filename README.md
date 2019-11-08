# UI Lang

> Somewhere to sketch my ideas for what a ui language to suplant react might look like

## Ideas

- duck typed
- immutable by default
- cross platform
- first class testing suport
- curried
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
- collecitons
  - struct ( normal and tuple )
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

## Modules

A given repo is unlikely to contain only one `module`, it will instead likely contain one `project`

```
// src/example-component.ui

module(v0.2.3)({ compileFlags, environment: { variables: { profile } } }){
   let mod1 = use("@dazzlebyte/reuseable/button");
   let mod2 = use("~/reuseable/theme");
   let mod3 = use("todo-item");

   type User = {
      id: String,
      name: String,
   };

   hook getUserById(id) {
      let ( user, setUser ) = useState<Option<Result<User, FetchError>>>(None);

      let response = await useFetch(`/api/users/${id}`, ( id ) );
   };

   component ExampleComponent({ name, id }){

   };

   return {
      getUserById,
      ExampleComponent
   };
}

tests({ describe, it}, {getUserById, ExampleComponent}){

}

stories({createStory}, { ExampleComponent }){

}
```
