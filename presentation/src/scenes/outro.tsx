import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import { all, beginSlide, createRef } from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const layout = createRef<Layout>();
  const text = createRef<Txt>();

  view.add(
    <Layout
      ref={layout}
      direction={"column"}
      alignItems={"center"}
      gap={20}
      y={50}
      opacity={0}
      layout
    >
      <Txt ref={text} fontFamily={"Roboto"} fill={"white"} fontSize={120} />
    </Layout>
  );

  yield* all(
    layout().opacity(1, 1),
    layout().y(0, 1),
    text().text("Thanks for watching!", 1)
  );

  yield* beginSlide("outro");

  yield* text().opacity(0, 2);

  yield* beginSlide("outro");
});
