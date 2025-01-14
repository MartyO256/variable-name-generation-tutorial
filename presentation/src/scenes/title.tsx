import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import { all, beginSlide, createRef } from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const layout = createRef<Layout>();
  const title = createRef<Txt>();
  const name = createRef<Txt>();
  const url = createRef<Txt>();

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
      <Txt ref={title} fontFamily={"Roboto"} fill={"white"} fontSize={120}>
        Variable Name Generation
      </Txt>
      <Txt ref={name} fontFamily={"Roboto"} fill={"white"} fontSize={60}>
        Marc-Antoine Ouimet
      </Txt>
      <Txt ref={url} fontFamily={"Roboto"} fill={"cyan"} fontSize={40}>
        https://github.com/MartyO256/variable-name-generation-tutorial
      </Txt>
    </Layout>
  );

  yield* all(layout().opacity(1, 1), layout().y(0, 1));

  yield* beginSlide("title");
});
