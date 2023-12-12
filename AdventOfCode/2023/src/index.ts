import { day1part1, day1part2 } from "./day1/day1";
import { day1Input } from "./day1/input";
import { day10part1, day10part2 } from "./day10";
import { day10Input } from "./day10/input";
import { day11part1, day11part2 } from "./day11";
import { day11Input } from "./day11/input";
import { day12part1, day12part2 } from "./day12";
import { day12Input } from "./day12/input";
import { day2part1, day2part2 } from "./day2/day2";
import { day2Input } from "./day2/input";
import { day3part1, day3part2 } from "./day3/day3";
import { day3Input } from "./day3/input";
import { day4part1, day4part2 } from "./day4/day4";
import { day4Input } from "./day4/input";
import { day5part1, day5part2 } from "./day5";
import { day5Input } from "./day5/input";
import { day6part1, day6part2 } from "./day6/day6";
import { day6Input } from "./day6/input";
import { day7part1, day7part2 } from "./day7";
import { day7Input } from "./day7/input";
import { day8part1, day8part2 } from "./day8";
import { day8Input } from "./day8/input";
import { day9part1, day9part2 } from "./day9";
import { day9Input } from "./day9/input";

type Part = {
  number: number;
  input: string;
  function: (input: string) => unknown;
  expected?: unknown;
};

type Day = Part[];

const levels: Day[] = [
  [
    { number: 1, input: day1Input, function: day1part1, expected: 54304 },
    { number: 2, input: day1Input, function: day1part2, expected: 54418 },
  ],
  [
    { number: 1, input: day2Input, function: day2part1, expected: 3035 },
    { number: 2, input: day2Input, function: day2part2, expected: 66027 },
  ],
  [
    { number: 1, input: day3Input, function: day3part1, expected: 539590 },
    { number: 2, input: day3Input, function: day3part2, expected: 80703636 },
  ],
  [
    { number: 1, input: day4Input, function: day4part1, expected: 32609 },
    { number: 2, input: day4Input, function: day4part2, expected: 14624680 },
  ],
  [
    { number: 1, input: day5Input, function: day5part1, expected: 226172555 },
    { number: 2, input: day5Input, function: day5part2, expected: 47909639 },
  ],
  [
    { number: 1, input: day6Input, function: day6part1, expected: 1660968 },
    { number: 2, input: day6Input, function: day6part2, expected: 26499773 },
  ],
  [
    { number: 1, input: day7Input, function: day7part1, expected: 251029473 },
    { number: 2, input: day7Input, function: day7part2, expected: 251003917 },
  ],
  [
    { number: 1, input: day8Input, function: day8part1, expected: 11911 },
    {
      number: 2,
      input: day8Input,
      function: day8part2,
      expected: 10151663816849,
    },
  ],
  [
    { number: 1, input: day9Input, function: day9part1, expected: 1980437560 },
    { number: 2, input: day9Input, function: day9part2, expected: 977 },
  ],
  [
    { number: 1, input: day10Input, function: day10part1, expected: 6714 },
    { number: 2, input: day10Input, function: day10part2, expected: 429 },
  ],
  [
    { number: 1, input: day11Input, function: day11part1, expected: 9329143 },
    {
      number: 2,
      input: day11Input,
      function: day11part2,
      expected: 710674907809,
    },
  ],
  [
    { number: 1, input: day12Input, function: day12part1, expected: 7753 },
    {
      number: 2,
      input: day12Input,
      function: day12part2,
      expected: 280382734828319,
    },
  ],
  [
    // { number: 1, input: day10Input, function: day10part1, expected: 0 },
    // { number: 2, input: day10Input, function: day10part2, expected: 0 },
  ],
];

const results = levels.flatMap((day, dayIndex) =>
  day.map((part) => {
    return {
      day: dayIndex + 1,
      part: part.number,
      result: part.function(part.input),
      expected: part.expected,
    };
  })
);

console.table(results);
