import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const example1 = createRef<Txt>();
  const example2 = createRef<Txt>();
  const example3 = createRef<Txt>();
  const example4 = createRef<Txt>();

  view.add(
    <Layout direction={"column"} gap={40} width={1920} padding={100} layout>
      <Txt fontFamily={"Roboto"} fill={"white"} fontSize={100}>
        Motivation
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} textWrap={true}>
        Variable name generation occurs whenever synthesized expressions with
        name binders have to be pretty-printed:
      </Txt>
      <Txt
        ref={example1}
        fontFamily={"Roboto"}
        fill={"white"}
        marginLeft={20}
        opacity={0.1}
      >
        • After generating a code snippet that needs to be splicing in source
        code
      </Txt>
      <Txt
        ref={example2}
        fontFamily={"Roboto"}
        fill={"white"}
        marginLeft={20}
        opacity={0.1}
      >
        • After type inference for error-reporting
      </Txt>
      <Txt
        ref={example3}
        fontFamily={"Roboto"}
        fill={"white"}
        marginLeft={20}
        opacity={0.1}
      >
        • After term reconstruction for implicit arguments
      </Txt>
      <Txt
        ref={example4}
        fontFamily={"Roboto"}
        fill={"white"}
        marginLeft={20}
        opacity={0.1}
      >
        • After automated proof search for synthesized programs
      </Txt>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("motivation");

  yield* example1().opacity(1, 1);

  yield* beginSlide("motivation");

  yield* example2().opacity(1, 1);

  yield* beginSlide("motivation");

  yield* example3().opacity(1, 1);

  yield* beginSlide("motivation");

  yield* example4().opacity(1, 1);

  yield* beginSlide("motivation");
});
