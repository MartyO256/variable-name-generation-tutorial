# Notes

## Title

Let's tackle the problem of generating fresh variable names to convert expressions to a named representation.
The source code for this project is available at the GitHub repository linked in the description.

## Motivation

In the implementation of a programming language, we may end up in a situation where we have an expression that is synthesized from scratch.
That is, it is not an expression given to us by the end user, but rather an expression generated algorithmically.
Synthesizing expressions is easier to do in a nameless representation precisely because we don't have to deal with variable name captures.

Examples of expression synthesis include:

- Generating code snippets as part of editor actions, like generating patterns for pattern-matching branches, or eta-expanding a highlighted expression.
- Error-reporting after inferring a type with binders like in type systems supporting polymorphism. In this case, the synthesized expression is the inferred type.
- Displaying hints involving reconstructed implicit arguments, like in dependently-typed settings where some function arguments can be automatically constructed based on other arguments in the function call.
- Splicing in the result of automated proof search, where we synthesize a program acting as a proof that checks against a type acting as a logical proposition.

Let's narrow down the problem of fresh variable name generation to a simpler setting and see how we can solve it.

## Problem Statement

Given an expression $M$ that can contain free variables, bound named variables, bound nameless variables represented by de Bruijn indices, named binders and nameless binders, we want to generate a mapping $C$ from binders to names such that applying $C$ to $M$ yields an expression that is equivalent up to renaming of bound variables.

We'll focus on expressions in the untyped lambda calculus, so here binders are lambda abstractions.

```
λ. λ. λ. 3 1 (2 1) => λx. λy. λz. x z (y z)
```

In this first expression, all binders are nameless, and all variables are represented as de Bruijn indices starting at 1.
We need to select parameter names for the three lambda abstractions such that we avoid variable name captures.
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
Here, our variables denoted by $\hat u_i$ correspond to the parameter names for binders in the input expression.
The domain for these parameter names is underscore, denoting that the parameter is not used, and any syntactically valid identifier, in this case alphanumeric strings.

For the untyped lambda calculus expressions we're dealing with, we can identify 4 constraints derived from the way variable names are resolved to binders.

1. If a variable is free, then all parent or enclosing binders cannot use its name.
2. If we reach a variable bound to a binder having parameter name $\hat u_i$, then the binders with a lesser distance cannot use parameter name $\hat u_i$.
3. Conversely, if we reach a variable bound to a binder having parameter name $\hat u_i$, then $\hat u_i$ cannot use the parameter names for binders with lesser distances.
4. Finally, every binder that is used must have a parameter name.

## Solution (continued)

To solve this constraint satisfaction problem, we'll proceed in 4 steps.

1. First, we'll construct a store for parameters. These start off without assigned names. As we proceed with the algorithm, we'll assign names for those parameters (if required).
2. Next, we'll construct a map from binders to those parameters in the store, along with constraints. Those constraints will be a boolean flag denoting whether the parameter is used, along with a set of parameters whose names cannot be reused.
3. With those data structures defined, we'll traverse the input expression and update the constraints for binders.
4. Then, once we have those constraints, we'll re-traverse the input expression, this time to select admissible parameter names.

Do note that the order in which parameter names are decided affects only the visual appeal of the resulting expression.
We can select names in any order we like, provided we satisfy the constraints.

Let's see this in action for the example expressions we saw earlier.

## Worked Example 1

Starting off with the fully nameless expression.
We first need to traverse the expression to construct and map constraints to binders.

For this first outermost lambda abstraction, we'll assign variable $\hat u_1$ to stand for its parameter name.
This variable has to be unique, but it stands for an undetermined name.
It is not the actual parameter name we ultimately want to use.
In curly braces, we'll keep track of restrictions on the value of $\hat u_1$.
These will be inadmissible names for this parameter.
Finally, we do not yet know whether this parameter is actually used in the lambda abstraction's body, so we mark it as unused.

We repeat this constraint construction procedure for the other two binders.

Next, we visit a nameless bound variable in the expression.
Its corresponding binder is the outermost one, shown in cyan.
As per the constraints we identified earlier, parameter names $\hat u_2$ and $\hat u_3$ must be different from $\hat u_1$.
So, we add $\hat u_1$ to the restriction set for both inner binders.
Additionally, as we're traversing from the innermost binder up to the binder for $\hat u_1$, we'll collect the parameter names for binders we encounter along the way.
These parameter names have to be added to the restriction set for $\hat u_1$.
We'll also mark $\hat u_1$ as used since we've found a variable that references it in the expression.

