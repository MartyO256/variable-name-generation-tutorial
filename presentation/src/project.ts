import { makeProject } from "@motion-canvas/core";

import title from "./scenes/title?scene";
import motivation from "./scenes/motivation?scene";
import problem from "./scenes/problem?scene";
import grammar from "./scenes/grammar?scene";
import parsing from "./scenes/parsing?scene";
import parsingImplementation1 from "./scenes/parsing-implementation1?scene";
import parsingImplementation2 from "./scenes/parsing-implementation2?scene";
import parsingImplementation3 from "./scenes/parsing-implementation3?scene";
import printingImplementation1 from "./scenes/printing-implementation1?scene";
import printingImplementation2 from "./scenes/printing-implementation2?scene";
import printingImplementation3 from "./scenes/printing-implementation3?scene";
import roundabout from "./scenes/roundabout?scene";

import "./global.css";

export default makeProject({
  name: "Variable Name Generation Tutorial",
  scenes: [
    title,
    motivation,
    problem,
    grammar,
    parsing,
    parsingImplementation1,
    parsingImplementation2,
    parsingImplementation3,
    printingImplementation1,
    printingImplementation2,
    printingImplementation3,
    roundabout,
  ],
});
