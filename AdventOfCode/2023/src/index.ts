import { day1part1, day1part2 } from "./day1/day1";
import { day1part1Input, day1part2Input } from "./day1/input";
import { day2part1, day2part2 } from "./day2/day2";
import { day2part1Input } from "./day2/input";

type Part = {
  number: number;
  input: string;
  function: (input: string) => unknown;
};
type Day = {
  number: number;
  parts: Part[];
};

const levels: Day[] = [
  {
    number: 1,
    parts: [
      { number: 1, input: day1part1Input, function: day1part1 },
      { number: 2, input: day1part2Input, function: day1part2 },
    ],
  },
  {
    number: 2,
    parts: [
      { number: 1, input: day2part1Input, function: day2part1 },
      { number: 2, input: day2part1Input, function: day2part2 },
    ],
  },
];

levels.forEach((day) => {
  day.parts.forEach((part) => {
    console.log(
      `Day ${day.number} Part ${part.number}: ${part.function(part.input)}`
    );
  });
});
