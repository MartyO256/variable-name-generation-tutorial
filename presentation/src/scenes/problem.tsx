import { Latex, Layout, makeScene2D, Node, Txt } from "@motion-canvas/2d";
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

  const example1 = createRef<Latex>();
  const example2 = createRef<Latex>();
  const example3 = createRef<Latex>();
  const example4 = createRef<Latex>();

  view.add(
    <Layout direction={"column"} gap={40} width={1920} padding={100} layout>
      <Txt fontFamily={"Roboto"} fill={"white"} fontSize={100}>
        Problem Statement
      </Txt>
      <Layout direction={"row"}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          Given an expression
        </Txt>
        <Latex
          tex={["M"]}
          fill="white"
          fontSize={54}
          marginLeft={15}
          marginRight={15}
        />
        <Txt fontFamily={"Roboto"} fill={"white"}>
          with variables (free, bound by name, or bound by index)
        </Txt>
      </Layout>
      <Layout direction={"row"}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          and binders (named or unnamed), generate an association
        </Txt>
        <Latex
          tex={["C"]}
          fill="white"
          fontSize={54}
          marginLeft={15}
          marginRight={15}
        />
        <Txt fontFamily={"Roboto"} fill={"white"}>
          from binders to
        </Txt>
      </Layout>
      <Layout direction={"row"}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          names such that applying
        </Txt>
        <Latex
          tex={["C"]}
          fill="white"
          fontSize={54}
          marginLeft={15}
          marginRight={15}
        />
        <Txt fontFamily={"Roboto"} fill={"white"}>
          to
        </Txt>
        <Latex
          tex={["M"]}
          fill="white"
          fontSize={54}
          marginLeft={15}
          marginRight={15}
        />
        <Txt fontFamily={"Roboto"} fill={"white"}>
          yields an alpha-equivalent expression.
        </Txt>
      </Layout>
      <Layout
        direction={"column"}
        gap={40}
        alignItems={"center"}
        height={400}
        layout
      >
        <Latex
          ref={example1}
          tex={[
            "\\vphantom{f}",
            "\\lambda{}",
            ".",
            "\\ ",
            "\\lambda{}",
            ".",
            "\\ ",
            "\\lambda{}",
            ".",
            "\\ ",
            "3\\ ",
            "1\\ ",
            "(",
            "2\\ ",
            "1",
            ")",
          ]}
          fill="white"
          fontSize={54}
          opacity={0.1}
        />
        <Latex
          ref={example2}
          tex={[
            "\\vphantom{f}",
            "\\lambda{}",
            "f.\\ ",
            "\\lambda{}",
            ".",
            "\\ ",
            "f\\ ",
            "1",
          ]}
          fill="white"
          fontSize={54}
          opacity={0.1}
        />
        <Latex
          ref={example3}
          tex={[
            "\\vphantom{f}",
            "\\lambda{}",
            ".",
            "\\ ",
            "\\lambda{}",
            ".",
            "\\ ",
            "2\\ ",
            "x\\ ",
            "1",
          ]}
          fill="white"
          fontSize={54}
          opacity={0.1}
        />
        <Latex
          ref={example4}
          tex={[
            "\\vphantom{f}",
            "\\lambda{}",
            ".\\ ",
            "\\lambda{}",
            ".",
            "\\ ",
            "1",
          ]}
          fill="white"
          fontSize={54}
          opacity={0.1}
        />
      </Layout>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("problem");

  yield* example1().opacity(1, 1);

  yield* beginSlide("problem");

  yield* example1().tex(
    [
      "\\vphantom{f}",
      "\\lambda{}",
      "x",
      ".",
      "\\ ",
      "\\lambda{}",
      "y",
      ".",
      "\\ ",
      "\\lambda{}",
      "z",
      ".",
      "\\ ",
      "x\\ ",
      "z\\ ",
      "(",
      "y\\ ",
      "z",
      ")",
    ],
    1
  );

  yield* beginSlide("problem");

  yield* example2().opacity(1, 1);

  yield* beginSlide("problem");

  yield* example2().tex(
    [
      "\\vphantom{f}",
      "\\lambda{}",
      "f.\\ ",
      "\\lambda{}",
      "x",
      ".",
      "\\ ",
      "f\\ ",
      "x",
    ],
    1
  );

  yield* beginSlide("problem");

  yield* example3().opacity(1, 1);

  yield* beginSlide("problem");

  yield* example3().tex(
    [
      "\\vphantom{f}",
      "\\lambda{}",
      "f",
      ".",
      "\\ ",
      "\\lambda{}",
      "y",
      ".",
      "\\ ",
      "f\\ ",
      "x\\ ",
      "y",
    ],
    1
  );

  yield* beginSlide("problem");

  yield* example4().opacity(1, 1);

  yield* beginSlide("problem");

  yield* example4().tex(
    [
      "\\vphantom{f}",
      "\\lambda{}",
      "\\textunderscore",
      ".\\ ",
      "\\lambda{}",
      "x",
      ".",
      "\\ ",
      "x",
    ],
    1
  );

  yield* beginSlide("problem");
});
