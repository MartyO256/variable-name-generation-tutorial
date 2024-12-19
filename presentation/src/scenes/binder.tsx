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
struct Binder {
    
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
}

enum Constraint {
    String(StringId),
    Identifier(IdentifierId),
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
    undesirables: HashSet<Constraint>,
}
    
enum Constraint {
    String(StringId),
    Identifier(IdentifierId),
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
    undesirables: HashSet<Constraint>,
    used: bool,
}
    
enum Constraint {
    String(StringId),
    Identifier(IdentifierId),
}`,
    1
  );

  yield* beginSlide("binder");

  yield* code().code(
    `\
struct Binder {
    source_parameter: Option<StringId>,
    destination_parameter: IdentifierId,
    restrictions: HashSet<Constraint>,
    undesirables: HashSet<Constraint>,
    used: bool,
}
    
enum Constraint {
    String(StringId),
    Identifier(IdentifierId),
}
    
impl Constraint {
    fn evaluate(&self, identifiers: &IdentifierArena) -> Option<StringId> {
        match self {
            Constraint::Identifier(identifier) => identifiers.lookup(*identifier),
            Constraint::String(string) => Option::Some(*string),
        }
    }
}`,
    1
  );

  yield* beginSlide("binder");
});
