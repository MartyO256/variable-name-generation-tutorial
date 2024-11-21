extern crate pretty;

use std::string::FromUtf8Error;

use pretty::RcDoc;

use crate::{
    expression::{Expression, ExpressionArena, ExpressionId},
    strings::{StringArena, StringId},
};

impl Expression {
    pub fn to_doc<'a>(
        strings: &StringArena,
        expressions: &ExpressionArena,
        e: ExpressionId,
    ) -> Result<RcDoc<'a>, FromUtf8Error> {
        expression(strings, expressions, e)
    }

    pub fn to_string(
        strings: &StringArena,
        expressions: &ExpressionArena,
        width: usize,
        e: ExpressionId,
    ) -> Result<String, PrettyPrintError> {
        let mut buffer = Vec::default();
        let document = Expression::to_doc(strings, expressions, e)?;
        document.render(width, &mut buffer)?;
        let rendered = String::from_utf8(buffer)?;
        Result::Ok(rendered)
    }
}

pub fn name<'a>(strings: &StringArena, n: StringId) -> Result<RcDoc<'a>, FromUtf8Error> {
    let n = String::from_utf8(strings[n].to_vec())?;
    Result::Ok(RcDoc::as_string(n))
}

fn name_option<'a>(strings: &StringArena, n: Option<StringId>) -> Result<RcDoc<'a>, FromUtf8Error> {
    match n {
        Option::Some(n) => name(strings, n),
        Option::None => Result::Ok(RcDoc::text("_")),
    }
}

fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Abstraction { parameter, body } => {
            let parameter_doc = name_option(strings, *parameter)?;
            let body_doc = expression1(strings, pool, *body)?;
            Result::Ok(
                RcDoc::text("λ")
                    .append(parameter_doc)
                    .append(".")
                    .append(RcDoc::line().append(body_doc).nest(2))
                    .group(),
            )
        }
        Expression::NamelessAbstraction { body } => {
            let body_doc = expression1(strings, pool, *body)?;
            Result::Ok(
                RcDoc::text("λ.")
                    .append(RcDoc::line().append(body_doc).nest(2))
                    .group(),
            )
        }
        _ => expression2(strings, pool, expression),
    }
}

fn expression2<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Application {
            function,
            arguments,
        } => {
            let function_doc = expression3(strings, pool, *function)?;
            let mut argument_docs = Vec::with_capacity(arguments.len());
            for &argument in arguments {
                let argument_doc = expression3(strings, pool, argument)?;
                argument_docs.push(argument_doc);
            }
            Result::Ok(
                function_doc
                    .append(
                        RcDoc::line()
                            .append(RcDoc::intersperse(argument_docs, RcDoc::line()))
                            .nest(2),
                    )
                    .group(),
            )
        }
        _ => expression3(strings, pool, expression),
    }
}

fn expression3<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Variable { identifier } => name(strings, *identifier),
        Expression::NamelessVariable { index } => Result::Ok(RcDoc::as_string(index.into_usize())),
        _ => {
            let expression_doc = expression1(strings, pool, expression)?;
            Result::Ok(
                RcDoc::text("(")
                    .append(
                        RcDoc::line_()
                            .append(expression_doc)
                            .append(RcDoc::line_())
                            .nest(2),
                    )
                    .append(RcDoc::text(")"))
                    .group(),
            )
        }
    }
}

pub fn expression<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    expression1(strings, pool, expression)
}

#[derive(Debug)]
pub enum PrettyPrintError {
    FromUtf8Error(FromUtf8Error),
    IO(std::io::Error),
}

impl std::fmt::Display for PrettyPrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrettyPrintError::FromUtf8Error(error) => error.fmt(f),
            PrettyPrintError::IO(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for PrettyPrintError {}

impl From<FromUtf8Error> for PrettyPrintError {
    fn from(value: FromUtf8Error) -> Self {
        PrettyPrintError::FromUtf8Error(value)
    }
}

impl From<std::io::Error> for PrettyPrintError {
    fn from(value: std::io::Error) -> Self {
        PrettyPrintError::IO(value)
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use rand::{thread_rng, Rng};

    use crate::referencing_environment::ReferencingEnvironment;

    use super::*;

    fn roundabout_test(input: &str) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();

        let parsed_expression =
            Expression::parse_expression(&mut strings, &mut expressions, input.as_bytes()).unwrap();

        let printed = Expression::to_string(&strings, &expressions, 80, parsed_expression).unwrap();

        let reparsed_expression =
            Expression::parse_expression(&mut strings, &mut expressions, printed.as_bytes())
                .unwrap();

        assert!(Expression::equals(
            (&expressions, parsed_expression),
            (&expressions, reparsed_expression)
        ));
    }

    #[test]
    fn roundabout_tests() {
        roundabout_test("x");
        roundabout_test("λx. x");
        roundabout_test("λ_. x");
        roundabout_test("λf. λx. f x");
        roundabout_test("λf. λx. f ((λg. g) x)");
        roundabout_test("λx. λy. λz. x z (y z)");
    }

    fn fuzz_test<R: Rng>(rng: &mut R, max_depth: usize) {
        let mut strings = StringArena::new();
        let mut expressions = ExpressionArena::new();
        let environment = Rc::new(ReferencingEnvironment::new());

        let expression =
            Expression::sample(&mut strings, &mut expressions, environment, rng, max_depth);

        let input = Expression::to_string(&strings, &expressions, 80, expression).unwrap();

        let parsed_expression =
            Expression::parse_mixed_expression(&mut strings, &mut expressions, input.as_bytes())
                .unwrap();

        assert!(Expression::equals(
            (&expressions, expression),
            (&expressions, parsed_expression),
        ));
    }

    #[test]
    fn fuzz_tests() {
        let mut rng = thread_rng();
        let max_depth = 7;
        let test_count = 50;
        for _ in 0..test_count {
            fuzz_test(&mut rng, max_depth);
        }
    }
}
