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

In this fourth expression, we have two lambda abstractions, but only the innermost one is actually used.
To make this problem more interesting, we'll also cover how to determine whether a binder's parameter is actually used, so that we can choose underscore as parameter name to denote that it is not used.

```
λx. λx. 2 1 => λx. λy. x y
```

In this last example, we have two lambda abstractions with the same parameter name.
In the named setting, this means the parameter for the outermost abstraction is unreachable.
However, we have unnamed variables referencing both parameters.
To convert to a named respresentation of the expression, we'll have to rename one of the two parameters.

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

Next up is the expression with a binder already having a parameter name.
We construct constraints for both binders, assigning undetermined parameter names $u_1$ and $u_2$ to the lambda abstractions.
When we reach bound variable name $f$, we traverse the stack of binders until we reach the one with parameter name $f$.
The binders we reach in-between cannot use name $u_1$, so we add it to the set of restrictions.

We use identifier $u_1$ instead of $f$ here is because we have not yet decided what should be the value for parameter name $u_1$.
We'll see why this is important in example 5 when we have to rename bound variables.

Once we've marked both parameters as used, we re-traverse the expression and select parameter names that satisfy the constraints we built.
For the outermost binder, we can use the existing parameter name $f$ since that name is not in the set of restrictions for it.
For $u_2$, we choose name $x$ following the sequence of names.

When we reach named variable $f$, we look up its corresponding binder in the original expression and use the parameter name assigned to it.
In this case, the parameter name is still $f$.

## Worked Example 3

Let's move to the next example expression, which contains a free variable.
We proceed with the creation of constraints for the binders like in the previous examples.
When we reach free variable $x$, we need to add its name to the set of restrictions for all parent abstractions.
In the implementation, we create a new undetermined variable $u_3$ and assign it value $x$.
This simplifies the data type for sets of restrictions.

Once we have the constraints, we select names for the parameters as before.
For this example, we could have two sequences of parameter names, one for variables and the other for functions.
We could decide which of the two sequences to use based on type information computed during type-checking.
So here, we select $f$ for the outermost binder abstracting over a function, and $y$ for the innermost binder abstracting over a ground value.

## Worked Example 4

Let's see an example where we have unused parameter names.
When we construct the constraints for binders, the flag for whether the parameter is used is initially set to `false`.
When we reach the bound unnamed variable with de Bruijn index `1`, we update the `used` flag to `true` for the innermost binder.
The outermost binder is still marked as unused after the first traversal of the expression.

In the second traversal, we select `_` as the parameter name for the binder, but we do not assign that `_` to $u_1$ since we want other binders to be able to use that same `_` name.
For the second binder we have no names in the restriction set, so we select parameter name $x$ and resolve the bound nameless variable to it.

## Worked Example 5

For this last example, we'll see how to handle the renaming of bound variables.
Here we have a reference to the outermost binder that cannot occur in a named representation while re-using the existing parameter names.
Ther innermost binder shadows the outermost one, so de Bruijn index `2` is problematic.
Thankfully, the way we construct constraints and restriction sets does not change.
Undetermined parameter name $u_1$ cannot be used for $u_2$.

In the second traversal of the expression, we choose parameter name $x$ for the outermost binder since that is the name that was already there in the initial expression.
This updates the restriction set for the second binder.
In that second binder, we would like to use the parameter name that was already present.
However, since that parameter name occurs in the restriction set, we know that we have to rename it, so we choose name $y$ instead.
We update the nameless bound variables accordingly.
If we had named variables referencing that innermost binder, we would have to perform a lookup on the referencing environment where the topmost binding for name $x$ maps it to identifier $u_2$ having value $y$.

## Grammar

Without further ado, let's jump into the implementation for this variable name generation problem.

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
