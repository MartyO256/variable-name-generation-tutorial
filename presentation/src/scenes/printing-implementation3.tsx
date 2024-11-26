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
fn expression3<'a>(
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
fn expression3<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Variable { identifier } => name(strings, *identifier)
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression3<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Variable { identifier } => name(strings, *identifier),
        _ => {
            let expression_doc = expression1(strings, pool, expression)?;
        }
    }
}`,
    1
  );

  yield* beginSlide("printing-implementation");

  yield* code().code(
    `\
fn expression3<'a>(
    strings: &StringArena,
    pool: &ExpressionArena,
    expression: ExpressionId,
) -> Result<RcDoc<'a>, FromUtf8Error> {
    match &pool[expression] {
        Expression::Variable { identifier } => name(strings, *identifier),
        _ => {
            let expression_doc = expression1(strings, pool, expression)?;
            Result::Ok(
                RcDoc::text("(").append(
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
}`,
    1
  );

  yield* beginSlide("printing-implementation");
});
