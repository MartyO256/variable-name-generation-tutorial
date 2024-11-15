import { Latex, Layout, makeScene2D, Rect, Txt } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";
import { backgroundFill } from "../constants";

export default makeScene2D(function* (view) {
  view.fontSize(50);
  view.fontFamily("Roboto");
  view.fill(backgroundFill);

  const envWf = createRef<Latex>();
  const termWf = createRef<Latex>();
  const indexing = createRef<Latex>();
  const naming = createRef<Latex>();
  const alphaEquiv = createRef<Latex>();

  view.add(
    <Layout direction={"column"} gap={40} layout>
      <Layout
        ref={envWf}
        opacity={0.1}
        direction={"row"}
        gap={40}
        alignItems={"baseline"}
        layout
      >
        <Txt fill={"white"}>Well-formed context:</Txt>
        <Latex tex={"\\vdash \\Sigma : \\mathsf{env}"} fill="white" />
      </Layout>
      <Layout
        ref={termWf}
        opacity={0.1}
        direction={"row"}
        gap={40}
        alignItems={"baseline"}
        layout
      >
        <Txt fill={"white"}>Well-formed term:</Txt>
        <Latex tex={"\\Sigma \\vdash M : \\mathsf{term}"} fill="white" />
      </Layout>
      <Layout
        ref={indexing}
        opacity={0.1}
        gap={40}
        alignItems={"baseline"}
        layout
      >
        <Txt fill={"white"}>To nameless:</Txt>
        <Latex
          tex={"\\Sigma \\vdash M \\rightsquigarrow \\tilde{M}"}
          fill="white"
        />
      </Layout>
      <Layout
        ref={naming}
        opacity={0.1}
        direction={"row"}
        gap={40}
        alignItems={"baseline"}
        layout
      >
        <Txt fill={"white"}>To named:</Txt>
        <Latex
          tex={"\\Sigma \\vdash \\tilde{M} \\hookrightarrow M'"}
          fill="white"
        />
      </Layout>
      <Layout
        ref={alphaEquiv}
        opacity={0.1}
        direction={"row"}
        gap={40}
        alignItems={"center"}
        layout
      >
        <Txt fill={"white"}>Equivalence up to naming:</Txt>
        <Latex tex={"\\Sigma \\vdash M =_{\\alpha} M'"} fill="white" />
      </Layout>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("envWf");

  yield* envWf().opacity(1, 0.5);

  yield* beginSlide("termWf");

  yield* termWf().opacity(1, 0.5);

  yield* beginSlide("indexing");

  yield* indexing().opacity(1, 0.5);

  yield* beginSlide("naming");

  yield* naming().opacity(1, 0.5);

  yield* beginSlide("alphaEquiv");

  yield* alphaEquiv().opacity(1, 0.5);

  yield* beginSlide("roundabout");
});
