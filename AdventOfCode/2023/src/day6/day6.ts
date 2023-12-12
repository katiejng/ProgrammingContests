import { isNumber } from "../utils/isNumber";
import { solveQuadraticFormula } from "../utils/math";

const parseInput = (input: string) => {
  const [time, distance] = input.split("\n").map((line) => {
    const [heading, numbers] = line.trim().split(":");
    return {
      heading: heading.trim(),
      numbers: numbers
        .trim()
        .split(" ")
        .filter((num) => num !== "")
        .map((num) => Number(num.trim())),
    };
  });

  const results = {
    time: time.numbers,
    distance: distance.numbers,
  };

  const part2 = {
    time: Number(time.numbers.join("")),
    distance: Number(distance.numbers.join("")),
  };

  const part1 = results.time.map((time, index) => ({
    time: time,
    distance: results.distance[index]!,
  }));

  return { part1, part2 };
};

export const day6part1 = (input: string) => {
  const parsedInput = parseInput(input).part1;
  return parsedInput
    .map((item) => {
      return getSolution(item);
    })
    .reduce((acc, curr) => acc * curr, 1);
};
const getSolution = (item: { time: number; distance: number }) => {
  const [solution1, solution2] = solveQuadraticFormula(
    -1,
    item.time,
    -item.distance
  );
  let minValidSolution = Math.ceil(solution1);
  let maxValidSolution = Math.floor(solution2);
  if (minValidSolution === solution1) {
    minValidSolution++;
  }
  if (maxValidSolution === solution2) {
    maxValidSolution--;
  }

  const numberOfSolutions = maxValidSolution - minValidSolution + 1;

  return numberOfSolutions;
};
export const day6part2 = (input: string) => {
  const parsedInput = parseInput(input).part2;
  return getSolution(parsedInput);
};

// forumla - 0 = -1(a)^2 + t(a) + -d
