import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import { beginSlide, Direction, slideTransition } from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  view.add(
    <Layout direction={"column"} gap={40} width={1920} padding={100} layout>
      <Txt fontFamily={"Roboto"} fill={"white"} fontSize={100}>
        Motivation
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} textWrap={true}>
        Variable name generation occurs whenever synthesized expressions with
        name binders have to be pretty-printed:
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
        • After generating a code snippet that needs to be splicing in source
        code
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
        • After type inference for error-reporting
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
        • After term reconstruction for implicit arguments
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
        • After automated proof search for synthesized programs
      </Txt>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("motivation");
});
