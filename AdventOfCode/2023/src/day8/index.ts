import { lcm } from "../utils/math";

enum Direction {
  Left,
  Right,
}
const parseInput = (input: string) => {
  const [pathString, graphString] = input.split("\n\n");
  const path = pathString
    .trim()
    .split("")
    .map((item) => (item === "L" ? Direction.Left : Direction.Right));

  const graphArray = graphString.split("\n").map((line) => {
    const [source, LR] = line.trim().split(" = ");

    const [left, right] = LR.slice(1, LR.length - 1).split(", ");

    return {
      [source]: {
        left,
        right,
      },
    };
  });

  const graph = graphArray.reduce((acc, curr) => ({ ...acc, ...curr }), {});
  return {
    path,
    graph,
  };
};

export const day8part1 = (input: string) => {
  const parsedInput = parseInput(input);
  let currPos = "AAA";
  let nextPaths = parsedInput.graph[currPos];
  const traveledPath = [];
  let pathIndex = 0;
  const path = parsedInput.path;

  while (currPos !== "ZZZ") {
    const currentDirection = path[pathIndex];

    if (currentDirection === Direction.Left) {
      currPos = nextPaths.left;
    } else {
      currPos = nextPaths.right;
    }

    pathIndex = (pathIndex + 1) % path.length;
    traveledPath.push(currPos);
    nextPaths = parsedInput.graph[currPos];
  }
  return traveledPath.length;
};

const endsInXFilter = (X: string, places: string[]) =>
  places.filter((place) => place[2] === X);

const endsInZ = (place: string) => place[2] === "Z";

export const day8part2 = (input: string) => {
  const { graph, path } = parseInput(input);

  let startPoints = endsInXFilter("A", Object.keys(graph));

  const loopPoints = startPoints.map((startPoint) => {
    let currPoint = startPoint;
    let steps = 0;
    let pathIndex = 0;
    while (true) {
      const nextDirection = path[pathIndex];
      const nextPoint =
        graph[currPoint][nextDirection === Direction.Left ? "left" : "right"];

      currPoint = nextPoint;
      steps += 1;

      if (endsInZ(currPoint)) {
        return steps;
      }

      pathIndex = (pathIndex + 1) % path.length;
    }
  });

  return lcm(loopPoints);
};
