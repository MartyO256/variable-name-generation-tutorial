import { Latex, Layout, makeScene2D, Txt } from "@motion-canvas/2d";
import {
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  const constraint1 = createRef<Layout>();
  const constraint2 = createRef<Layout>();
  const constraint3 = createRef<Layout>();
  const constraint4 = createRef<Layout>();

  view.add(
    <Layout direction={"column"} gap={40} width={1920} padding={100} layout>
      <Txt fontFamily={"Roboto"} fill={"white"} fontSize={100}>
        Solution
      </Txt>
      <Txt fontFamily={"Roboto"} fill={"white"} textWrap={true}>
        This can be formulated as a constraint satisfaction problem.
      </Txt>
      <Layout direction={"row"} gap={20}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          Variables:
        </Txt>
        <Latex tex={["u_i"]} fill="white" fontSize={54} marginTop={15} />
        <Txt fontFamily={"Roboto"} fill={"white"}>
          represents the parameter for binder
        </Txt>
        <Latex tex={["i"]} fill="white" fontSize={54} marginTop={5} />
      </Layout>
      <Layout direction={"row"} gap={20}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          Domains:
        </Txt>
        <Latex
          tex={["D_i = \\{\\textunderscore, x, x_1, x_2, \\dots\\}"]}
          fill="white"
          fontSize={54}
        />
      </Layout>
      <Layout direction={"column"} gap={20}>
        <Txt fontFamily={"Roboto"} fill={"white"}>
          Constraints:
        </Txt>
        <Layout ref={constraint1} opacity={0.1}>
          <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
            1. If a variable is free, then all parent binders cannot use its
            name
          </Txt>
        </Layout>
        <Layout
          direction={"column"}
          gap={20}
          marginLeft={20}
          ref={constraint2}
          opacity={0.1}
        >
          <Layout direction={"row"}>
            <Txt fontFamily={"Roboto"} fill={"white"}>
              2. If a variable is bound to a binder having parameter
            </Txt>
            <Latex
              tex={["u_i"]}
              fill="white"
              fontSize={54}
              marginLeft={20}
              marginTop={15}
            />
            <Txt fontFamily={"Roboto"} fill={"white"}>
              , then the binders with
            </Txt>
          </Layout>
          <Layout direction={"row"} marginLeft={50}>
            <Txt fontFamily={"Roboto"} fill={"white"}>
              with a lesser distance cannot use parameter
            </Txt>
            <Latex
              tex={["u_i"]}
              fill="white"
              fontSize={54}
              marginLeft={20}
              marginRight={20}
              marginTop={15}
            />
          </Layout>
        </Layout>
        <Layout
          direction={"column"}
          gap={20}
          marginLeft={20}
          ref={constraint3}
          opacity={0.1}
        >
          <Layout direction={"row"}>
            <Txt fontFamily={"Roboto"} fill={"white"}>
              3. If a variable is bound to a binder having parameter
            </Txt>
            <Latex
              tex={["u_i"]}
              fill="white"
              fontSize={54}
              marginLeft={20}
              marginTop={15}
            />
            <Txt fontFamily={"Roboto"} fill={"white"}>
              , then
            </Txt>
            <Latex
              tex={["u_i"]}
              fill="white"
              fontSize={54}
              marginLeft={20}
              marginRight={20}
              marginTop={15}
            />
            <Txt fontFamily={"Roboto"} fill={"white"}>
              cannot use
            </Txt>
          </Layout>
          <Layout direction={"row"} marginLeft={50}>
            <Txt fontFamily={"Roboto"} fill={"white"}>
              the parameter names for binders with lesser distances
            </Txt>
          </Layout>
        </Layout>
        <Layout ref={constraint4} opacity={0.1}>
          <Txt fontFamily={"Roboto"} fill={"white"} marginLeft={20}>
            4. Every binder that is used must have a parameter name
          </Txt>
        </Layout>
      </Layout>
    </Layout>
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("solution");

  yield* constraint1().opacity(1, 1);

  yield* beginSlide("solution");

  yield* constraint2().opacity(1, 1);

  yield* beginSlide("solution");

  yield* constraint3().opacity(1, 1);

  yield* beginSlide("solution");

  yield* constraint4().opacity(1, 1);

  yield* beginSlide("solution");
});
