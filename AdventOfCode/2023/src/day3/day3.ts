import { isNumber } from "../utils/isNumber";

const isSymbol = (char: string) => {
  if (isNumber(char)) {
    return false;
  }
  if (char === ".") {
    return false;
  }
  return true;
};
type Point = { r: number; c: number };

const parseInputPart1 = (input: string) => {
  const lines = input.split("\n");
  const bounds = {
    r: lines.length,
    c: lines[0].length,
  };

  const numbersResult: { number: number; matchingSymbols: any }[] = [];
  lines.forEach((line, r) => {
    let c = 0;

    while (c < line.length) {
      if (isNumber(line[c])) {
        let numberEndIndex = c + 1;
        while (true) {
          if (!isNumber(line[numberEndIndex])) {
            numberEndIndex--;
            break;
          } else {
            numberEndIndex++;
          }
        }

        // read number
        const numberString = line.slice(c, numberEndIndex + 1);

        // read symbols
        const matchingSymbols = [];
        for (let i = c; i <= numberEndIndex; i++) {
          const adjacentPoints = getAdjacentPoints({ r, c: i }, bounds);
          matchingSymbols.push(
            ...adjacentPoints.filter((point) =>
              isSymbol(lines[point.r][point.c])
            )
          );
        }

        numbersResult.push({ number: Number(numberString), matchingSymbols });
        c = numberEndIndex + 1;
      }
      c++;
    }
  });
  return numbersResult;
};

export const day3part1 = (input: string) => {
  const parsedInput = parseInputPart1(input);
  return parsedInput
    .filter((item) => item.matchingSymbols.length > 0)
    .map((item) => item.number)
    .reduce((a, b) => a + b, 0);
};

type SurroundingNumber = { number: number; startPoint: Point };

const parseInputPart2 = (input: string) => {
  const lines = input.split("\n");

  const bounds = {
    r: lines.length,
    c: lines[0].length,
  };

  const starResults: {
    surroundingNumbers: SurroundingNumber[];
  }[] = [];

  lines.forEach((line, r) => {
    line.split("").map((char, c) => {
      if (char === "*") {
        const adjacentPoints = getAdjacentPoints({ r, c }, bounds);
        const surroundingNumbers = adjacentPoints
          .map((point) => getNumberAtPoint(point, lines))
          .filter(
            (number): number is SurroundingNumber => number !== undefined
          );

        const results: SurroundingNumber[] = [];

        surroundingNumbers.forEach((surroundingNumber) => {
          if (
            results.find(
              (result) =>
                result.startPoint.r === surroundingNumber?.startPoint.r &&
                result.startPoint.c === surroundingNumber?.startPoint.c
            )
          ) {
            return;
          } else {
            results.push(surroundingNumber);
          }
        });
        starResults.push({ surroundingNumbers: results });
      }
    });
  });
  return starResults;
};

export const day3part2 = (input: string) => {
  const parsedResult = parseInputPart2(input);
  // console.log({ parsedResult });

  return parsedResult
    .filter((result) => result.surroundingNumbers.length === 2)
    .map((result) =>
      result.surroundingNumbers
        .map((num) => num.number)
        .reduce((a, b) => a * b, 1)
    )
    .reduce((a, b) => a + b, 0);
};

const getAdjacentPoints = (point: Point, bounds: Point) => {
  const vectors: Point[] = [-1, 0, 1].flatMap((r) =>
    [-1, 0, 1].map((c) => ({ r, c }))
  );

  return vectors
    .map((vector): Point | undefined => {
      if (vector.r === 0 && vector.c === 0) {
        return undefined;
      }

      const newPoint: Point = {
        r: point.r + vector.r,
        c: point.c + vector.c,
      };

      if (newPoint.r < 0 || newPoint.c < 0) {
        return undefined;
      }
      if (newPoint.r >= bounds.r || newPoint.c >= bounds.c) {
        return undefined;
      }
      return newPoint;
    })
    .filter((point): point is Point => point !== undefined);
};

const getNumberAtPoint = (point: Point, lines: string[]) => {
  const chars = lines[point.r].split("");
  if (isNumber(chars[point.c])) {
    let currentIndex = point.c;

    let startIndex = 0;
    let endIndex = 0;
    // found a number

    // go left
    while (true) {
      currentIndex--;
      if (!isNumber(chars[currentIndex])) {
        startIndex = currentIndex + 1;
        break;
      }
    }

    // go right
    currentIndex = point.c;
    while (true) {
      currentIndex++;
      if (!isNumber(chars[currentIndex])) {
        endIndex = currentIndex - 1;
        break;
      }
    }

    // read number

    const numberString = chars.slice(startIndex, endIndex + 1).join("");
    return {
      number: Number(numberString),
      startPoint: { r: point.r, c: startIndex },
    };
  }
  return undefined;
};
