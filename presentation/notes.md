# Notes

## Title

Let's tackle the problem of generating fresh variable names to convert expressions to a named representation.
The source code for this project is available at the GitHub repository linked in the description.

## Motivation

In the implementation of a programming language, we may end up in a situation where we have an expression that is synthesized from scratch.
That is, it is not an expression given to us by the end user.
Synthesizing expressions is easier to do in a nameless representation precisely because we don't have to deal with variable captures.

Examples of expression synthesis include:

- Generating code snippets as part of editor actions, like generating pattern-matching branches, or eta-expandind a highlighted expression.
- Error-reporting after inferring a type with binders like in type systems supporting polymorphism. In this case, the synthesized expression is the inferred type.
- Displaying hints involving reconstructed implicit arguments, like in dependently-typed settings where some function arguments can be automatically constructed based on other arguments in the function call.
- Splicing in the result of automated proof search, where we synthesize a program acting as a proof that checks against a type acting as a logical proposition.

Let's narrow down the problem of fresh variable generation to a simpler setting and see how we can solve it.

## Problem Statement

Given an expression $M$ that can contain free variables, variables bound by name, nameless variables bound by de Bruijn indices, named binders and nameless binders, we want to generate a mapping $C$ from binders to names such that applying $C$ to $M$ yields an expression that is equivalent up to renaming of bound variables.

We'll focus on expressions in the untyped lambda calculus, so here binders are lambda abstractions.

```
λ. λ. λ. 3 1 (2 1) => λx. λy. λz. x z (y z)
```

In this first expression, all binders are nameless, and all variables are represented as de Bruijn indices starting at 1.
We need to select parameter names for the three lambda abstractions such that we avoid variable captures.
This is solved easily by selecting distinct names for all parameters.

```
λf. λ. f 1 => λf. λx. f x
```

In this second expression, we have a mix of named and unnamed lambda abstractions.
Because of the parameter name $f$ in the outermost abstraction, the variable name $f$ is in scope for the body of the innermost abstraction.
When we select a name for the second abstraction, we have to avoid reusing the name $f$.

```
λ. λ. 2 x 1 => λf. λz. f x y
```

In this third expression, the variable $x$ is free.
This means that both lambda abstractions cannot use $x$ as parameter name, otherwise we would capture that free variable.

```
λ. λ. 1 => λ_. λx. x
```

In this last expression, we have two lambda abstractions, but only the innermost one is actually used.
To make this problem more interesting, we'll also cover how to determine whether a binder's parameter is actually used, so that we can choose underscore as parameter name to denote that it is not used.

Let's see how we can solve this problem.

## Solution

The first step in the solution is to realise that variable name generation is a constraint satisfaction problem.
Here, our variables denoted $u_i$ correspond to the parameter names for binders in the input expression.
The domain for these parameter names is underscore, denoting that the parameter is not used, and any syntactically valid identifier.

For the untyped lambda calculus expressions we're dealing with, we can identify 4 constraints derived from the way variable names are resolved to binders.

1. If a variable is free, then all parent or enclosing binders cannot use its name.
2. If we reach a bound variable, and that bound variable is bound to a binder having parameter name $u_i$, then the binders with a lesser distance cannot use parameter name $u_i$.
3. Conversely, if a variable is bound to a binder having parameter name $u_i$, then $u_i$ cannot use the parameter names for binders with lesser distances.
4. Finally, every binder that is used must have a parameter name.

## Solution (continued)

To solve this constraint satisfaction problem, we'll proceed in 4 steps.

1. First, we'll construct a store for parameters. These start off without assigned names. As we proceed with the algorithm, we'll assign names for those parameters (if required).
2. Next, we'll construct a map from binders to those parameters in the store, along with constraints. Those constraints will be a boolean flag denoting whether the parameter is used, and a set of parameters whose names cannot be reused.
3. With those data structures defined, we'll traverse the input expression and update the constraints for binders.
4. Then, once we have those constraints, we'll re-traverse the input expression, this time to select admissible parameter names.

Let's see this in action for the example expressions we saw earlier.

## Worked Example 1

Starting off with the fully nameless expression.
We start off by a traversal of the expression to construct and map constraints to binders.

For this first outermost lambda abstraction, we'll assign variable $u_1$ to stand for its parameter name.
This variable has to be unique, but it stands for an undetermined named.
It is not the actual parameter name we ultimately want to use.
In curly braces, we'll keep track of restrictions on the value of $u_1$.
These will be inadmissible names for this parameter.
Finally, we do not yet know whether this parameter is actually used in the lambda expression's body, so we mark it as unused.

We repeat this constraint construction procedure for the other two binders.

Next, we visit a nameless bound variable in the expression.
Its corresponding binder is the outermost one, shown in cyan.
As per the constraints we identified earlier, parameter names $u_2$ and $u_3$ must be different from $u_1$.
So, we add $u_1$ to the restriction set for both inner binders.
We'll also mark $u_1$ as used since we've found a variable that references it in the expression.

Moving on to the next sub-expression.
We reach another unnamed variable, this time with corresponding binder with parameter name $u_3$.
We simply mark that parameter name as used and move on to the next sub-expression.

We reach an unnamed variable referencing parameter $u_2$.
So for parameter $u_3$, we cannot assign a name equal to $u_2$.
We also mark $u_2$ as used.

We again reach an unnamed variable referencing parameter $u_3$.
This time around, we don't have any updates to make to constraints.

We've finished traversing the input expression.
We now have a constraints computed for all the binders it contains, so we move on to choosing admissible names for $u_1$, $u_2$ and $u_3$.
To select parameter names, we'll proceed somewhat naively by guessing names in a sequence until we find an admissible one.
We'll use sequence $x$, $y$, $z$, $x_1$, $y_1$, $z_1$ and so on as needed to solve this.

We now re-traverse the expression.
Starting with the outermost binder with parameter $u_1$.
We know at this point that $u_1$ is used, and that it has no restrictions on its name.
Hence using our sequence of guesses for parameter names, we choose name $u_1$ equal to $x$.
This updates the restriction sets for $u_2$ and $u_3$, with $u_1$ now instantiated as $x$.

Moving on to the next binder.
Here, $u_2$ is used, but cannot be named $x$.
Following our sequence, we'll choose name $u_2$ equal to $y$.
This again updates the restriction set for $u_3$.

Moving on to the third binder.
Here, $u_3$ is used, but cannot be $x$ or $y$.
We choose name $u_3$ equal to $z$.

Now that we've selected admissible names for all the binders in the expression, all that's left to do it to resolve nameless variables to their corresponding binders.

We've successfully converted from that fully nameless expression to an alpha-equivalent one by choosing admissible parameter names for its unnamed binders.

## Worked Example 2

## Worked Example 3

## Worked Example 4

## Grammar

## AST

## Expression Arena

## Expression Arena Example

## Expression Arena Caveat

## Identifier Arena

## Referencing Environment

## Constraints

## Constraint Store Builder

## Variable Name Generation

## Conversion to Named Representation

## Conclusion

## Future Work and Extensions
