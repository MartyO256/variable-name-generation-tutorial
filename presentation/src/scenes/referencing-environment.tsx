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
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {
    fn bind(
        &mut self,
        name: StringId,
        identifier: IdentifierId,
        binder: ExpressionId
    ) {
        match self.bindings_map.entry(name) {
            Entry::Occupied(mut stack) => {
                stack.get_mut().push(identifier);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![identifier]);
            }
        };
        self.binders_stack.push(binder);
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {

}`,
    1
  );

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {
    fn unbind(&mut self, identifier: StringId) {
        let stack = self.bindings_map.get_mut(&identifier).unwrap();
        stack.pop();
        if stack.is_empty() {
            self.bindings_map.remove(&identifier);
        }
        self.binders_stack.pop();
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {

}`,
    1
  );

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {
    fn lookup(&self, identifier: StringId) -> Option<IdentifierId> {
        self.bindings_map
            .get(&identifier)
            .and_then(|stack| stack.last().copied())
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {

}`,
    1
  );

  yield* code().code(
    `\
struct ReferencingEnvironment {
    bindings_map: HashMap<StringId, Vec<IdentifierId>>,
    binders_stack: Vec<ExpressionId>,
}

impl ReferencingEnvironment {
    fn binders_iter(&self) -> Rev<Iter<'_, ExpressionId>> {
        self.binders_stack.iter().rev()
    }
}`,
    1
  );

  yield* beginSlide("constraint-store");
});
