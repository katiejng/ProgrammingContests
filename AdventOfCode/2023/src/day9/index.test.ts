import { day9part1, day9part2 } from ".";
const input = `0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45`;
describe("day9part1", () => {
  it.each([
    {
      input,
      output: 114,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day9part1(input)).toBe(output);
  });
});

describe("day9part2", () => {
  it.each([
    {
      input,
      output: 2,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day9part2(input)).toBe(output);
  });
});
