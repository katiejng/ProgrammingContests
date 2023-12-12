const parseInput = (input: string) => {
  const results = input.split("\n").map((line) => {
    const [cardNum, numbers] = line.split(":");
    const cardId = cardNum.split(" ")[1];
    const [winningNums, yourNums] = numbers.split("|");
    const winningNumbers = winningNums
      .split(" ")
      .filter((number) => number.length > 0)
      .map(Number);
    const yourNumbers = yourNums
      .split(" ")
      .filter((number) => number.length > 0)
      .map(Number);

    return {
      cardId,
      yourNumbers,
      winningNumbers,
    };
  });

  return results;
};

export const day4part1 = (input: string) => {
  const parsedInput = parseInput(input);
  return parsedInput
    .map((game) => {
      const yourSet = new Set(game.yourNumbers);
      const intersection = game.winningNumbers.filter((x) => yourSet.has(x));
      return intersection;
    })
    .filter((winningNumbers) => winningNumbers.length > 0)
    .map((winningNumbers) => 2 ** (winningNumbers.length - 1))
    .reduce((a, b) => a + b, 0);
};

export const day4part2 = (input: string) => {
  const parsedInput = parseInput(input);

  const numberOfCards = new Array(parsedInput.length).fill(1);

  parsedInput.map((game, index) => {
    const numberOfCardsCurrent = numberOfCards[index];
    const yourSet = new Set(game.yourNumbers);
    const intersection = game.winningNumbers.filter((x) => yourSet.has(x));

    const numberOfCardsWon = intersection.length;
    for (let i = 0; i < numberOfCardsWon; i++) {
      numberOfCards[index + i + 1] += numberOfCardsCurrent;
    }
  });

  return numberOfCards.reduce((a, b) => a + b, 0);
};
