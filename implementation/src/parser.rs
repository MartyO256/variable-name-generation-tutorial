extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, multispace0, multispace1, u32},
    combinator::{eof, map, opt, recognize},
    multi::separated_list1,
    sequence::{delimited, pair, terminated},
    Finish, IResult,
};

use crate::{
    expression::{ExpressionArena, ExpressionId},
    strings::StringArena,
};

pub enum Expression {
    Variable {
        identifier: Box<[u8]>,
    },
    NamelessVariable {
        index: usize,
    },
    Abstraction {
        parameter: Option<Box<[u8]>>,
        body: Box<Expression>,
    },
    NamelessAbstraction {
        body: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

fn lambda(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("λ")(input)
}

fn dot(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(".")(input)
}

fn underscore(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("_")(input)
}

fn left_parenthesis(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("(")(input)
}

fn right_parenthesis(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(")")(input)
}

fn name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    recognize(pair(alpha1, alphanumeric0))(input)
}

/// <expression> ::=
///   | <expression1>
///
/// <expression1> ::=
///   | `λ` <name> `.` <expression1>
///   | `λ` `_` `.` <expression1>
///   | <expression2>
///
/// <expression2> ::=
///   | <expression3>+
///
/// <expression3> ::=
///   | <name>
///   | `(` <expression1> `)`
fn expression(input: &[u8]) -> IResult<&[u8], Expression> {
    expression1(input)
}

fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            alt((
                map(underscore, |_| Option::None),
                map(name, |n| Option::Some(n.to_vec().into_boxed_slice())),
            )),
            multispace0,
        )(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
        let (input, body) = expression1(input)?;
        IResult::Ok((
            input,
            Expression::Abstraction {
                parameter,
                body: Box::new(body),
            },
        ))
    }

    alt((lambda_expression, expression2))(input)
}

fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, expression3)(input)?;
    if terms.len() == 1 {
        Result::Ok((input, terms.remove(0)))
    } else {
        let function = terms.remove(0);
        Result::Ok((
            input,
            Expression::Application {
                function: Box::new(function),
                arguments: terms,
            },
        ))
    }
}

fn expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(name, |n| Expression::Variable {
            identifier: n.to_vec().into_boxed_slice(),
        })(input)
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        delimited(
            left_parenthesis,
            delimited(multispace0, expression1, multispace0),
            right_parenthesis,
        )(input)
    }

    alt((variable_expression, parenthesized_expression))(input)
}

/// <mixed-expression> ::=
///   | <mixed-expression1>
///
/// <mixed-expresion1> ::=
///   | `λ` <name> `.` <mixed-expression1>
///   | `λ` `_` `.` <mixed-expression1>
///   | `λ` `.` <mixed-expression1>
///   | <mixed-expression2>
///
/// <mixed-expression2> ::=
///   | <mixed-expression3>+
///
/// <mixed-expression3> ::=
///   | <name>
///   | <number>
///   | `(` <mixed-expression1> `)`
fn mixed_expression(input: &[u8]) -> IResult<&[u8], Expression> {
    mixed_expression1(input)
}

fn mixed_expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = opt(terminated(
            alt((
                map(underscore, |_| Option::None),
                map(name, |n| Option::Some(n.to_vec().into_boxed_slice())),
            )),
            multispace0,
        ))(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
        let (input, body) = mixed_expression1(input)?;
        match parameter {
            Option::Some(parameter) => IResult::Ok((
                input,
                Expression::Abstraction {
                    parameter,
                    body: Box::new(body),
                },
            )),
            Option::None => IResult::Ok((
                input,
                Expression::NamelessAbstraction {
                    body: Box::new(body),
                },
            )),
        }
    }

    alt((lambda_expression, mixed_expression2))(input)
}

fn mixed_expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, mixed_expression3)(input)?;
    if terms.len() == 1 {
        Result::Ok((input, terms.remove(0)))
    } else {
        let function = terms.remove(0);
        Result::Ok((
            input,
            Expression::Application {
                function: Box::new(function),
                arguments: terms,
            },
        ))
    }
}

fn mixed_expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn nameless_variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(u32, |n| Expression::NamelessVariable { index: n as usize })(input)
    }

    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(name, |n| Expression::Variable {
            identifier: n.to_vec().into_boxed_slice(),
        })(input)
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        delimited(
            left_parenthesis,
            delimited(multispace0, mixed_expression1, multispace0),
            right_parenthesis,
        )(input)
    }

    alt((
        nameless_variable_expression,
        variable_expression,
        parenthesized_expression,
    ))(input)
}

fn lower(
    strings: &mut StringArena,
    expressions: &mut ExpressionArena,
    expression: &Expression,
) -> ExpressionId {
    match expression {
        Expression::Variable { identifier } => {
            let lowered_identifier = strings.intern(identifier);
            expressions.variable(lowered_identifier)
        }
        Expression::NamelessVariable { index } => expressions.nameless_variable((*index).into()),
        Expression::Abstraction { parameter, body } => {
            let lowered_parameter = parameter.as_ref().map(|n| strings.intern(n));
            let lowered_body = lower(strings, expressions, body);
            expressions.abstraction(lowered_parameter, lowered_body)
        }
        Expression::NamelessAbstraction { body } => {
            let lowered_body = lower(strings, expressions, body);
            expressions.nameless_abstraction(lowered_body)
        }
        Expression::Application {
            function,
            arguments,
        } => {
            let lowered_function = lower(strings, expressions, function);
            let mut lowered_arguments = Vec::with_capacity(arguments.len());
            for argument in arguments {
                let lowered_argument = lower(strings, expressions, argument);
                lowered_arguments.push(lowered_argument);
            }
            expressions.application(lowered_function, lowered_arguments)
        }
    }
}

pub fn parse_expression<'a>(
    strings: &mut StringArena,
    expressions: &mut ExpressionArena,
    input: &'a [u8],
) -> Result<ExpressionId, nom::error::Error<&'a [u8]>> {
    match terminated(delimited(multispace0, expression, multispace0), eof)(input).finish() {
        Result::Ok((_input, parsed)) => {
            let lowered = lower(strings, expressions, &parsed);
            Result::Ok(lowered)
        }
        Result::Err(error) => Result::Err(error),
    }
}

pub fn parse_mixed_expression<'a>(
    strings: &mut StringArena,
    expressions: &mut ExpressionArena,
    input: &'a [u8],
) -> Result<ExpressionId, nom::error::Error<&'a [u8]>> {
    match terminated(delimited(multispace0, mixed_expression, multispace0), eof)(input).finish() {
        Result::Ok((_input, parsed)) => {
            let lowered = lower(strings, expressions, &parsed);
            Result::Ok(lowered)
        }
        Result::Err(error) => Result::Err(error),
    }
}