Moving on to the next sub-expression.
We reach another unnamed variable, this time with corresponding binder with parameter name $\hat u_3$.
We simply mark that parameter name as used and move on to the next sub-expression.

We reach an unnamed variable referencing parameter $\hat u_2$.
So, for parameter $\hat u_3$, we cannot assign a name equal to $\hat u_2$.
We keep track of parameter name $\hat u_3$ to add it to the restriction set for $\hat u_2$.
We also mark $\hat u_2$ as used.

We again reach an unnamed variable referencing parameter $\hat u_3$.
This time around, we don't have any updates to make to constraints.

We've finished traversing the input expression.
We now have constraints computed for all the binders it contains, so we move on to choosing admissible names for $\hat u_1$, $\hat u_2$ and $\hat u_3$.
To select parameter names, we'll proceed somewhat naively by guessing names in a sequence until we find an admissible one.
We'll use sequence $x$, $y$, $z$, $x_1$, $y_1$, $z_1$ and so on as needed to solve this.

We now re-traverse the expression.
Starting with the outermost binder with parameter $\hat u_1$.
We know at this point that $\hat u_1$ is used, and that it has no restrictions on its name.
Hence using our sequence of guesses for parameter names, we choose name $\hat u_1$ equal to $x$.
This updates the restriction sets for $\hat u_2$ and $\hat u_3$, with $\hat u_1$ now instantiated as $x$.

Moving on to the next binder.
Here, $\hat u_2$ is used, but cannot be named $x$.
At this point, we do not have a parameter name chosen for $\hat u_3$, so we can ignore it when we select the name for $\hat u_2$.
Following our sequence, we'll choose name $\hat u_2$ equal to $y$.
This updates the restriction sets for $\hat u_1$ and $\hat u_3$.

Moving on to the third binder.
Here, $\hat u_3$ is used, but cannot be $x$ or $y$.
We choose name $\hat u_3$ equal to $z$.

Now that we've selected admissible names for all the binders in the expression, all that's left to do it to resolve nameless variables to their corresponding binders.

We've successfully converted a nameless expression to an alpha-equivalent one by choosing admissible parameter names for its unnamed binders.

## Worked Example 2

Next up is the expression with a binder already having a parameter name.
We construct constraints for both binders, assigning undetermined parameter names $\hat u_1$ and $\hat u_2$ to the lambda abstractions.
When we reach bound variable name $f$, we traverse the stack of binders until we reach the one with parameter name $f$.
The binders we reach in-between cannot use name $\hat u_1$, so we add it to the set of restrictions.

We use identifier $\hat u_1$ instead of $f$ here because we have not yet decided what should be the value for parameter name $\hat u_1$.
We'll see why this is important in example 5 when we have to rename bound variables.

Once we've marked both parameters as used, we re-traverse the expression and select parameter names that satisfy the constraints we built.
For the outermost binder, we can use the existing parameter name $f$ since that name is not in the set of restrictions for it.
For $\hat u_2$, we choose name $x$ following the sequence of names.

When we reach named variable $f$, we look up its corresponding binder in the original expression and use the parameter name assigned to it.
In this case, the parameter name is still $f$.

## Worked Example 3

Let's move to the next example expression, which contains a free variable.
We proceed with the creation of constraints for the binders like in the previous examples.
When we reach free variable $x$, we need to add its name to the set of restrictions for all parent abstractions.

Once we have the constraints, we select names for the parameters as before.
For this example, we could have two sequences of parameter names, one for variables and the other for functions.
We could decide which of the two sequences to use based on type information computed during type-checking.
So here, we select $f$ for the outermost binder abstracting over a function, and $y$ for the innermost binder abstracting over a ground value.

## Worked Example 4

Let's see an example where we have unused parameter names.
When we construct the constraints for binders, the flag for whether the parameter is used is initially set to `false`.
When we reach the bound unnamed variable with de Bruijn index `1`, we update the `used` flag to `true` for the innermost binder.
The outermost binder is still marked as unused after the first traversal of the expression.

In the second traversal, we select `_` as the parameter name for the binder, but we do not assign that `_` to $\hat u_1$ since we want other binders to be able to use that same `_` name.
For the second binder, we have no names in the restriction set, so we select parameter name $x$ and resolve the bound nameless variable to it.

## Worked Example 5

For this last example, we'll see how to handle the renaming of bound variables.
Here we have a reference to the outermost binder that cannot occur in a named representation while re-using the existing parameter names.
The innermost binder shadows the outermost one, so de Bruijn index `2` is problematic.
Thankfully, the way we construct constraints and restriction sets does not change.
Undetermined parameter name $\hat u_1$ cannot be used for $\hat u_2$.
Likewise, $\hat u_2$ cannot be used for $\hat u_1$.

