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
fn computation(expression: &Expression) {
    ...
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("expression-arena-caveat");

  yield* code().code(
    `\
fn computation(expression: &Expression) {
    ...
}

fn computation(expressions: &ExpressionArena, expression_id: ExpressionId) {
    let expression: &Expression = &expressions[expression_id];
    ...
}`,
    1
  );

  yield* beginSlide("expression-arena-caveat");
});
