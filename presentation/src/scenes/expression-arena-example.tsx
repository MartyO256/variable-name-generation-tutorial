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
      width={1920}
      padding={100}
      code={`\
// Construct expression [f x y]`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("expression-arena-example");

  yield* code().code(
    `\
// Construct expression [f x y]
        
let mut strings: StringArena = StringArena::new();

let f: StringId = strings.intern_str("f");
let x: StringId = strings.intern_str("x");
let y: StringId = strings.intern_str("y");`,
    1
  );

  yield* beginSlide("expression-arena-example");

  yield* code().code(
    `\
// Construct expression [f x y]
        
let mut strings: StringArena = StringArena::new();

let f: StringId = strings.intern_str("f");
let x: StringId = strings.intern_str("x");
let y: StringId = strings.intern_str("y");

let mut expressions: ExpressionArena = ExpressionArena::new();

let vf: ExpressionId = expressions.variable(f);
let vx: ExpressionId = expressions.variable(x);
let vy: ExpressionId = expressions.variable(y);
let expression_id: ExpressionId = expressions.application(vf, vec![vx, vy]);`,
    1
  );

  yield* beginSlide("expression-arena-example");

  yield* code().code(
    `\
// Construct expression [f x y]
        
let mut strings: StringArena = StringArena::new();

let f: StringId = strings.intern_str("f");
let x: StringId = strings.intern_str("x");
let y: StringId = strings.intern_str("y");

let mut expressions: ExpressionArena = ExpressionArena::new();

let vf: ExpressionId = expressions.variable(f);
let vx: ExpressionId = expressions.variable(x);
let vy: ExpressionId = expressions.variable(y);
let expression_id: ExpressionId = expressions.application(vf, vec![vx, vy]);

let expression: &Expression = &expressions[expression_id];
`,
    1
  );

  yield* beginSlide("expression-arena-example");
});
