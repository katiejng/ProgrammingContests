type SpringProblem = {
  springs: string;
  counts: number[];
};

const parseInput = (input: string): SpringProblem[] => {
  return input.split("\n").map((line) => {
    const [damagedString, numsString] = line.split(" ");
    const counts = numsString.split(",").map((num) => Number(num.trim()));

    return {
      springs: damagedString
        .split("")
        .map((char) => (char === "." ? " " : char))
        .join(""),
      counts,
    };
  });
};

const memoize = (fn: Function) => {
  const cache: Record<string, number> = {};

  return (springs: string, counts: number[]): number => {
    const key = `${springs} ${counts.join(" ")}`;
    if (key in cache) {
      return cache[key];
    }
    const result = fn(springs, counts);
    cache[key] = result;
    return result;
  };
};

export const legalPositions = memoize(
  (springs: string, counts: number[]): number => {
    let spring = springs.trimStart(); // Do not care about extra . at the start

    if (spring === "") {
      if (counts.length === 0) {
        // Valid case `spring: "", counts: []`
        return 1;
      }
      // Impossible - can't have more counts than springs
      return 0;
    }

    if (counts.length === 0) {
      if (spring.includes("#")) {
        // Impossible - can't have more springs than count
        return 0;
      }
      // Valid case `spring: "?????", counts: []`
      return 1;
    }
    if (spring[0] == "#") {
      if (
        spring.length < counts[0] ||
        spring.slice(0, counts[0]).includes(" ")
      ) {
        return 0; // Impossible - not enough space for the spring
      }
      if (spring.length === counts[0]) {
        return counts.length === 1 ? 1 : 0;
      }
      if (spring[counts[0]] === "#") {
        return 0;
      }

      // calculate solutions if we remove the first spring
      return legalPositions(
        spring.slice(counts[0] + 1, spring.length),
        counts.slice(1, counts.length)
      );
    }

    // calculate solutions if we replace the first spring with a # or a .
    return (
      legalPositions("#" + spring.slice(1, spring.length), counts) +
      legalPositions(spring.slice(1, spring.length), counts)
    );
  }
);

export const day12part1 = (input: string) => {
  const parsedInput = parseInput(input);

  return parsedInput
    .map(({ springs, counts }) => legalPositions(springs, counts))
    .reduce((acc, curr) => acc + curr, 0);
};

export const day12part2 = (input: string) => {
  const parsedInput = parseInput(input);

  const mappedSprings = parsedInput.map((input) => ({
    springs: new Array(5).fill(input.springs).join("?"),
    counts: new Array(5).fill(input.counts).flat(),
  }));

  return mappedSprings
    .map(({ springs, counts }) => {
      return legalPositions(springs, counts);
    })
    .reduce((acc, curr) => acc + curr, 0);
};
