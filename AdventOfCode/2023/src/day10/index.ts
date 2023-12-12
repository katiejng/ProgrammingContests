import { Point, addDirection, addPoints, isValidPoint } from "../utils/points";

const getDirection = (direction: Directions) => {
  switch (direction) {
    case Directions.North:
      return North;
    case Directions.South:
      return South;
    case Directions.East:
      return East;
    case Directions.West:
      return West;
    default:
      throw new Error("INVALID");
  }
};
const North = { r: -1, c: 0 };
const South = { r: 1, c: 0 };
const East = { r: 0, c: 1 };
const West = { r: 0, c: -1 };

type Map = {
  symbol: string;
  directions: Directions[];
  newSymbol: string;
  subPoints?: { walls: Directions[]; symbol: string }[];
}[][];
const parseInput = (input: string) => {
  const lines = input.split("\n");

  let startPoint: Point = { r: 0, c: 0 };
  const pipes = lines.map((line, r) => {
    return line.split("").map((item, c) => {
      const directions = pipeMap[item]!;
      const isStart = item === "S";
      if (isStart) {
        startPoint = { r, c };
      }

      const subPoints = pipeQuadrantMap[item];

      return {
        symbol: item,
        directions,
        newSymbol: pipeSymbolMap[item]!,
        subPoints,
      };
    });
  });

  pipes[startPoint.r][startPoint.c] = remapStartPoint({
    graph: pipes,
    startPoint,
  });

  return {
    pipes,
    startPoint,
  };
};

const remapStartPoint = ({
  graph,
  startPoint,
}: {
  graph: Map;
  startPoint: Point;
}) => {
  const pipes = graph;
  const rows = pipes.length;
  const cols = pipes[0].length;
  const bounds = { r: rows, c: cols };

  const north = addPoints(startPoint, North);
  const south = addPoints(startPoint, South);
  const east = addPoints(startPoint, East);
  const west = addPoints(startPoint, West);

  const isNorth = isValidPoint(north, bounds)
    ? pipes[north.r]![north.c]!.directions.includes(Directions.South)
    : false;
  const isSouth = isValidPoint(south, bounds)
    ? pipes[south.r]![south.c]!.directions.includes(Directions.North)
    : false;
  const isEast = isValidPoint(east, bounds)
    ? pipes[east.r]![east.c]!.directions.includes(Directions.West)
    : false;
  const isWest = isValidPoint(west, bounds)
    ? pipes[west.r]![west.c]!.directions.includes(Directions.East)
    : false;

  const wall = mapDirectionsToWall({ isNorth, isEast, isSouth, isWest });
  const symbol = mapDirectionsToSymbol({
    isNorth,
    isEast,
    isSouth,
    isWest,
  });
  const subPoints = pipeQuadrantMap[symbol];

  return {
    ...pipes[startPoint.r][startPoint.c],
    directions: wall.walls,
    subPoints,
    symbol,
  };
};

const printMap = (pipes: Map, startPoint: Point) => {
  console.log(
    pipes.map((pipe) => pipe.map((item) => item.newSymbol).join("")).join("\n")
  );
  console.log({ startPoint: pipes[startPoint.r][startPoint.c] });
};

const printSubMap = (pipes: Map) => {
  console.log(
    pipes
      .map((pipe) => {
        const firstLine = pipe.flatMap((item) => {
          const subPoints = item.subPoints;
          if (!subPoints) {
            return "╔══╗";
          }

          return subPoints
            .slice(0, 2)
            .map((item) => item.symbol)
            .join("");
        });
        const secondLine = pipe.flatMap((item) => {
          const subPoints = item.subPoints;
          if (!subPoints) {
            return "╚══╝";
          }

          return subPoints
            .slice(2, 4)
            .map((item) => item.symbol)
            .join("");
        });
        return [firstLine.join(""), secondLine.join("")].join("\n");
      })
      .join("\n")
  );
};

