import { Code, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

Code.defaultHighlighter = new LezerHighlighter(parser);

export default makeScene2D(function* (view) {
  const code = createRef<Code>();

  view.add(
    <Code
      ref={code}
      fontSize={34}
      code={`\
fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {

    }
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Abstraction { parameter, body } => {
            
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Abstraction { parameter, body } => {
            let parameter_doc = match *parameter {
                Option::Some(n) => name(strings, n),
                Option::None => Result::Ok(RcDoc::text("_")),
            }?;
            let body_doc = expression1(strings, pool, *body)?;
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Abstraction { parameter, body } => {
            let parameter_doc = match *parameter {
                Option::Some(n) => name(strings, n),
                Option::None => Result::Ok(RcDoc::text("_")),
            }?;
            let body_doc = expression1(strings, pool, *body)?;
            Result::Ok(
                RcDoc::text("λ")
                    .append(parameter_doc)
                    .append(".")
                    .append(RcDoc::line().append(body_doc).nest(2))
                    .group(),
            )
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression1<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Abstraction { parameter, body } => {
            let parameter_doc = match *parameter {
                Option::Some(n) => name(strings, n),
                Option::None => Result::Ok(RcDoc::text("_")),
            }?;
            let body_doc = expression1(strings, pool, *body)?;
            Result::Ok(
                RcDoc::text("λ")
                    .append(parameter_doc)
                    .append(".")
                    .append(RcDoc::line().append(body_doc).nest(2))
                    .group(),
            )
        }
        _ => expression2(strings, pool, expression),
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");
});
