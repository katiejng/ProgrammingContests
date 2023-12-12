import { lastItemInList } from "../utils/arrays";

const parseInput = (input: string) => {
  const sequences = input
    .split("\n")
    .map((line) => line.split(" ").map((item) => Number(item.trim())));
  return sequences;
};

const formSequenceTriangle = (sequence: number[]) => {
  const subSequences = [sequence];

  while (true) {
    const lastSequence = subSequences[subSequences.length - 1];
    const nextSequence = [];
    for (let i = 0; i < lastSequence.length - 1; i++) {
      const diff = lastSequence[i + 1] - lastSequence[i];
      nextSequence.push(diff);
    }

    subSequences.push(nextSequence);

    if (nextSequence.filter((item) => item !== 0).length === 0) {
      return subSequences;
    }
  }
};
export const day9part1 = (input: string) => {
  const parsedInput = parseInput(input);

  const mappedResults = parsedInput.map((sequence) => {
    return formSequenceTriangle(sequence);
  });

  mappedResults.forEach((sequenceTriangle) => {
    for (let i = sequenceTriangle.length - 1; i >= 0; i--) {
      if (i === sequenceTriangle.length - 1) {
        sequenceTriangle[i].push(0);
      } else {
        const nextItem =
          lastItemInList(sequenceTriangle[i + 1]) +
          lastItemInList(sequenceTriangle[i]);
        sequenceTriangle[i].push(nextItem);
      }
    }
  });

  return mappedResults
    .map((triangle) => lastItemInList(triangle[0]))
    .reduce((a, b) => a + b, 0);
};

export const day9part2 = (input: string) => {
  const parsedInput = parseInput(input);

  const mappedResults = parsedInput.map((sequence) => {
    return formSequenceTriangle(sequence);
  });

  mappedResults.forEach((sequenceTriangle) => {
    for (let i = sequenceTriangle.length - 1; i >= 0; i--) {
      if (i === sequenceTriangle.length - 1) {
        sequenceTriangle[i].push(0);
      } else {
        const nextItem =
          firstItemInList(sequenceTriangle[i]) -
          firstItemInList(sequenceTriangle[i + 1]);
        sequenceTriangle[i] = [nextItem, ...sequenceTriangle[i]];
      }
    }
  });

  return mappedResults
    .map((triangle) => triangle[0][0])
    .reduce((a, b) => a + b, 0);
};

const firstItemInList = (sequence: number[]) => sequence[0];
