import {
  day10part1,
  day10part2,
  getSubpipePosition,
  getSubpointPoint,
} from ".";
const input1 = `-L|F7
7S-7|
L|7||
-L-J|
L|-JF`;

const input2 = `..F7.
.FJ|.
SJ.L7
|F--J
LJ...`;
describe("day10part1", () => {
  it.each([
    {
      input: input1,
      output: 4,
    },
    {
      input: input2,
      output: 8,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day10part1(input)).toBe(output);
  });
});

describe("day10part2", () => {
  it.each([
    {
      input: `..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........`,
      output: 4,
    },
    {
      input: `.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...`,
      output: 8,
    },
    {
      input: `FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L`,
      output: 10,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day10part2(input)).toBe(output);
  });
});

describe("getSubpointPoint", () => {
  it.each([
    {
      subPoint: { r: 0, c: 0 },
      expected: { parentPoint: { r: 0, c: 0 }, subPointOffset: 0 },
    },
    {
      subPoint: { r: 0, c: 1 },
      expected: { parentPoint: { r: 0, c: 0 }, subPointOffset: 1 },
    },
    {
      subPoint: { r: 1, c: 0 },
      expected: { parentPoint: { r: 0, c: 0 }, subPointOffset: 2 },
    },
    {
      subPoint: { r: 1, c: 1 },
      expected: { parentPoint: { r: 0, c: 0 }, subPointOffset: 3 },
    },
    {
      subPoint: { r: 2, c: 0 },
      expected: { parentPoint: { r: 1, c: 0 }, subPointOffset: 0 },
    },
    {
      subPoint: { r: 2, c: 1 },
      expected: { parentPoint: { r: 1, c: 0 }, subPointOffset: 1 },
    },
    {
      subPoint: { r: 1, c: 2 },
      expected: { parentPoint: { r: 0, c: 1 }, subPointOffset: 2 },
    },
    {
      subPoint: { r: 2, c: 2 },
      expected: { parentPoint: { r: 1, c: 1 }, subPointOffset: 0 },
    },
  ])("should get the subpoint", ({ subPoint, expected }) => {
    expect(getSubpointPoint(subPoint)).toEqual(expected);
  });
});
