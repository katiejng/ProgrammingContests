import { day3part1, day3part2 } from "./day3";

describe("day3part1", () => {
  it.each([
    {
      input: `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`,
      output: 4361,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day3part1(input)).toBe(output);
  });
});

describe("day3part2", () => {
  it.each([
    {
      input: `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`,
      output: 467835,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day3part2(input)).toBe(output);
  });
});