In the second traversal of the expression, we choose parameter name $x$ for the outermost binder since that is the name that was already there in the initial expression.
This updates the restriction set for the second binder.
In that second binder, we would like to use the parameter name that was already present.
However, since that parameter name occurs in the restriction set, we know that we have to rename it, so we choose name $y$ instead.
We update the nameless bound variables accordingly.
If we had named variables referencing that innermost binder, we would have to perform a lookup on the referencing environment where the topmost binding for name $x$ maps it to identifier $\hat u_2$ having value $y$.

## Grammar

Without further ado, let's jump into the implementation for this variable name generation problem.
I invite you to have a look at the implementation available at the GitHub repository linked in the description.
Feel free to pause this video as we walk through the code.

Like in the examples, we'll focus on an untyped lambda calculus in mixed representation.
By mixed representation, I mean that we can have named free or bound variables and named lambda abstractions, and that we can also have nameless bound variables represented as de Bruijn indices and unnamed lambda abstractions.
Our goal is to convert an expression in mixed representation to a fully named representation by soundly generating parameter names for unnamed lambda abstractions.

## AST

In Rust, the typical way to represent such expressions is with a recursive `enum` type, with nested expressions stored as boxes of expressions (`Box<Expression>`).
This could work, but we're missing a key feature with this data representation.
We won't be able to implement a mapping from binders to constraints.
To do that, we need a way to uniquely refer to expressions in a region, and a way to map to and from such references.

We solve this by using a flattened representation for expressions, with an arena allocator implemented as a vector of expressions.
Here, within a given expression arena, expressions are uniquely referred to by their index in the vector.
To construct a mapping from binders to constraints, we'll be able to use a hash map, with expression IDs acting as keys.

Distinct expressions in an expression arena will have distinct expression IDs.
This means that, for instance, each occurrence of a variable `x` in an expression has its own expression ID.

We'll use a similar arena-based representation for strings.
However here, two equal strings will have equal string IDs.
This will speed up operations involving hash sets of strings or hash maps of strings, since it is less expensive to hash an integer than it is to hash a string.

## Expression Arena

We'll need two basic operations for expression arenas:

- a function to get an expression from an ID, and
- a function to add an expression into the arena and obtain its corresponding ID.

For convenience, we can implement the `Index<ExpressionId>` trait so that we can use the vector indexing notation to get expressions out of arenas.

We'll implement simple builder functions to construct expressions owned by the arena.
These functions simply take as input values for the fields of each variant, construct the corresponding expressions, add them to the arena and return their IDs.

## Expression Arena Example

Using string and expression arenas, we can construct expressions like the application of `f` with arguments `x` and `y` as follows.
First, we construct an arena for the strings, and intern the identifiers `f`, `x` and `y`.
Second, we construct the expression "bottom-up" by constructing the named variables first using those interned strings, and then by constructing the application using the variables.
Finally, to retrieve the actual expression stored in the arena, we use the index operation with the ID we got when we constructed the expression.

## Expression Arena Caveat

This arena-based flat representation for expressions has its flaws.
Wherever we have a computation taking an expression as input, we now need to pass both an expression arena and an expression ID.
We then have to perform a lookup in the arena for the expression with that ID just to get back the expression.

That being said, all expressions in an arena are stored contiguously in memory, which can have performance benefits.
It is also trivial to serialize an expression since it just amounts to serializing the vector.
Taking ownership of an expression means taking ownership of the expression arena it lies in.
This can help in implementing safe multithreaded processes.
For variable name generation however, we just need this representation to support constructing maps from expression IDs to constraints.

Alternatives to this design include giving global IDs to expressions as they are created.
This does come with the issue of ensuring that each ID is unique.

We could also use a cursor data structure to construct a tree annotated with the data we need for binders as we traverse the input AST.
However, this would not be as lightweight as simply having a hash map.

## Identifier Arena

Next up, we need a way to create unique variables for binder parameter names.
We'll use a similar arena-based approach where a unique ID for a parameter name within an arena of identifiers is just the index into the vector for that identifier.

This identifier arena structure holds a vector of optional strings.
When we create a new identifier, it starts off without having an assigned string value to it.
That is to say, the identifier still has an undetermined value.
Lookups and assignments in an identifier arena proceed in a straightforward manner.

## Referencing Environment

Later, we'll be performing two top-down traversals of the input expression:

