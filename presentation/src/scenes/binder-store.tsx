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
struct BinderStore {
    binders: HashMap<ExpressionId, Binder>,
}

impl BinderStore {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("binder-store");

  yield* code().code(
    `\
struct BinderStore {
    binders: HashMap<ExpressionId, Binder>,
}

impl BinderStore {
    fn set(&mut self, expression: ExpressionId, binder: Binder) {
        self.binders.insert(expression, binder);
    }

    fn get(&self, expression: ExpressionId) -> Option<&Binder> {
        self.binders.get(&expression)
    }
}`,
    1
  );

  yield* beginSlide("binder-store");

  yield* code().code(
    `\
struct BinderStore {
    binders: HashMap<ExpressionId, Binder>,
}

impl BinderStore {
    fn set(&mut self, expression: ExpressionId, binder: Binder) {
        self.binders.insert(expression, binder);
    }

    fn get_mut(&mut self, expression: ExpressionId) -> Option<&mut Binder> {
        self.binders.get_mut(&expression)
    }
}`,
    1
  );

  yield* beginSlide("binder-store");
});
