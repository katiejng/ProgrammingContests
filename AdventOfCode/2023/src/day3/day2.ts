export const day3part1 = (input: string) => {
  const parsedInput = parseGame(input);

  const possibleGames = parsedInput.filter((game) => {
    const isImpossible = game.games.some((shownItems) => {
      return shownItems.some((item) => {
        const maxItems = part1Rule[item.color];
        if (maxItems) {
          return item.number > maxItems;
        }
        return true;
      });
    });
    return !isImpossible;
  });

  return possibleGames.map((game) => game.gameId).reduce((a, b) => a + b, 0);
};

const part1Rule: Record<string, number> = {
  red: 12,
  green: 13,
  blue: 14,
};

const parseGame = (input: string) => {
  return input.split("\n").map((line) => {
    const [gameIdString, games] = line.trim().split(":");
    const [_, gameNumberString] = gameIdString.trim().split(" ");
    const gameId = Number(gameNumberString);

    const parsedgames = games
      .trim()
      .split(";")
      .map((game) => {
        const shownItems = game
          .trim()
          .split(", ")
          .map((item) => {
            const [number, color] = item.split(" ");
            const parsedNumber = Number(number);
            return { number: parsedNumber, color };
          });
        return shownItems;
      });
    return { gameId, games: parsedgames };
  });
};

export const day3part2 = (input: string) => {
  const parsedInput = parseGame(input);

  const powers = parsedInput.map((game) => {
    let maxRed = 0;
    let maxBlue = 0;
    let maxGreen = 0;
    game.games.forEach((shownItems) => {
      shownItems.forEach((item) => {
        switch (item.color) {
          case "red":
            maxRed = Math.max(maxRed, item.number);
            break;
          case "blue":
            maxBlue = Math.max(maxBlue, item.number);
            break;
          case "green":
            maxGreen = Math.max(maxGreen, item.number);
            break;
          default:
            break;
        }
      });
    });

    const power = maxRed * maxBlue * maxGreen;
    return power;
  });
  return powers.reduce((a, b) => a + b, 0);
};
