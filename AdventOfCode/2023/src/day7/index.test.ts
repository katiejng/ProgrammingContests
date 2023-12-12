import { day7part1, day7part2 } from ".";

const testInput = `32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483`;
describe("day7part1", () => {
  it.each([
    {
      input: testInput,
      output: 6440,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day7part1(input)).toBe(output);
  });
});

describe("day7part2", () => {
  it.each([
    {
      input: testInput,
      output: 5905,
    },
    {
      input: `JJJJJ 765`,
      output: 765,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day7part2(input)).toBe(output);
  });
});
