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
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("expression-arena");

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn get(&self, id: ExpressionId) -> &Expression {
        &self.expressions[id.index]
    }

    pub fn add(&mut self, expression: Expression) -> ExpressionId {
        let id = ExpressionId::new(self.expressions.len());
        self.expressions.push(expression);
        id
    }
}`,
    1
  );

  yield* beginSlide("expression-arena");

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn get(&self, id: ExpressionId) -> &Expression {
        &self.expressions[id.index]
    }
}`,
    1
  );

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn get(&self, id: ExpressionId) -> &Expression {
        &self.expressions[id.index]
    }
}

impl Index<ExpressionId> for ExpressionArena {
    type Output = Expression;

    fn index(&self, index: ExpressionId) -> &Self::Output {
        self.get(index)
    }
}`,
    1
  );

  yield* beginSlide("expression-arena");

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {

}`,
    1
  );

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn variable(&mut self, identifier: StringId) -> ExpressionId {
        self.add(Expression::Variable { identifier })
    }
}`,
    1
  );

  yield* beginSlide("expression-arena");

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {

}`,
    1
  );

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn abstraction(
        &mut self,
        parameter: Option<StringId>,
        body: ExpressionId
    ) -> ExpressionId {
        self.add(Expression::Abstraction { parameter, body })
    }
}`,
    1
  );

  yield* beginSlide("expression-arena");

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {

}`,
    1
  );

  yield* code().code(
    `\
pub enum Expression { ... }
pub struct ExpressionId { index: usize }

pub struct ExpressionArena {
    expressions: Vec<Expression>,
}

impl ExpressionArena {
    pub fn application(
        &mut self,
        function: ExpressionId,
        arguments: Vec<ExpressionId>,
    ) -> ExpressionId {
        self.add(Expression::Application { function, arguments })
    }
}`,
    1
  );

  yield* beginSlide("expression-arena");
});
