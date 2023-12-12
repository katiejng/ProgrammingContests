import { day12part1, day12part2, legalPositions } from ".";

const input = `???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1`;

describe("day12part1", () => {
  it.each([{ input, output: 21 }])(
    "should convert $input to $output",
    ({ input, output }) => {
      expect(day12part1(input)).toBe(output);
    }
  );
});

describe("day12part2", () => {
  it.each([{ input, output: 525152 }])(
    "should convert $input to $output",
    ({ input, output }) => {
      expect(day12part2(input)).toBe(output);
    }
  );
});

describe("LegalPositions", () => {
  it.each([
    { input: { springs: "   ?", counts: [] }, output: 1 },
    { input: { springs: "?????", counts: [] }, output: 1 },
    { input: { springs: "#", counts: [1] }, output: 1 },
    { input: { springs: "?", counts: [1] }, output: 1 },
    { input: { springs: "", counts: [1] }, output: 0 },
    { input: { springs: "#", counts: [2] }, output: 0 },
    { input: { springs: "# #", counts: [2] }, output: 0 },
    { input: { springs: "#", counts: [1] }, output: 1 },
    { input: { springs: "#", counts: [1, 1] }, output: 0 },
    {
      input: { springs: "????#???????#?????", counts: [1, 8, 1, 1, 1] },
      output: 9,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    const result = legalPositions(input.springs, input.counts);
    expect(result).toEqual(output);
  });
});
