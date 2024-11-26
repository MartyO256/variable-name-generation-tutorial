import { Code, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";
import { backgroundFill } from "../constants";

Code.defaultHighlighter = new LezerHighlighter(parser);

export default makeScene2D(function* (view) {
  view.fill(backgroundFill);

  const code = createRef<Code>();

  view.add(
    <Code
      ref={code}
      fontSize={34}
      code={`\
fn expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        
    }
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(name, |n| Expression::Variable {
            identifier: n.to_vec().into_boxed_slice(),
        })(input)
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(name, |n| Expression::Variable {
            identifier: n.to_vec().into_boxed_slice(),
        })(input)
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        delimited(
            left_parenthesis,
            delimited(multispace0, expression1, multispace0),
            right_parenthesis,
        )(input)
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression3(input: &[u8]) -> IResult<&[u8], Expression> {
    fn variable_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        map(name, |n| Expression::Variable {
            identifier: n.to_vec().into_boxed_slice(),
        })(input)
    }

    fn parenthesized_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        delimited(
            left_parenthesis,
            delimited(multispace0, expression1, multispace0),
            right_parenthesis,
        )(input)
    }

    alt((variable_expression, parenthesized_expression))(input)
}`,
    1
  );

  yield* beginSlide("parsing-implementation");
});
