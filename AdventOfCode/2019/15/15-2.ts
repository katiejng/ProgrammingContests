import input from "./in15-2";
import IntCodeComputer from "../11/IntCode";

function calculatePositionInfo(computer: IntCodeComputer, direction) {
  computer.inputs.push(direction);
  computer.processList();
  return computer.outputs.shift();
}

function getNextPos(x, y, dir) {
  switch (dir) {
    case 1:
      return { x, y: y - 1 };
      break;
    case 2:
      return { x, y: y + 1 };
      break;
    case 3:
      return { x: x - 1, y };
      break;
    case 4:
      return { x: x + 1, y };
      break;
  }
  return { x, y };
}

function getDirection(lastStep: number) {
  switch (lastStep) {
    case 1:
      return [1, 3, 4];
    case 2:
      return [2, 3, 4];
    case 3:
      return [1, 2, 3];
    case 4:
      return [1, 2, 4];
  }
}

function getNextStep(
  lastStep: any,
  computer: IntCodeComputer,
  x: number,
  y: number
) {
  let directions = getDirection(lastStep);

  return directions.map(dir => {
    let nextPos = getNextPos(x, y, dir);
    return { direction: dir, computer: computer, position: nextPos };
  });
}

function loop(map: Hull, directions: Array<any>) {
  let count = 0;
  let nextSteps = directions;

  while (true) {
    //loop each step
    if (nextSteps.length === 0) {
      break;
    }
    let directions = [...nextSteps];

    nextSteps = [];

    for (let dirIndex = 0; dirIndex < directions.length; dirIndex++) {
      // loop each direction

      let copy = directions[dirIndex].computer.getCopy();
      let newComp = new IntCodeComputer("", [], []);
      newComp.setCopy(copy);

      let position = directions[dirIndex].position;
      let curDir = directions[dirIndex].direction;
      let result = calculatePositionInfo(newComp, curDir);
      map.setXY(position.x, position.y, result);

      switch (result) {
        case 0: //wall
          break;
        case 1: // path
          let nextDirs = getNextStep(curDir, newComp, position.x, position.y);
          // calulate if we've already been there
          nextDirs.filter(direction => {
            let nextPos = getNextPos(
              position.x,
              position.y,
              direction.direction
            );
            return !map.hasVisitedXY(nextPos.x, nextPos.y);
          });
          nextSteps.push(...nextDirs);
          break;
        case 2: // goal
          console.log("foudn it at", count);
      }
    }
    count++;
    map.print();
  }

  return map;
}

function loop2(map: Hull, directions: Array<any>) {
  let count = 0;
  let nextSteps = directions;

  while (true) {
    //loop each step
    if (nextSteps.length === 0) {
      break;
    }
    let directions = [...nextSteps];

    nextSteps = [];

    for (let dirIndex = 0; dirIndex < directions.length; dirIndex++) {
      // loop each direction

      let copy = directions[dirIndex].computer.getCopy();
      let newComp = new IntCodeComputer("", [], []);
      newComp.setCopy(copy);

      let position = directions[dirIndex].position;
      let curDir = directions[dirIndex].direction;
      let result = map.getXY(position.x, position.y);
      map.setXY(position.x, position.y, 9);

      switch (result) {
        case 9:
          break; //visited
        case 0: //wall
          break;
        case 1: // path
          let nextDirs = getNextStep(curDir, newComp, position.x, position.y);
          // calulate if we've already been there
          nextDirs.filter(direction => {
            let nextPos = getNextPos(
              position.x,
              position.y,
              direction.direction
            );
            return !map.hasVisitedXY(nextPos.x, nextPos.y);
          });
          nextSteps.push(...nextDirs);
          break;
        case 2: // goal
          console.log("foudn it at", count);
      }
    }
    count++;
    map.print();
  }

  return count;
}

class Hull {
  map: object;

  //index will be x-y

  constructor() {
    this.map = {};
  }

  hasVisitedXY(x: number, y: number) {
    return this.getStringKey(x, y) in this.map;
  }
  set(index, value) {
    this.map[index] = value;
  }

  setXY(x: number, y: number, value) {
    this.map[this.getStringKey(x, y)] = value;
  }

  getStringKey(x: number, y: number) {
    return `${x}|${y}`;
  }

  get(index) {
    const item = this.map[index];
    return item ? item : 0; // 0 BLACK
  }

  getXY(x: number, y: number) {
    const item = this.map[this.getStringKey(x, y)];
    return item ? item : 0; // 0 BLACK
  }

  print() {
    const keys = Object.keys(this.map).map(key => [
      ...key.split("|").map(x => parseInt(x)),
      this.map[key]
    ]);
    let xMin = Math.min(...keys.map(key => key[0]));
    let xMax = Math.max(...keys.map(key => key[0])) + 1;
    let yMin = Math.min(...keys.map(key => key[1]));
    let yMax = Math.max(...keys.map(key => key[1])) + 1;

    let grid = new Array(yMax - yMax);
    for (let y0 = yMin; y0 < yMax; y0++) {
      grid[y0 - yMin] = new Array(xMax - xMin).fill("_");
    }
    for (let i = 0; i < keys.length; i++) {
      let item = keys[i];
      grid[item[1] - yMin][item[0] - xMin] = item[2];
    }
    console.log("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    console.log(
      grid
        .map(x => {
          return x
            .map(spot => {
              switch (spot) {
                case 0:
                  return "_";
                case 1:
                  return "▫️";
                case 2:
                  return "❇︎";
                case 9:
                  return "✶";
                default:
                  return "_";
              }
            })
            .join("");
        })
        .join("\n")
    ),
      "\n";
  }
}

function main() {
  const visited = new Hull();
  const position = { x: 0, y: 0 };
  visited.setXY(position.x, position.y, 1);
  const codes = input.value.split(",").map(x => parseInt(x));
  const computerA = new IntCodeComputer("A", codes, []);
  const computerB = new IntCodeComputer("B", codes, []);
  const computerC = new IntCodeComputer("C", codes, []);
  const computerD = new IntCodeComputer("D", codes, []);
  let directions = [
    { direction: 1, computer: computerA, position: { x: 0, y: -1 } }, //N
    { direction: 2, computer: computerB, position: { x: 0, y: 1 } }, //S
    { direction: 3, computer: computerC, position: { x: -1, y: 0 } }, //W
    { direction: 4, computer: computerD, position: { x: 1, y: 0 } } //E
  ];

  let result = loop(visited, directions);

  return result;
}

function main2(map: Hull) {
  //  let keys = Object.keys(map.map);
  let gasPos = { x: 16, y: 12 }; //keys.filter(key => map.map[key] == 2)[0].spl;
  console.log(gasPos);
  const codes = input.value.split(",").map(x => parseInt(x));
  const computerA = new IntCodeComputer("A", codes, []);
  const computerB = new IntCodeComputer("B", codes, []);
  const computerC = new IntCodeComputer("C", codes, []);
  const computerD = new IntCodeComputer("D", codes, []);

  map.setXY(16, 12, 9);
  let directions = [
    { direction: 1, computer: computerA, position: { x: 16, y: 11 } }, //N
    { direction: 2, computer: computerB, position: { x: 16, y: 13 } }, //S
    { direction: 3, computer: computerC, position: { x: 15, y: 12 } }, //W
    { direction: 4, computer: computerD, position: { x: 17, y: 12 } } //E
  ];

  let result = loop2(map, directions);
  return result;
}

const result = main();
const result2 = main2(result);
console.log(result2);
export { main, calculatePositionInfo, getNextStep };
