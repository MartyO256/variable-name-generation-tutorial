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
fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    
}
`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, expression3)(input)?;
    
}
`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, expression3)(input)?;
    if terms.len() == 1 {
      
    } else {
        
    }
}
`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, expression3)(input)?;
    if terms.len() == 1 {
        Result::Ok((input, terms.remove(0)))
    } else {
        
    }
}
`,
    1
  );

  yield* beginSlide("parsing-implementation");

  yield* code().code(
    `\
fn expression2(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, mut terms) = separated_list1(multispace1, expression3)(input)?;
    if terms.len() == 1 {
        Result::Ok((input, terms.remove(0)))
    } else {
        let function = terms.remove(0);
        Result::Ok((
            input,
            Expression::Application {
                function: Box::new(function),
                arguments: terms,
            },
        ))
    }
}
`,
    1
  );

  yield* beginSlide("parsing-implementation");
});
