import { dayXpart1, dayXpart2 } from ".";

describe.skip("dayXpart1", () => {
  it.each([
    {
      input: ``,
      output: 1,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(dayXpart1(input)).toBe(output);
  });
});

describe.skip("dayXpart2", () => {
  it.each([
    {
      input: ``,
      output: 1,
    },
  ])("should convert $input to $output", ({ input, output }) => {
    expect(dayXpart2(input)).toBe(output);
  });
});