1. the first time for creating constraints, and
2. the second time to choose parameter names satisfying those constraints, renaming bound variables as needed.

We'll need a representation for the state of identifiers in scope and what they are bound to.
For this, we introduce this referencing environment structure.

The `bindings_map` field holds an assocation from parameter names in the input expression to a stack of undetermined identifiers.
This map allows use to perform lookups by name.
The stacks held as values in the map allow us to represent the shadowing of bindings.

We'll also store a stack of expressions for the binders in the language.
This will allow us to resolve nameless variables to their binders.

- When we bind a name, we push the identifier onto the stack of bindings for that name, and push the binder expression onto the stack of binders.
- When we unbind a name, we pop the stack mapped to that name, and we pop the stack of binders.
- When we lookup the latest identifier associated with a given name, we lookup the hash map and retrieve the last element pushed onto the stack of bindings.
- When we want to iterate from the innermost binder to the outermost binder (from right to left in the examples), we need to iterate over the stack of binders in reverse order.

## Binders

Let's define a `Binder` structure holding the hints for selecting admissible parameter names.
We store the original source parameter as an optional string.
Next, we store the destination parameter as an undetermined variable.
Then, we add a hash set of restrictions, with each restriction being either a string or an identifier.

Just like in the examples, we'll traverse the input expression twice in top-down order.
Because of this, we already know the order in which parameter names will be decided.
So, we can split the hash set of restrictions into two, with the second part being the set of quote-unquote undesirable parameter names.
These are the parameter names for binders in sub-expressions: if the parameter name for a parent binder is in that set, then we may have to perform renamings to ensure alpha-equivalence with the input expression.
In most cases, these renamings can be avoided, and this set of undesirables is there precisely to minimize the occurrences of renamings.

## Binder Store

To store these binders, we use a sparse hash map data structure with expression IDs as keys.
We provide a `get` function to retrieve the constraints for a given binder, as well as a `get_mut` function.

## Binder Store Builder

With all these helper data structures defined, we've now reached the point where we need to traverse the input expression and build constraints for the binder parameter names.
We store all the traversal state data in the `ConstraintStoreBuilder` fields, and implement a recursive `visit` function.

- In the case where the expression is a lambda abstraction, we create a new identifier for its parameter name. Next, we construct a new constraint and assign it to the `expression`. Then, we check if the parameter has a user-defined name. If it does, we bind it in the referencing environment, then we recurse on the lambda expression's body, and finally we unbind the parameter. If the lambda expression does not have a user-defined name, we shift the referencing environment then we recurse on the lambda expression's body, and finally we unshift the referencing environment.
- In the case where the expression is a nameless abstraction, we proceed much like in the previous case. We create an identifier for the parameter, create a constraint and map it to the expression, shift the referencing environment, recurse on the lambda expression's body, and unshift the referencing environment.
- In the case where the expression is a named variable, we need to resolve the variable to an identifier. We lookup the referencing environment by name to see if the variable is bound or free.
  - If it is bound, then we already have an identifier for it as defined in some parent lambda abstraction. Like we did in the examples, we'll traverse the stack of binders until we reach the binder for the variable. Along the way, we'll collect the names of parameters for the other binders. Those names are to be avoided for the variable's binder because if we select one of them, we may end up doing unnecessary parameter renamings. We'll also add the variable's corresponding parameter identifier to the set of restrictions for the nested lambda abstractions. Don't forget to mark the parameter as used.
  - If the variable is free, then we create a new identifier for it and immediately assign the variable's name as the name for the identifier. Next, we traverse all the parent binder expressions. For each of them, we mutably get the constraint mapped to it, and add the variable identifier to the set of restrictions.
- In the case where the expression is a nameless variable, we resolve the variable to an identifier. To do this, we lookup the referencing environment by index using the stack of binders. Then, we get the constraint assigned to the binder, and extract its source and destination parameters. Like in the named abstraction case, we iterate over the binders that are parent to the nameless variable up to but not including the variable's binder. For those sub-binders, we add the nameless variable to the set of restrictions. We simultaneously collect the set of parameters for the sub-binders we encounter along the way. Those collected identifiers are added to the set of parameter names to avoid.
- In the case where the expression is an application, we just recursively visit the function sub-expression and its argument sub-expressions.

## Variable Name Generation

Now let's go over the implementation for the sequence of guesses for admissible variable names.
To generate a fresh variable name, we need a mutable reference to the arena of strings since we're generating new strings and need to have them interned.
We also need a predicate to check if a generated name is admissible.
As we'll see later, this predicate will perform lookups in sets of restricted or undesirable identifiers.

