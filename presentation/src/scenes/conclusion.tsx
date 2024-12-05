import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const step1 = createRef<Txt>();
  const step2 = createRef<Txt>();
  const step3 = createRef<Txt>();
  const step4 = createRef<Txt>();

  view.add(
    <Layout direction={"column"} gap={40} width={1920} padding={100} layout>
      <Txt fontFamily={"Roboto"} fill={"white"} fontSize={100}>
        Conclusion
      </Txt>
      <Txt ref={step1} fontFamily={"Roboto"} fill={"white"} opacity={0.1}>
        1. Use an AST representation that supports extensions
      </Txt>
      <Txt ref={step2} fontFamily={"Roboto"} fill={"white"} opacity={0.1}>
        2. Formulate name generation as a constraint satisfaction problem
      </Txt>
      <Txt ref={step3} fontFamily={"Roboto"} fill={"white"} opacity={0.1}>
        3. Compute constraints mapped to binders
      </Txt>
      <Txt ref={step4} fontFamily={"Roboto"} fill={"white"} opacity={0.1}>
        4. Generate names satisfying pre-computed constraints
      </Txt>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("solution-implementation");

  yield* step1().opacity(1, 1);

  yield* beginSlide("solution-implementation");

  yield* step2().opacity(1, 1);

  yield* beginSlide("solution-implementation");

  yield* step3().opacity(1, 1);

  yield* beginSlide("solution-implementation");

  yield* step4().opacity(1, 1);

  yield* beginSlide("solution-implementation");
});
