import { Code, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";
import { backgroundFill } from "../constants";

Code.defaultHighlighter = new LezerHighlighter(parser);

export default makeScene2D(function* (view) {
  view.fill(backgroundFill);

  const code = createRef<Code>();

  view.add(
    <Code
      ref={code}
      fontSize={34}
      code={`\
fn expression2<'a>(
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
fn expression2<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Application { function, arguments } => {
            let function_doc = expression3(strings, pool, *function)?;
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression2<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Application { function, arguments } => {
            let function_doc = expression3(strings, pool, *function)?;
            let mut argument_docs = Vec::with_capacity(arguments.len());
            for &argument in arguments {
                let argument_doc = expression3(strings, pool, argument)?;
                argument_docs.push(argument_doc);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression2<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Application { function, arguments } => {
            let function_doc = expression3(strings, pool, *function)?;
            let mut argument_docs = Vec::with_capacity(arguments.len());
            for &argument in arguments {
                let argument_doc = expression3(strings, pool, argument)?;
                argument_docs.push(argument_doc);
            }
            Result::Ok(
                function_doc.append(
                    RcDoc::line()
                        .append(RcDoc::intersperse(argument_docs, RcDoc::line()))
                        .nest(2),
                ).group(),
            )
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression2<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Application { function, arguments } => {
            let function_doc = expression3(strings, pool, *function)?;
            let mut argument_docs = Vec::with_capacity(arguments.len());
            for &argument in arguments {
                let argument_doc = expression3(strings, pool, argument)?;
                argument_docs.push(argument_doc);
            }
            Result::Ok(
                function_doc.append(
                    RcDoc::line()
                        .append(RcDoc::intersperse(argument_docs, RcDoc::line()))
                        .nest(2),
                ).group(),
            )
        }
        _ => expression3(strings, pool, expression),
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");
});