const getBfsDistance = (pipes: Map, startPoint: Point) => {
  const bfsDistance: number[][] = [];
  for (let r = 0; r < pipes.length; r++) {
    bfsDistance.push(new Array(pipes[r].length).fill(-1));
  }

  bfsDistance[startPoint.r][startPoint.c] = 0;
  let currentPositions = [startPoint];

  while (currentPositions.length > 0) {
    let nextPositions: Point[] = [];

    currentPositions.forEach((position) => {
      const pipeAtPosition = pipes[position.r][position.c];
      const directions = pipeAtPosition.directions;
      directions.forEach((direction) => {
        const nextPosition = addPoints(position, getDirection(direction));
        if (bfsDistance[nextPosition.r][nextPosition.c] === -1) {
          bfsDistance[nextPosition.r][nextPosition.c] =
            bfsDistance[position.r][position.c] + 1;
          nextPositions.push(nextPosition);
        }
      });
    });

    currentPositions = nextPositions;
  }

  return bfsDistance;
};
export const day10part1 = (input: string) => {
  const { startPoint, pipes } = parseInput(input);
  // printMap(pipes, startPoint);

  const distances = getBfsDistance(pipes, startPoint);

  return Math.max(...distances.flatMap((row) => row));
};

export const day10part2 = (input: string) => {
  const { startPoint, pipes } = parseInput(input);

  const bfsDistance = getBfsDistance(pipes, startPoint);
  const cleanedPipes = pipes.map((row, r) =>
    row.map((item, c) => {
      if (bfsDistance[r][c] === -1) {
        return {
          newSymbol: ".",
          directions: [],
          symbol: ".",
        };
      }
      return item;
    })
  );
  // printSubMap(newPipes);

  const bfsDistanceSubmap = getBfsDistanceSubmap(cleanedPipes);

  let total = 0;
  cleanedPipes.map((row, r) => {
    row.map((item, c) => {
      if (item.directions.length === 0) {
        // check if bfsDistanceMap is all -1
        const mappedValue = bfsDistanceSubmap[r * 2][c * 2];
        if (mappedValue === -1) {
          total++;
        }
      }
    });
  });

  return total;
};

const getBfsDistanceSubmap = (pipes: Map) => {
  const bfsDistance: number[][] = [];
  const bounds = {
    r: pipes.length * 2,
    c: pipes[0].length * 2,
  };

  let currentPositions = []; // all edge pieces
  for (let r = 0; r < bounds.r; r++) {
    for (let c = 0; c < bounds.c; c++) {
      if (r == 0 || c == 0 || r == bounds.r - 1 || c == bounds.c - 1) {
        currentPositions.push({ r, c });
      }
    }
  }

  for (let r = 0; r < bounds.r; r++) {
    bfsDistance.push(new Array(bounds.c).fill(-1));
  }

  while (currentPositions.length > 0) {
    let nextPositions: Point[] = [];

    currentPositions.forEach((position) => {
      const subPipeAtPosition = getSubpipePosition(position, pipes);
      const directionsToGo = [
        Directions.North,
        Directions.East,
        Directions.South,
        Directions.West,
      ];

      const directions = directionsToGo.filter(
        (direction) => !subPipeAtPosition.walls.includes(direction)
      );

      directions.forEach((direction) => {
        const nextPosition = addPoints(position, getDirection(direction));
        if (!isValidPoint(nextPosition, bounds)) {
          return;
        }

        if (bfsDistance[nextPosition.r][nextPosition.c] === -1) {
          bfsDistance[nextPosition.r][nextPosition.c] = 0;
          nextPositions.push(nextPosition);
        }
      });
    });

    currentPositions = nextPositions;
  }

  return bfsDistance;
};

