import { day3part1, day3part2 } from "./day3";

describe("day3part1", () => {
  it.each([
    {
      input: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`,
      output: 8,
    },
    {
      input: `Game 1: 14 blue; 12 red; 13 green`,
      output: 1,
    },
    {
      input: `Game 1: 15 blue; 12 red; 13 green`,
      output: 0,
    },
    {
      input: `Game 1: 14 blue; 13 red; 13 green`,
      output: 0,
    },
    {
      input: `Game 1: 14 blue; 12 red; 14 green`,
      output: 0,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day3part1(input)).toBe(output);
  });
});

describe("day3part2", () => {
  it.each([
    {
      input: `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`,
      output: 2286,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day3part2(input)).toBe(output);
  });
});