For the sequence of variable names `x`, `y`, `z`, `x1`, `y1`, `z1`, and so on, we store a vector of bases containing the names `x`, `y` and `z`.
Then, we go into a loop where we build the next guess for a variable.
For each guess, we pick the next base name in the sequence.
We also update the numeric suffix based on the number of attempts we've done so far.
This loop terminates only if the admissibility predicate returns `true` for some generated name.
Since our predicates are implemented by querying finite sets of names, we know this will terminate because each attempt generates a different string from the ones generated previously.

## Conversion to Named Representation

We now have all the pieces required to select admissible parameter names.
We'll traverse a source expression top-down and construct a destination expression.

For this `NameGeneration` struct, we'll need:

- the arena of strings
- the arena of source expressions
- the arena of destination expressions
- the store of undetermined identifiers for parameter names and restrictions
- the store of constraints on parameter names
- the referencing environment to resolve named and unnamed variables to their binders, and finally
- a variable name generator creating streams of strings.

We start by defining a helper function to convert a set of identifiers into a set of strings.
This will be necessary to evaluate the set of restrictions for a parameter into its corresponding set of inadmissible strings.

Let's call our AST traversal function `convert_to_named`.
We proceed by pattern-matching on the source expression.

- In the case where the expression is an abstraction, we lookup the constraints mapped to it. Next we need to choose a name for the parameter. If the abstraction already had an initial string parameter, we need to check if a renaming is required. So, we lookup the restriction set and check if that initial parameter name is inadmissible. If it is, then we generate a new fresh name for it, one that is neither restricted nor undesirable. Otherwise, we use the existing name. In both of these cases, we also update the identifier store to reflect that we've chosen a name for that parameter. In the case where the initial parameter was not named, we check whether the parameter is actually used. If it is, then we generate a fresh name and assign it to the parameter. If the parameter is not used, then we return `None` so that it is represented as an underscore. Now that we've chosen a parameter name, we need to recursively convert the lambda abstraction's body. Like in the constraint store builder step, we update the state of the referencing environment accordingly. Note that since we're traversing the source expression, we introduce a binding for the original parameter name, such that when we lookup a source named variable, we can resolve it to the new parameter name.
- In the case where the expression is a nameless abstraction, we proceed in a similar fashion as in the previous case. We lookup the constraints for the parameter name, then we check if the parameter is used. If it is, then we generate a fresh name for it. Otherwise, we'll use underscore as parameter name. We then recurse on the abstraction's body and construct the output named abstraction.
- In the case where the expression is a named variable, we check using the referencing environment whether the variable is free or bound. If it is bound, then we lookup the value of the identifier, which was set when we selected parameter names for abstractions. If the variable is free, then by definition it does not have a corresponding binder, so we use its existing name.
- In the case where the expression is a nameless variable, we simply lookup the variable's binder using the stack of binders and the variable's index. Then we lookup the parameter identifier assigned to that binder, evaluate the identifier to a string, and construct the named variable.
- Finally, in the case where the expression is an application, we just recursively convert the function and argument sub-expressions to named representation and re-build an application.

## Conclusion

In conclusion, we've seen how to implement a sound fresh variable name generation algorithm for expressions in a lambda calculus having both named and unnamed variables.
The approach we implemented relied on having a flat arena-based representation for the abstract syntax tree, such that we could easily map from binders to constraints.
We formulated name generation as a constraint satisfaction problem, and implemented a two-phase algorithm that builds constraints and then selects parameter names satisfying those constraints.

## Future Work and Extensions

Now, this implementation was for a simple calculus by design.
We could extend this in many ways to accomodate the requirements of a more realistic programming language.

Specifically, we could extend the AST with a variant for referencing constants in namespaces.
In that case, we would need to consider the first segment in fully qualified identifiers as an identifier appearing in the set of restrictions for parent binders.
This ensures that name generation does not shadow a used namespace.

We could also improve the variable name selection using type information generated during type-checking.
This would allow us to generate different kinds of names based on whether the variable stands for a function or a ground value.

Similarly, we could extend the language with datatype declarations.
We could then provide a mechanism for the user to specify a template for variable names for values of that type.

Finally, we could keep track of the parameter names used for the parent binders when we generate a fresh variable name.
This would allow us to add some form of constraint to discourage variable name shadowing.
Indeed, our current algorithm will always use the first admissible parameter name, so shadowing is bound to occur more often.
Discouraging that would improve the readability of output named expressions.

## Outro

Thanks for watching, and happy coding!
