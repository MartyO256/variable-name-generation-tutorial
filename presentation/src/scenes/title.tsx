import { makeScene2D, Txt } from "@motion-canvas/2d";
import { all, beginSlide, createRef } from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const title = createRef<Txt>();
  view.add(
    <Txt
      ref={title}
      fontFamily={"Roboto"}
      fill={"white"}
      fontSize={100}
      y={50}
      opacity={0}
    />
  );

  title().text("Variable Name Generation");
  yield* all(title().opacity(1, 1), title().y(0, 1));

  yield* beginSlide("title");
});
