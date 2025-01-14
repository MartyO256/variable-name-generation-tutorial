import { Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import {
  all,
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const layout = createRef<Layout>();
  const url = createRef<Txt>();

  view.add(
    <Layout
      ref={layout}
      direction={"column"}
      alignItems={"center"}
      gap={20}
      layout
    >
      <Txt ref={url} fontFamily={"Roboto"} fill={"cyan"} fontSize={50}>
        https://github.com/MartyO256/variable-name-generation-tutorial
      </Txt>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("repository-link");
});
