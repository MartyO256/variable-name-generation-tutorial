import { Latex, Layout, makeScene2D } from "@motion-canvas/2d";
import {
  all,
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";
import { backgroundFill } from "../constants";

export default makeScene2D(function* (view) {
  view.fill(backgroundFill);

  const grammar0 = createRef<Latex>();
  const grammar1 = createRef<Latex>();
  const grammar2 = createRef<Latex>();
  const grammar3 = createRef<Latex>();
  const grammar4 = createRef<Latex>();
  const grammar5 = createRef<Latex>();
  const grammar6 = createRef<Latex>();

  view.add(
    <Layout direction={"column"} gap={40} padding={100} layout>
      <Latex ref={grammar0} fill="white" fontSize={54} layout={false} />
      <Latex
        ref={grammar1}
        tex={[
          "\\langle{}\\text{expression}\\rangle{}",
          "\\Coloneqq{}",
          "\\lambda{}\\langle{}\\text{identifier}\\rangle{}.",
          "\\langle{}\\text{expression}\\rangle{}",
        ]}
        fill="white"
        fontSize={54}
      />
      <Latex ref={grammar2} fill="white" fontSize={54} layout={false} />
      <Latex
        ref={grammar3}
        tex={[
          "\\hphantom{\\langle{}\\text{expression}\\rangle{}}",
          "\\hphantom{\\Coloneqq{}}",
          "\\mid{}",
          "\\langle{}\\text{expression}\\rangle{}\\ \\langle{}\\text{expression}\\rangle{}+",
        ]}
        fill="white"
        fontSize={54}
      />
      <Latex
        ref={grammar4}
        tex={[
          "\\hphantom{\\langle{}\\text{expression}\\rangle{}}",
          "\\hphantom{\\Coloneqq{}}",
          "\\mid{}",
          "\\langle{}\\text{identifier}\\rangle{}",
        ]}
        fill="white"
        fontSize={54}
      />
      <Latex ref={grammar5} fill="white" fontSize={54} layout={false} />
      <Latex ref={grammar6} fill="white" fontSize={54} layout={false} />
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("ambiguous-parsing-grammar");

  yield* all(
    grammar0()
      .layout(true)
      .tex(
        [
          "\\langle{}\\text{expression}\\rangle{}",
          "\\Coloneqq{}",
          "\\langle{}\\text{expression1}\\rangle{}",
        ],
        1
      ),
    grammar1().tex(
      [
        "\\langle{}\\text{expression1}\\rangle{}",
        "\\Coloneqq{}",
        "\\lambda{}\\langle{}\\text{identifier}\\rangle{}.",
        "\\langle{}\\text{expression1}\\rangle{}",
      ],
      1
    ),
    grammar2()
      .layout(true)
      .tex(
        [
          "\\hphantom{\\langle{}\\text{expression1}\\rangle{}}",
          "\\hphantom{\\Coloneqq{}}",
          "\\mid{}",
          "\\langle{}\\text{expression2}\\rangle{}",
        ],
        1
      ),
    grammar3().tex(
      [
        "\\langle{}\\text{expression2}\\rangle{}",
        "\\Coloneqq{}",
        "\\langle{}\\text{expression3}\\rangle{}+",
      ],
      1
    ),
    grammar4()
      .layout(true)
      .tex(
        [
          "\\langle{}\\text{expression3}\\rangle{}",
          "\\Coloneqq{}",
          "\\langle{}\\text{identifier}\\rangle{}",
        ],
        1
      ),
    grammar5()
      .layout(true)
      .tex(
        [
          "\\hphantom{\\langle{}\\text{expression3}\\rangle{}}",
          "\\hphantom{\\Coloneqq{}}",
          "\\mid{}",
          "(\\langle{}\\text{expression1}\\rangle{})",
        ],
        1
      )
  );

  yield* beginSlide("unambiguous-parsing-grammar");
});
