import { day1part1, day1part2 } from "./day1";

describe("day1part1", () => {
  it.each([
    {
      input: `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`,
      output: 142,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day1part1(input)).toBe(output);
  });
});

describe("day1part2", () => {
  it.each([
    {
      input: `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`,
      output: 281,
    },
    { input: `oneone`, output: 11 },
    { input: `twoeightwo`, output: 22 },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day1part2(input)).toBe(output);
  });
});
