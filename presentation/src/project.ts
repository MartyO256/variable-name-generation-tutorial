import { makeProject } from "@motion-canvas/core";

import title from "./scenes/title?scene";
import motivation from "./scenes/motivation?scene";
import problem from "./scenes/problem?scene";
import solution from "./scenes/solution?scene";
import solutionImplementation from "./scenes/solution-implementation?scene";
import workedExample1 from "./scenes/worked-example1?scene";
import workedExample2 from "./scenes/worked-example2?scene";
import workedExample3 from "./scenes/worked-example3?scene";
import workedExample4 from "./scenes/worked-example4?scene";
import grammar from "./scenes/grammar?scene";
import ast from "./scenes/ast?scene";
import expressionArena from "./scenes/expression-arena?scene";
import expressionArenaExample from "./scenes/expression-arena-example?scene";
import expressionArenaCaveat from "./scenes/expression-arena-caveat?scene";

import "./global.css";

export default makeProject({
  name: "Variable Name Generation Tutorial",
  scenes: [
    title,
    motivation,
    problem,
    solution,
    solutionImplementation,
    workedExample1,
    workedExample2,
    workedExample3,
    workedExample4,
    grammar,
    ast,
    expressionArena,
    expressionArenaExample,
    expressionArenaCaveat,
  ],
});
