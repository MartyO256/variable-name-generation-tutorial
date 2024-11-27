import { Code, Layout, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
  all,
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

Code.defaultHighlighter = new LezerHighlighter(parser);

export default makeScene2D(function* (view) {
  const codeLeft = createRef<Code>();
  const codeRight = createRef<Code>();

  view.add(
    <Layout direction={"row"} width={1920} padding={100} gap={50} layout>
      <Code
        ref={codeLeft}
        fontSize={34}
        width={1920 / 2}
        code={`\
pub enum Expression {
    Variable {
        identifier: Box<[u8]>,
    },
    NamelessVariable {
        index: DeBruijnIndex,
    },
    Abstraction {
        parameter: Option<Box<[u8]>>,
        body: Box<Expression>,
    },
    NamelessAbstraction {
        body: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}`}
      />
      <Code
        ref={codeRight}
        fontSize={34}
        width={1920 / 2}
        code={`\
pub struct DeBruijnIndex {
    index: usize,
}`}
      />
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("ast");

  yield* all(
    codeLeft().code(
      `\
pub enum Expression {
    Variable {
        identifier: StringId,
    },
    NamelessVariable {
        index: DeBruijnIndex,
    },
    Abstraction {
        parameter: Option<StringId>,
        body: ExpressionId,
    },
    NamelessAbstraction {
        body: ExpressionId,
    },
    Application {
        function: ExpressionId,
        arguments: Vec<ExpressionId>,
    },
}`,
      1
    ),
    codeRight().code(
      `\
pub struct DeBruijnIndex {
    index: usize,
}

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}
pub struct ExpressionId {
    index: usize,
}

pub struct StringId {
    index: usize,
}
pub struct StringArena {
    ids: HashMap<Rc<Box<[u8]>>, StringId>,
    strings: Vec<Rc<Box<[u8]>>>,
}`,
      1
    )
  );

  yield* beginSlide("ast");
});
