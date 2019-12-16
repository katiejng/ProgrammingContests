import input from "./in9-2";
import { IntCodeComputer } from "./IntCode";

function main(codes: Array<number>, input: Array<number> = []) {
  const computer = new IntCodeComputer("A", codes, input);
  computer.processList();

  return computer.outputs;
}

export { main };

// const codes = input.one.split(",").map(x => parseInt(x));
// const res = main(codes);

// console.log(res);
//4248984
