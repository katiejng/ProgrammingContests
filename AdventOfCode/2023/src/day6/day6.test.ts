import { day6part1, day6part2 } from "./day6";

describe("day6part1", () => {
  it.each([
    {
      input: `Time:      7  15   30
Distance:  9  40  200`,
      output: 288,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day6part1(input)).toBe(output);
  });
});

describe("day6part2", () => {
  it.each([
    {
      input: `Time:      7  15   30
Distance:  9  40  200`,
      output: 71503,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day6part2(input)).toBe(output);
  });
});
