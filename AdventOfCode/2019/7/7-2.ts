import input from "./in7-2";
import { IntCodeComputer } from "./IntCode";

function doItLikeBefore(phases: Array<number>, args: Array<number>) {
  const compA = new IntCodeComputer("A", [...args], [phases[0], 0]);
  const compB = new IntCodeComputer("B", [...args], [phases[1]]);
  const compC = new IntCodeComputer("C", [...args], [phases[2]]);
  const compD = new IntCodeComputer("D", [...args], [phases[3]]);
  const compE = new IntCodeComputer("E", [...args], [phases[4]]);

  const computers = [compA, compB, compC, compD, compE];
  let inputs = [];
  for (let i = 0; i < computers.length; i++) {
    computers[i].inputs.push(...inputs);
    computers[i].processList();
    inputs = [...computers[i].outputs];
    computers[i].outputs = [];
  }
  return inputs;
}
function doIt(phases: Array<number>, args: Array<number>) {
  const compA = new IntCodeComputer("A", [...args], [phases[0], 0]);
  const compB = new IntCodeComputer("B", [...args], [phases[1]]);
  const compC = new IntCodeComputer("C", [...args], [phases[2]]);
  const compD = new IntCodeComputer("D", [...args], [phases[3]]);
  const compE = new IntCodeComputer("E", [...args], [phases[4]]);

  const computers = [compA, compB, compC, compD, compE];
  let inputs = [];
  let i = 0;
  let computer = computers[i];

  while (!computer.isDead) {
    computer.inputs.push(...inputs);
    // computer.printAll();
    computer.processList();
    // computer.printAll();

    inputs = [...computer.outputs];
    computer.outputs = [];
    i = (i += 1) % computers.length;

    computer = computers[i];
  }
  return inputs;
}

function permute(permutation: Array<number>) {
  var length = permutation.length,
    result = [permutation.slice()],
    c = new Array(length).fill(0),
    i = 1,
    k,
    p;

  while (i < length) {
    if (c[i] < i) {
      k = i % 2 && c[i];
      p = permutation[i];
      permutation[i] = permutation[k];
      permutation[k] = p;
      ++c[i];
      i = 1;
      result.push(permutation.slice());
    } else {
      c[i] = 0;
      ++i;
    }
  }
  return result;
}

function main() {
  console.log("KATIE", new Date().toString());
  const DONT_USE_GLOBAL = input.value.split(",").map(x => parseInt(x));
  const permutations = permute([5, 6, 7, 8, 9]);
  const results = [];
  for (let i = 0; i < permutations.length; i++) {
    results.push(doIt(permutations[i], DONT_USE_GLOBAL)[0]);
  }
  return Math.max(...results);
}
console.log(main());

export { main, permute, doItLikeBefore, doIt };

//4248984
