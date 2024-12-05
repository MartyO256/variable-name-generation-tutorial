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
      fontSize={30}
      width={1920}
      padding={100}
      code={`\
trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

// x, y, z, x1, y1, z1, x2, y2, z2, ...
impl FreshVariableNameGenerator for VariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        
    }
}`,
    1
  );

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

// x, y, z, x1, y1, z1, x2, y2, z2, ...
impl FreshVariableNameGenerator for VariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        let n = self.bases.len();
        let mut attempts = 0;
        let mut suffix = 0;
        loop {

        }
    }
}`,
    1
  );

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
trait FreshVariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId;
}

struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

// x, y, z, x1, y1, z1, x2, y2, z2, ...
impl FreshVariableNameGenerator for VariableNameGenerator {
    fn fresh_name(&mut self, strings: &mut StringArena, claimed: &HashSet<StringId>) -> StringId {
        let n = self.bases.len();
        let mut attempts = 0;
        let mut suffix = 0;
        loop {
            let mut candidate = self.bases[attempts % n].to_vec();
            if suffix > 0 {
                candidate.extend(suffix.to_string().as_bytes());
            }
            let id = strings.intern(&candidate);
            if !claimed.contains(&id) {
                return id;
            }
            attempts += 1;
            if attempts % n == 0 {
                suffix += 1;
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("variable-generator");
});
