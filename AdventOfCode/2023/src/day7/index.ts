const parseInput = (input: string) => {
  return input.split("\n").map((line) => {
    const [hand, bid] = line.split(" ");
    return {
      hand: hand.trim(),
      bid: Number(bid.trim()),
    };
  });
};

const cardValuesP1: Record<string, number> = {
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
  6: 6,
  7: 7,
  8: 8,
  9: 9,

  T: 10,
  J: 11,
  Q: 12,
  K: 13,
  A: 14,
};

const compareHandsP1 = <T extends { hand: string }>(hand1: T, hand2: T) => {
  const hand1Score = getHandP1(hand1.hand);
  const hand2Score = getHandP1(hand2.hand);

  const diff = hand2Score - hand1Score; // sort descending
  if (diff !== 0) {
    return diff;
  }

  for (let i = 0; i < 5; i++) {
    const hand1V = cardValuesP1[hand1.hand[i]];
    const hand2V = cardValuesP1[hand2.hand[i]];
    const diff = hand1V - hand2V; // sort ascending
    if (diff !== 0) {
      return diff;
    }
  }
  return 0;
};

const getHandP1 = (hand: string): Hands => {
  const map: Record<string, number> = {};
  hand.split("").forEach((char) => {
    map[char] = map[char] ? map[char] + 1 : 1;
  });

  const values = Object.values(map);

  if (values.includes(5)) {
    return Hands.Five;
  }
  if (values.includes(4)) {
    return Hands.Four;
  }
  if (values.includes(3) && values.includes(2)) {
    return Hands.FullHouse;
  }
  if (values.includes(3)) {
    return Hands.Three;
  }
  const pairs = values.filter((value) => value === 2);
  if (pairs.length == 2) {
    return Hands.TwoPair;
  }
  if (pairs.length == 1) {
    return Hands.OnePair;
  }
  return Hands.HighCard;
};

enum Hands {
  Five = 1,
  Four = 2,
  FullHouse = 3,
  Three = 4,
  TwoPair = 5,
  OnePair = 6,
  HighCard = 7,
}

export const day7part1 = (input: string) => {
  const parsedInput = parseInput(input);

  const sortedResults = parsedInput.sort(compareHandsP1);

  return sortedResults
    .map((item, index) => item.bid * (index + 1))
    .reduce((acc, curr) => acc + curr, 0);
};

export const day7part2 = (input: string) => {
  const parsedInput = parseInput(input);
  const sortedResults = parsedInput.sort(compareHandsP2);

  return sortedResults
    .map((item, index) => item.bid * (index + 1))
    .reduce((acc, curr) => acc + curr, 0);
};

const cardValuesP2: Record<string, number> = {
  1: 1,
  2: 2,
  3: 3,
  4: 4,
  5: 5,
  6: 6,
  7: 7,
  8: 8,
  9: 9,

  T: 10,
  J: 0,
  Q: 12,
  K: 13,
  A: 14,
};

const compareHandsP2 = <T extends { hand: string }>(hand1: T, hand2: T) => {
  const hand1Score = getHandP2(hand1.hand);
  const hand2Score = getHandP2(hand2.hand);

  const diff = hand2Score - hand1Score; // sort descending
  if (diff !== 0) {
    return diff;
  }

  for (let i = 0; i < 5; i++) {
    const hand1V = cardValuesP2[hand1.hand[i]];
    const hand2V = cardValuesP2[hand2.hand[i]];
    const diff = hand1V - hand2V; // sort ascending
    if (diff !== 0) {
      return diff;
    }
  }
  return 0;
};

const getHandP2 = (hand: string): Hands => {
  const map: Record<string, number> = {};
  hand.split("").forEach((char) => {
    map[char] = map[char] ? map[char] + 1 : 1;
  });

  let NumberOfJs = map["J"];

  if (NumberOfJs === 5) {
    return Hands.Five;
  }
  delete map["J"];

  const values = Object.values(map);
  values.sort((a, b) => b - a); // sort desc?

  let i = 0;
  while (NumberOfJs > 0) {
    if (values[i] < 5) {
      values[i]++;
      NumberOfJs--;
    } else {
      i++;
    }
  }

  if (values.includes(5)) {
    return Hands.Five;
  }
  if (values.includes(4)) {
    return Hands.Four;
  }
  if (values.includes(3) && values.includes(2)) {
    return Hands.FullHouse;
  }
  if (values.includes(3)) {
    return Hands.Three;
  }
  const pairs = values.filter((value) => value === 2);
  if (pairs.length == 2) {
    return Hands.TwoPair;
  }
  if (pairs.length == 1) {
    return Hands.OnePair;
  }
  return Hands.HighCard;
};
