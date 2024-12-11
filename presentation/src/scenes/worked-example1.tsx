import { Latex, Layout, makeScene2D, Polygon, Txt } from "@motion-canvas/2d";
import {
  all,
  beginSlide,
  createRef,
  createSignal,
  Direction,
  slideTransition,
  unwrap,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const layoutRef = createRef<Layout>();

  const lambda1Layout = createRef<Layout>();
  const lambda1 = createRef<Latex>();

  const lambda2Layout = createRef<Layout>();
  const lambda2 = createRef<Latex>();

  const lambda3Layout = createRef<Layout>();
  const lambda3 = createRef<Latex>();

  const vxLayout = createRef<Layout>();
  const vx = createRef<Latex>();

  const vyLayout = createRef<Layout>();
  const vy = createRef<Latex>();

  const vz1Layout = createRef<Layout>();
  const vz1 = createRef<Latex>();

  const vz2Layout = createRef<Layout>();
  const vz2 = createRef<Latex>();

  view.add(
    <Layout
      ref={layoutRef}
      direction={"row"}
      gap={50}
      alignItems={"center"}
      layout
    >
      <Layout
        ref={lambda1Layout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex
          ref={lambda1}
          tex={["\\vphantom{(}\\lambda{}", "", "."]}
          fill="white"
          fontSize={70}
        />
      </Layout>
      <Layout
        ref={lambda2Layout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex
          ref={lambda2}
          tex={["\\vphantom{(}\\lambda{}", "", "."]}
          fill="white"
          fontSize={70}
        />
      </Layout>
      <Layout
        ref={lambda3Layout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex
          ref={lambda3}
          tex={["\\vphantom{(}\\lambda{}", "", "."]}
          fill="white"
          fontSize={70}
        />
      </Layout>
      <Layout
        ref={vxLayout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex ref={vx} tex={"\\vphantom{(}3"} fill="white" fontSize={70} />
      </Layout>
      <Layout
        ref={vz1Layout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex ref={vz1} tex={"\\vphantom{(}1"} fill="white" fontSize={70} />
      </Layout>
      <Layout
        ref={vyLayout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex ref={vy} tex={["(", "2"]} fill="white" fontSize={70} />
      </Layout>
      <Layout
        ref={vz2Layout}
        width={50}
        height={50}
        alignItems={"center"}
        alignContent={"center"}
      >
        <Latex ref={vz2} tex={["1", ")"]} fill="white" fontSize={70} />
      </Layout>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("worked-example");

  yield* layoutRef().gap(180, 1);

  yield* beginSlide("worked-example");

  const cursor1X = createSignal<number>(unwrap<number>(lambda1Layout().x));
  const cursor1Y = createSignal<number>(
    unwrap<number>(lambda1Layout().y) - 100
  );

  const cursor1 = createRef<Polygon>();
  view.add(
    <Polygon
      ref={cursor1}
      sides={3}
      size={50}
      fill={"red"}
      rotation={180}
      opacity={0}
      x={cursor1X}
      y={cursor1Y}
    />
  );

  const cursor2X = createSignal<number>(unwrap<number>(lambda1Layout().x));
  const cursor2Y = createSignal<number>(
    unwrap<number>(lambda1Layout().y) - 100
  );

  const cursor2 = createRef<Polygon>();
  view.add(
    <Polygon
      ref={cursor2}
      sides={3}
      size={50}
      fill={"cyan"}
      rotation={180}
      opacity={0}
      x={cursor2X}
      y={cursor2Y}
    />
  );

  yield* cursor1().opacity(1, 1);

  yield* beginSlide("worked-example");

  const binder1 = createRef<Layout>();
  const u1 = createRef<Latex>();
  const constraints1 = createRef<Latex>();
  const used1 = createRef<Txt>();

  view.add(
    <Layout
      ref={binder1}
      direction={"column"}
      gap={40}
      x={lambda1Layout().x}
      top={lambda1Layout().bottom}
      alignItems={"center"}
      paddingTop={100}
      opacity={0}
      layout
    >
      <Layout width={50} height={50} layout>
        <Latex ref={u1} tex={["u_1"]} fill="white" fontSize={50} />
      </Layout>
      <Latex
        ref={constraints1}
        tex={["\\{", "\\}"]}
        fill="white"
        fontSize={50}
      />
      <Txt ref={used1} fontFamily={"Roboto"} fill={"white"}>
        Unused
      </Txt>
    </Layout>
  );

  yield* binder1().opacity(1, 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(lambda2Layout().x), 1);

  yield* beginSlide("worked-example");

  const binder2 = createRef<Layout>();
  const u2 = createRef<Latex>();
  const constraints2 = createRef<Latex>();
  const used2 = createRef<Txt>();

  view.add(
    <Layout
      ref={binder2}
      direction={"column"}
      gap={40}
      x={lambda2Layout().x}
      top={lambda2Layout().bottom}
      alignItems={"center"}
      paddingTop={100}
      opacity={0}
      layout
    >
      <Layout width={50} height={50} layout>
        <Latex ref={u2} tex={["u_2"]} fill="white" fontSize={50} />
      </Layout>
      <Latex
        ref={constraints2}
        tex={["\\{", "\\}"]}
        fill="white"
        fontSize={50}
      />
      <Txt ref={used2} fontFamily={"Roboto"} fill={"white"}>
        Unused
      </Txt>
    </Layout>
  );

  yield* binder2().opacity(1, 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(lambda3Layout().x), 1);

  yield* beginSlide("worked-example");

  const binder3 = createRef<Layout>();
  const u3 = createRef<Latex>();
  const constraints3 = createRef<Latex>();
  const used3 = createRef<Txt>();

  view.add(
    <Layout
      ref={binder3}
      direction={"column"}
      gap={40}
      x={lambda3Layout().x}
      top={lambda3Layout().bottom}
      alignItems={"center"}
      paddingTop={100}
      opacity={0}
      layout
    >
      <Layout width={50} height={50} layout>
        <Latex ref={u3} tex={["u_3"]} fill="white" fontSize={50} />
      </Layout>
      <Latex
        ref={constraints3}
        tex={["\\{", "\\}"]}
        fill="white"
        fontSize={50}
      />
      <Txt ref={used3} fontFamily={"Roboto"} fill={"white"}>
        Unused
      </Txt>
    </Layout>
  );

  yield* binder3().opacity(1, 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vxLayout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor2X(unwrap<number>(cursor1X), 0),
    cursor2Y(unwrap<number>(cursor1Y), 0),
    lambda1().fill("cyan", 1)
  );

  yield* beginSlide("worked-example");

  yield* all(
    cursor2().opacity(1, 1),
    cursor2X(unwrap<number>(lambda3Layout().x), 1)
  );

  yield* beginSlide("worked-example");

  yield* constraints3().tex(["\\{", "u_1", "\\}"], 1);

  yield* beginSlide("worked-example");

  yield* cursor2X(unwrap<number>(lambda2Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* constraints2().tex(["\\{", "u_1", "\\}"], 1);

  yield* beginSlide("worked-example");

  yield* cursor2X(unwrap<number>(lambda1Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(cursor2().opacity(0, 1), lambda1().fill("white", 1));

  yield* used1().text("", 0.5);
  yield* used1().text("Used", 0.5);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vz1Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor2X(unwrap<number>(cursor1X), 0),
    cursor2Y(unwrap<number>(cursor1Y), 0),
    lambda3().fill("cyan", 1)
  );

  yield* beginSlide("worked-example");

  yield* all(
    cursor2().opacity(1, 1),
    cursor2X(unwrap<number>(lambda3Layout().x), 1)
  );

  yield* beginSlide("worked-example");

  yield* all(cursor2().opacity(0, 1), lambda3().fill("white", 1));

  yield* used3().text("", 0.5);
  yield* used3().text("Used", 0.5);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vyLayout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor2X(unwrap<number>(cursor1X), 0),
    cursor2Y(unwrap<number>(cursor1Y), 0),
    lambda2().fill("cyan", 1)
  );

  yield* beginSlide("worked-example");

  yield* all(
    cursor2().opacity(1, 1),
    cursor2X(unwrap<number>(lambda3Layout().x), 1)
  );

  yield* beginSlide("worked-example");

  yield* constraints3().tex(["\\{", "u_1", ",", "u_2", "\\}"], 1);

  yield* beginSlide("worked-example");

  yield* cursor2X(unwrap<number>(lambda2Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(cursor2().opacity(0, 1), lambda2().fill("white", 1));

  yield* used2().text("", 0.5);
  yield* used2().text("Used", 0.5);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vz2Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor2X(unwrap<number>(cursor1X), 0),
    cursor2Y(unwrap<number>(cursor1Y), 0),
    lambda3().fill("cyan", 1)
  );

  yield* beginSlide("worked-example");

  yield* all(
    cursor2().opacity(1, 1),
    cursor2X(unwrap<number>(lambda3Layout().x), 1)
  );

  yield* beginSlide("worked-example");

  yield* all(cursor2().opacity(0, 1), lambda3().fill("white", 1));

  yield* beginSlide("worked-example");

  yield* cursor1().opacity(0, 1);

  yield* beginSlide("worked-example");

  const nameSequence = createRef<Latex>();

  view.add(
    <Latex
      ref={nameSequence}
      tex={["(x, y, z, x_1, y_1, z_1, x_2, y_2, z_2, \\dots)"]}
      fill="white"
      fontSize={50}
      y={-300}
      opacity={0}
    />
  );

  yield* nameSequence().opacity(1, 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor1X(unwrap<number>(lambda1Layout().x), 0),
    cursor1().opacity(1, 1)
  );

  yield* beginSlide("worked-example");

  yield* all(
    u1().tex(["x"], 1),
    lambda1().tex(["\\vphantom{(}\\lambda{}", "x", "."], 1),
    constraints2().tex(["\\{", "x", "\\}"], 1),
    constraints3().tex(["\\{", "x,", "u_2", "\\}"], 1)
  );

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(lambda2Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    u2().tex(["y"], 1),
    lambda2().tex(["\\vphantom{(}\\lambda{}", "y", "."], 1),
    constraints3().tex(["\\{", "x,", "y", "\\}"], 1)
  );

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(lambda3Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* all(
    u3().tex(["z"], 1),
    lambda3().tex(["\\vphantom{(}\\lambda{}", "z", "."], 1)
  );

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vxLayout().x), 1);

  yield* beginSlide("worked-example");

  yield* vx().tex(["\\vphantom{(}x"], 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vz1Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* vz1().tex(["\\vphantom{(}z"], 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vyLayout().x), 1);

  yield* beginSlide("worked-example");

  yield* vy().tex(["(", "y"], 1);

  yield* beginSlide("worked-example");

  yield* cursor1X(unwrap<number>(vz2Layout().x), 1);

  yield* beginSlide("worked-example");

  yield* vz2().tex(["z", ")"], 1);

  yield* beginSlide("worked-example");

  yield* all(
    cursor1().opacity(0, 1),
    nameSequence().opacity(0, 1),
    binder1().opacity(0, 1),
    binder2().opacity(0, 1),
    binder3().opacity(0, 1)
  );

  yield* beginSlide("worked-example");
});
