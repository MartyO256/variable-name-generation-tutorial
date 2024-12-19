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
trait AdmissibleVariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId;
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
trait AdmissibleVariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId;
}

// x, y, z, x1, y1, z1, x2, y2, z2, ...
struct VariableNameGenerator { bases: Vec<Box<[u8]>> }`,
    1
  );

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
// x, y, z, x1, y1, z1, x2, y2, z2, ...
struct VariableNameGenerator { bases: Vec<Box<[u8]>> }`,
    1
  );

  yield* code().code(
    `\
// x, y, z, x1, y1, z1, x2, y2, z2, ...
struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

impl AdmissibleVariableNameGenerator for VariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId {
        
    }
}`,
    1
  );

  yield* beginSlide("variable-generator");

  yield* code().code(
    `\
// x, y, z, x1, y1, z1, x2, y2, z2, ...
struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

impl AdmissibleVariableNameGenerator for VariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId {
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
// x, y, z, x1, y1, z1, x2, y2, z2, ...
struct VariableNameGenerator { bases: Vec<Box<[u8]>> }

impl AdmissibleVariableNameGenerator for VariableNameGenerator {
    fn generate_admissible_name<F: Fn(StringId) -> bool>(
        &mut self,
        strings: &mut StringArena,
        is_admissible: F,
    ) -> StringId {
        let n = self.bases.len();
        let mut attempts = 0;
        let mut suffix = 0;
        loop {
            let mut candidate = self.bases[attempts % n].to_vec();
            if suffix > 0 {
                candidate.extend(suffix.to_string().as_bytes());
            }
            let id = strings.intern(&candidate);
            if is_admissible(id) { {
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