enum Directions {
  North = "N",
  East = "E",
  South = "S",
  West = "W",
  Start = "Start",
}
const pipeMap: Record<string, Directions[]> = {
  "|": [Directions.North, Directions.South],
  "-": [Directions.East, Directions.West],
  L: [Directions.North, Directions.East],
  J: [Directions.North, Directions.West],
  "7": [Directions.South, Directions.West],
  F: [Directions.South, Directions.East],
  ".": [],
  S: [Directions.Start],
};

// top left, top right, bottom left, bottom right
const wallWest = {
  symbol: "| ",
  walls: [Directions.West],
};

const wallNorth = {
  symbol: "^^",
  walls: [Directions.North],
};

const wallEast = {
  symbol: " |",
  walls: [Directions.East],
};
const wallSouth = {
  symbol: "__",
  walls: [Directions.South],
};

const wallNorthEast = {
  symbol: "^|",
  walls: [Directions.North, Directions.East],
};
const wallSouthEast = {
  symbol: "_|",
  walls: [Directions.South, Directions.East],
};

const wallSoutWest = {
  symbol: "|_",
  walls: [Directions.South, Directions.West],
};
const wallNorthWest = {
  symbol: "|^",
  walls: [Directions.North, Directions.West],
};

const wallNone = {
  symbol: "  ",
  walls: [],
};

const pipeQuadrantMap: Record<
  string,
  { walls: Directions[]; symbol: string }[]
> = {
  "|": [wallEast, wallWest, wallEast, wallWest],
  "-": [wallSouth, wallSouth, wallNorth, wallNorth],
  L: [wallEast, wallSoutWest, wallNone, wallNorth],
  "7": [wallSouth, wallNone, wallNorthEast, wallWest],
  F: [wallNone, wallSouth, wallEast, wallNorthWest],
  J: [wallSouthEast, wallWest, wallNorth, wallNone],
};

const pipeSymbolMap: Record<string, string> = {
  "|": "║",
  "-": "═",
  L: "╚",
  J: "╝",
  "7": "╗",
  F: "╔",
  ".": ".",
  S: "S",
};

const canMoveToPoint = (start: Point, direction: Point, pipes: Map) => {
  // can move IF current spot is a pipe that points to the direction, and the destination is a pipe that points to that direction.

  return true;
};

const mapDirectionsToSymbol = (input: {
  isNorth: boolean;
  isEast: boolean;
  isSouth: boolean;
  isWest: boolean;
}) => {
  if (input.isNorth) {
    if (input.isEast) {
      return "L";
    }
    if (input.isWest) {
      return "J";
    }
    if (input.isSouth) {
      return "|";
    }
  }
  if (input.isSouth) {
    if (input.isEast) {
      return "F";
    }
    if (input.isWest) {
      return "7";
    }
  }
  return "-";
};

const mapDirectionsToWall = (input: {
  isNorth: boolean;
  isEast: boolean;
  isSouth: boolean;
  isWest: boolean;
}) => {
  if (input.isNorth) {
    if (input.isEast) {
      return wallNorthEast;
    }
    if (input.isWest) {
      return wallNorthWest;
    }
    return wallNorth;
  }
  if (input.isSouth) {
    if (input.isEast) {
      return wallSouthEast;
    }
    if (input.isWest) {
      return wallSoutWest;
    }
    return wallSouth;
  }
  return wallNone;
};

export const getSubpointPoint = (subPoint: Point) => {
  const parentPoint = {
    r: Math.floor(subPoint.r / 2),
    c: Math.floor(subPoint.c / 2),
  };
  const subPointOffset = (subPoint.r % 2) * 2 + (subPoint.c % 2);

  return {
    parentPoint,
    subPointOffset,
  };
};
export const getSubpipePosition = (subPoint: Point, pipes: Map) => {
  const { parentPoint, subPointOffset } = getSubpointPoint(subPoint);

  const parentItem = pipes[parentPoint.r][parentPoint.c];
  if (parentItem.subPoints) {
    return parentItem.subPoints[subPointOffset];
  }
  return {
    walls: [],
    symbol: ".",
  };
};
