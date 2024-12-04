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
struct Constraint {
    parameter: IdentifierId,
    restrictions: HashSet<IdentifierId>,
    used: bool,
}

struct ConstraintStore {
    constraints: HashMap<ExpressionId, Constraint>,
}

impl ConstraintStore {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct Constraint {
    parameter: IdentifierId,
    restrictions: HashSet<IdentifierId>,
    used: bool,
}

struct ConstraintStore {
    constraints: HashMap<ExpressionId, Constraint>,
}

impl ConstraintStore {
    fn set(&mut self, expression: ExpressionId, constraint: Constraint) {
        self.constraints.insert(expression, constraint);
    }

    fn get(&self, expression: ExpressionId) -> Option<&Constraint> {
        self.constraints.get(&expression)
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct Constraint {
    parameter: IdentifierId,
    restrictions: HashSet<IdentifierId>,
    used: bool,
}

struct ConstraintStore {
    constraints: HashMap<ExpressionId, Constraint>,
}

impl ConstraintStore {
    fn set(&mut self, expression: ExpressionId, constraint: Constraint) {
        self.constraints.insert(expression, constraint);
    }

    fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Constraint> {
        self.constraints.get_mut(&expression)
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");
});
