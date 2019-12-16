import input from "./in16-1";
function calculate(
  input: Array<number>,
  phases: Array<number>,
  phase: number,
  index: number
) {
  let result = input.map((value, resultIndex) => {
    let phaseIndex =
      Math.floor((resultIndex + 1) / (index + 1)) % phases.length;
    return [value, phases[phaseIndex]];
  });
  return result;
}

function main(input: string, phases: Array<number>) {
  let current;
  let next = input.split("").map(x => parseInt(x));
  for (let i = 0; i < 100; i++) {
    current = next;
    next = [];
    for (let j = 0; j < current.length; j++) {
      let val = calculate(current, phases, i, j);
      console.log(val.join("|"));
    }
  }
}
const phase = [0, 1, 0, -1];
main(input.zero, phase);
