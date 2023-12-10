export const day1part1 = (input: string) => {
  const lines = input.split("\n").map((line) => {
    const parsedLine = line
      .trim()
      .split("")
      .map((char) => (isNumber(char) ? char : Number(char)));

    const numbers = parsedLine.filter((char) => isNumber(char));

    const first = numbers[0];
    const last = numbers[numbers.length - 1];
    return Number(first.toString() + last.toString());
  });
  return lines.reduce((a, b) => a + b, 0);
};

const isNumber = (maybeNumber: string | number) => !isNaN(Number(maybeNumber));

export const day1part2 = (input: string) => {
  const lines = input.split("\n").map((line) => {
    const parsedLine = line
      .trim()
      .split("")
      .map((char) => (isNumber(char) ? Number(char) : char));

    const allNumbers: number[] = [];
    let currentNumberAsString = [];
    let firstNumber: number;
    // parse right
    for (let i = 0; i < parsedLine.length; i++) {
      if (isNumber(parsedLine[i])) {
        firstNumber = parsedLine[i] as number;
        break;
      }
      currentNumberAsString.push(parsedLine[i]);

      const mappedNumber = containsNumber(currentNumberAsString.join(""));
      if (mappedNumber) {
        firstNumber = mappedNumber;
        currentNumberAsString = [];
        break;
      }
    }

    let lastNumber: number;
    // parse left
    for (let i = parsedLine.length - 1; i >= 0; i--) {
      if (isNumber(parsedLine[i])) {
        lastNumber = parsedLine[i] as number;
        break;
      }

      currentNumberAsString = [parsedLine[i], ...currentNumberAsString];
      const mappedNumber = containsNumber(currentNumberAsString.join(""));
      if (mappedNumber) {
        lastNumber = mappedNumber as number;
        currentNumberAsString = [];
        break;
      }
    }

    // console.log({ line, allNumbers, firstNumber, lastNumber });
    return firstNumber! * 10 + lastNumber!;
  });

  return lines.reduce((a, b) => a + b, 0);
};

const numbersWordMap: Record<string, number> = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9,
};

const containsNumber = (maybeNumber: string) => {
  const numbers = Object.keys(numbersWordMap);

  const matchedNumber = numbers.find((number) => maybeNumber.includes(number));
  if (matchedNumber) {
    return numbersWordMap[matchedNumber];
  }
  return undefined;
};
