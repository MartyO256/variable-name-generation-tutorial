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
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {

    }
}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            map(name, |n| n.to_vec().into_boxed_slice()),
            multispace0,
        )(input)?;
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            map(name, |n| n.to_vec().into_boxed_slice()),
            multispace0,
        )(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            map(name, |n| n.to_vec().into_boxed_slice()),
            multispace0,
        )(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
        let (input, body) = expression1(input)?;
        IResult::Ok((
            input,
            Expression::Abstraction {
                parameter,
                body: Box::new(body),
            },
        ))
    }
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            map(name, |n| n.to_vec().into_boxed_slice()),
            multispace0,
        )(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
        let (input, body) = expression1(input)?;
        IResult::Ok((
            input,
            Expression::Abstraction {
                parameter,
                body: Box::new(body),
            },
        ))
    }

    alt((lambda_expression, expression2))(input)
}`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression1(input: &[u8]) -> IResult<&[u8], Expression> {
    fn lambda_expression(input: &[u8]) -> IResult<&[u8], Expression> {
        let (input, _) = terminated(lambda, multispace0)(input)?;
        let (input, parameter) = terminated(
            alt((
                map(underscore, |_| Option::None),
                map(name, |n| Option::Some(n.to_vec().into_boxed_slice())),
            )),
            multispace0,
        )(input)?;
        let (input, _) = terminated(dot, multispace0)(input)?;
        let (input, body) = expression1(input)?;
        IResult::Ok((
            input,
            Expression::Abstraction {
                parameter,
                body: Box::new(body),
            },
        ))
    }

    alt((lambda_expression, expression2))(input)
}`,
    1
  );

  yield* beginSlide("parsing-implementation");
});
