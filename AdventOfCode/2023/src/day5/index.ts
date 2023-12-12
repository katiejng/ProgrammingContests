type Mapping = {
  sourceName: string;
  destinationName: string;
  content: {
    destination: number;
    range: number;
    start: number;
  }[];
};

const compareRange = <T extends SeedRange>(a: T, b: T) => {
  if (a.start === b.start) {
    return a.range - b.range;
  }
  return a.start - b.start;
};

const parseInput = (input: string) => {
  const sections = input.split("\n\n");
  const [seeds, ...rest] = sections;
  const seedNumbers = seeds
    .split(":")[1]
    .trim()
    .split(" ")
    .map((num) => Number(num.trim()));

  const mappings = rest.map((section) => {
    const [title, content] = section.trim().split(":");
    const [sourceName, _, destinationName] = title
      .trim()
      .split(" ")[0]
      .split("-");
    const mappedContent = content
      .trim()
      .split("\n")
      .map((numbers) => {
        const [destination, source, range] = numbers
          .split(" ")
          .map((number) => Number(number.trim()));
        return {
          destination,
          range,
          start: source,
        };
      });

    mappedContent.sort(compareRange);

    return {
      sourceName,
      destinationName,
      content: mappedContent,
    };
  });
  return {
    seedNumbers,
    mappings,
  };
};

export const day5part1 = (input: string) => {
  const parsedInput = parseInput(input);

  return iterate(parsedInput.mappings, parsedInput.seedNumbers);
};

const iterate = (mappings: Mapping[], seeds: number[]) => {
  let mappedItems = seeds;
  let currentType = "seed";

  while (true) {
    const correctMap = mappings.find(
      (mapping) => mapping.sourceName === currentType
    )!;
    currentType = correctMap.destinationName;
    mappedItems = mapItems(correctMap, mappedItems);

    if (currentType === "location") {
      return Math.min(...mappedItems);
    }
  }
};

type SeedRange = { start: number; range: number };

export const day5part2 = (input: string) => {
  const parsedInput = parseInput(input);
  let seedRanges: SeedRange[] = [];

  for (let i = 0; i < parsedInput.seedNumbers.length; i += 2) {
    const start = parsedInput.seedNumbers[i];
    const range = parsedInput.seedNumbers[i + 1];

    seedRanges.push({ start, range });
  }
  seedRanges.sort(compareRange);
  seedRanges = mergeRanges(seedRanges);

  return Math.min(
    ...iteratePart2(parsedInput.mappings, seedRanges).map((item) => item.start)
  );
};

export const mergeRanges = (ranges: SeedRange[]) => {
  ranges.sort(compareRange);
  const mergedRanges: SeedRange[] = [ranges[0]];

  ranges.forEach((seedRange) => {
    const lastSeedRange = mergedRanges[mergedRanges.length - 1];

    if (lastSeedRange.start + lastSeedRange.range >= seedRange.start) {
      const rangeEnd1 = seedRange.start + seedRange.range;
      const rangeEnd2 = lastSeedRange.start + lastSeedRange.range;

      const biggerRange = Math.max(rangeEnd1, rangeEnd2);
      const newRange = biggerRange - lastSeedRange.start;
      mergedRanges[mergedRanges.length - 1].range = newRange;
    } else {
      mergedRanges.push(seedRange);
    }
  });

  return mergedRanges;
};

const iteratePart2 = (mappings: Mapping[], seeds: SeedRange[]) => {
  let currentType = "seed";
  let currentRanges = seeds;

  while (true) {
    // console.log({
    //   currentType,
    //   currentRanges,
    // });
    if (currentType === "location") {
      return currentRanges;
    }

    let nextRanges: SeedRange[] = [];

    const currentTypeMap = mappings.find(
      (mapping) => mapping.sourceName === currentType
    )!;

    currentRanges.forEach((range) => {
      nextRanges.push(...mapRanges(currentTypeMap, range));
    });

    currentType = currentTypeMap.destinationName;
    currentRanges = mergeRanges(nextRanges);
  }
};

const mapRanges = (mappings: Mapping, range: SeedRange) => {
  const allMappingRangesSet = new Set(
    mappings.content
      .flatMap((aMapping) => {
        return [aMapping.start, aMapping.start + aMapping.range];
      })
      .filter((item) => item > range.start && item < range.start + range.range)
  );

  const ranges = [
    range.start,
    ...allMappingRangesSet,
    range.start + range.range,
  ];

  ranges.sort((a, b) => a - b);

  const seedRanges: SeedRange[] = [];
  for (let i = 0; i < ranges.length - 1; i += 1) {
    const firstRange = ranges[i];
    const secondRange = ranges[i + 1];

    const newRange = { start: firstRange, range: secondRange - firstRange };

    seedRanges.push(newRange);
  }

  const results = seedRanges.map((seedRange) => {
    return mapRange(mappings, seedRange);
  });

  return results;
};

const mapRange = (mappings: Mapping, range: SeedRange) => {
  const result = mapItem(mappings, range.start);
  const newRange = { start: result, range: range.range };

  return newRange;
};

const mapItems = (mappings: Mapping, items: number[]) => {
  return items.map((item) => {
    return mapItem(mappings, item);
  });
};

const mapItem = (mappings: Mapping, item: number) => {
  const correctMapping = mappings.content.find((aMapping) => {
    const minRange = aMapping.start;
    const maxRange = aMapping.start + aMapping.range;

    return item < maxRange && item >= minRange;
  });

  if (!correctMapping) {
    return item;
  }

  const difference = correctMapping.destination - correctMapping.start;
  return item + difference;
};
