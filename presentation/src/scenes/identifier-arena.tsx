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
struct IdentifierId { index: usize }

struct IdentifierArena { identifiers: Vec<Option<StringId>> }

impl IdentifierArena {
    pub fn new_identifier(&mut self) -> IdentifierId {
        let index = self.identifiers.len();
        self.identifiers.push(Option::None);
        IdentifierId::new(index)
    }
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("identifier-arena");

  yield* code().code(
    `\
struct IdentifierId { index: usize }

struct IdentifierArena { identifiers: Vec<Option<StringId>> }

impl IdentifierArena {
    pub fn new_identifier(&mut self) -> IdentifierId {
        let index = self.identifiers.len();
        self.identifiers.push(Option::None);
        IdentifierId::new(index)
    }

    pub fn lookup(&self, id: IdentifierId) -> Option<StringId> {
        self.identifiers[id.into_usize()]
    }
}`,
    1
  );

  yield* beginSlide("identifier-arena");

  yield* code().code(
    `\
struct IdentifierId { index: usize }

struct IdentifierArena { identifiers: Vec<Option<StringId>> }

impl IdentifierArena {
    pub fn new_identifier(&mut self) -> IdentifierId {
        let index = self.identifiers.len();
        self.identifiers.push(Option::None);
        IdentifierId::new(index)
    }

    pub fn lookup(&self, id: IdentifierId) -> Option<StringId> {
        self.identifiers[id.into_usize()]
    }

    pub fn set(&mut self, id: IdentifierId, name: StringId) {
        self.identifiers[id.into_usize()] = Option::Some(name);
    }
}`,
    1
  );

  yield* beginSlide("identifier-arena");
});
