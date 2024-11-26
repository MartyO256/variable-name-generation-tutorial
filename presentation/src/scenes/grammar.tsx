import { Latex, makeScene2D } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";
import { backgroundFill } from "../constants";

export default makeScene2D(function* (view) {
  view.fill(backgroundFill);

  const grammar = createRef<Latex>();
  view.add(
    <Latex
      ref={grammar}
      tex={[
        "\\text{Expressions}\\ ",
        "M, N",
        "\\Coloneqq{}",
        "x",
        "\\mid",
        "{\\lambda x. M}",
        "\\mid",
        "{M\\ N_1\\ N_2 \\cdots N_k}",
      ]}
      fill="white"
      fontSize={54}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("named-grammar");

  yield* grammar().tex(
    [
      "\\text{Expressions}\\ ",
      "M, N",
      "\\Coloneqq{}",
      "x",
      "\\mid",
      "{\\lambda x. M}",
      "\\mid",
      "{M\\ N_1\\ N_2 \\cdots N_k}",
      "\\mid",
      "{\\color{cyan}{\\iota}}",
      "\\mid",
      "{\\color{cyan}{{\\lambda. M}}}",
    ],
    1
  );

  yield* beginSlide("nameless-grammar");
});
