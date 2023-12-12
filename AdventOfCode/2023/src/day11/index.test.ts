import { day11part1, day11part2 } from ".";

const input = `...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....`;
describe("day11part1", () => {
  it.each([
    {
      input,
      output: 374,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day11part1(input)).toBe(output);
  });
});

describe("day11part2", () => {
  it.each([
    {
      input,
      expansionFactor: 2,
      output: 374,
    },
    {
      input,
      expansionFactor: 10,
      output: 1030,
    },
    {
      input,
      expansionFactor: 100,
      output: 8410,
    },
  ])(
    "should convert $input to $output",
    ({ input, expansionFactor, output }) => {
      expect(day11part2(input, expansionFactor)).toBe(output);
    }
  );
});
