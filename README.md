# Variable Name Generation Tutorial

This repository contains the implementation and presentation for a tutorial on variable name generation.
The goal for this tutorial is to implement an algorithm for converting expressions from a mixed representation to a named representation.
This is solved by formulating sound parameter name generation as a constraint satisfaction problem.

The algorithm supports performing the following conversions:

```
       Generate names in a natural order
λ. λ. λ. 3 1 (2 1)   =>   λx. λy. λz. x z (y z)

  Keep existing parameter names if possible
        λf. λ. f 1   =>   λf. λx. f x

         Avoid free variable captures
       λ. λ. 2 x 1   =>   λf. λy. f x y

    Mark unused parameters as underscores
           λ. λ. 1   =>   λ_. λx. x

     Rename bound variables if necessary
       λx. λx. 2 1   =>   λx. λy. x y
```

Formally, the grammar for this language is as follows, where `x` ranges over alphanumeric strings and `i` ranges over de Bruijn indices starting at 1:

```
<mixed-expression> ::= <named-expression> | <nameless-expression>

<named-expression> ::= x | λx. <named-expression> | <named-expression> <named-expression>

<nameless-expression> ::= i | λ. <nameless-expression> | <nameless-expression> <nameless-expression>
```

This project is split into:

- [a Rust implementation](./implementation/README.md), and
- [a Motion Canvas presentation](./presentation/README.md) for the video.

The main files are listed below:

```
implementation/
├─ src/
│  ├─ expression.rs   Expressions and expression arenas
│  ├─ to_named.rs     Convert expressions to named representation

presentation/
├─ src/
│  ├─ scenes/         Animated slides
│  ├─ project.ts      Motion Canvas project
```

## Watch the Tutorial Video

[![Watch the tutorial on YouTube](https://img.youtube.com/vi/DtaWjKkLg1A/maxresdefault.jpg)](https://youtu.be/DtaWjKkLg1A)

Click the image or [watch the tutorial on YouTube](https://youtu.be/DtaWjKkLg1A) to follow along step by step.
