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

describe.skip("day11part2", () => {
  it.each([
    {
      input: ``,
      output: 1,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day11part2(input)).toBe(output);
  });
});
