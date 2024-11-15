import { makeProject } from "@motion-canvas/core";

import title from "./scenes/title?scene";
import motivation from "./scenes/motivation?scene";
import grammar from "./scenes/grammar?scene";

import "./global.css";

export default makeProject({
  name: "Variable Name Generation Tutorial",
  scenes: [title, motivation, grammar],
});
