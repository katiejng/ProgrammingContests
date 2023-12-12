import { day5part1, day5part2, mergeRanges } from "./";

describe("day5part1", () => {
  it.each([
    {
      input: `seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
`,
      output: 35,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day5part1(input)).toBe(output);
  });
});

describe("day5part2", () => {
  it.each([
    {
      input: `seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4`,
      output: 46,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(day5part2(input)).toBe(output);
  });
});

describe("mergeRanges", () => {
  it.each([
    { input: [{ start: 1, range: 1 }], expected: [{ start: 1, range: 1 }] },
    {
      input: [
        { start: 1, range: 1 },
        { start: 1, range: 1 },
      ],
      expected: [{ start: 1, range: 1 }],
    },
    {
      input: [
        { start: 1, range: 1 },
        { start: 2, range: 1 },
      ],
      expected: [{ start: 1, range: 2 }],
    },
    {
      input: [
        { start: 1, range: 3 },
        { start: 2, range: 1 },
      ],
      expected: [{ start: 1, range: 3 }],
    },
    {
      input: [
        { start: 2, range: 3 },
        { start: 5, range: 1 },
      ],
      expected: [{ start: 2, range: 4 }],
    },
    {
      input: [
        { start: 2, range: 0 },
        { start: 2, range: 1 },
      ],
      expected: [{ start: 2, range: 1 }],
    },
    {
      input: [{ start: 74975775, range: 73896614 }],
      expected: [{ start: 74975775, range: 73896614 }],
    },
  ])("should convert $input to $expected", ({ input, expected }) => {
    expect(mergeRanges(input)).toEqual(expected);
  });
});
