import { Latex, Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  view.add(
    <Layout direction={"column"} alignItems={"center"} gap={60} layout>
      <Latex
        tex={[
          "\\vphantom{f}",
          "\\lambda{}",
          ".",
          "\\ \\ ",
          "\\lambda{}",
          ".",
          "\\ \\ ",
          "\\lambda{}",
          ".",
          "\\ \\ ",
          "3\\ \\ ",
          "1\\ \\ ",
          "(",
          "2\\ \\ ",
          "1",
          ")",
        ]}
        fill="white"
        fontSize={150}
      />
      <Latex tex={["\\Downarrow"]} fill="white" fontSize={150} />
      <Latex
        tex={["\\lambda{}x.\\ \\lambda{}y.\\ \\lambda{}z.\\ x\\ z\\ (y\\ z)"]}
        fill="white"
        fontSize={150}
      />
    </Layout>
  );
});
