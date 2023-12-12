import { day8part1, day8part2 } from ".";
const input1 = `RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)`;

const input2 = `LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)`;

describe("day8part1", () => {
  it.each([
    {
      input: input1,
      output: 2,
    },
    {
      input: input2,
      output: 6,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day8part1(input)).toBe(output);
  });
});

describe("day8part2", () => {
  it.each([
    {
      input: `LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)`,
      output: 6,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day8part2(input)).toBe(output);
  });
});
