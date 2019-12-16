import input from "./in11-2";
import IntCodeComputer from "../11/IntCode";
// indices are map[y][x] !!!
class Hull {
  map: object;

  //index will be x-y

  constructor() {
    this.map = {};
  }

  set(index, value) {
    this.map[index] = value;
  }

  setXY(x: number, y: number, value) {
    this.map[this.getStringKey(x, y)] = value;
  }

  getStringKey(x: number, y: number) {
    return `${x}-${y}`;
  }

  get(index) {
    const item = this.map[index];
    return item ? item : 0; // 0 BLACK
  }

  getXY(x: number, y: number) {
    const item = this.map[this.getStringKey(x, y)];
    return item ? item : 0; // 0 BLACK
  }
}

enum Direction {
  UP,
  RIGHT,
  DOWN,
  LEFT
}
class HullPaintingRobot {
  computer: IntCodeComputer;
  xCurrent: number;
  yCurrent: number;
  direction: Direction;
  hull: Hull;
  constructor(computer: IntCodeComputer, hull: Hull) {
    this.computer = computer;
    this.xCurrent = 0;
    this.yCurrent = 0;
    this.hull = hull;
    this.direction = Direction.UP;
  }
  paint() {
    while (this.computer.isDead == false) {
      let currentBlock = this.hull.getXY(this.xCurrent, this.yCurrent);
      this.computer.inputs.push(currentBlock);
      this.computer.processList();
      let result = this.computer.outputs;
      this.computer.outputs = [];

      this.hull.setXY(this.xCurrent, this.yCurrent, result[0]);

      this.processDirection(result[1]);
      this.moveForward();
    }
    console.log(this.hull.map);
    console.log("RESULT: ", Object.values(this.hull.map).length);
  }

  moveForward() {
    switch (this.direction) {
      case Direction.UP:
        this.yCurrent -= 1;
        break;
      case Direction.DOWN:
        this.yCurrent += 1;
        break;
      case Direction.LEFT:
        this.xCurrent -= 1;
        break;
      case Direction.RIGHT:
        this.xCurrent += 1;
        break;
    }
  }

  processDirection(direction: number) {
    if (direction === 0) {
      this.direction = (this.direction += 3) % 4;
    } else {
      this.direction = (this.direction += 1) % 4;
    }
  }
}
function main() {
  const program = input.value.split(",").map(num => parseInt(num));
  const hull = new Hull();
  hull.setXY(0, 0, 1);
  const computer = new IntCodeComputer("A", program, []);
  const robot = new HullPaintingRobot(computer, hull);
  robot.paint();
  return 0;
}

function graph() {
  const input = [
    [0, 0, 0],
    [1, 0, 1],
    [1, 1, 1],
    [2, 1, 0],
    [2, 0, 0],
    [3, 0, 0],
    [3, 1, 0],
    [4, 1, 1],
    [4, 0, 1],
    [5, 0, 0],
    [5, 1, 0],
    [6, 1, 1],
    [6, 0, 1],
    [7, 0, 1],
    [7, 1, 0],
    [8, 1, 0],
    [8, 0, 1],
    [9, 0, 0],
    [9, 1, 1],
    [10, 1, 0],
    [10, 0, 0],
    [11, 0, 0],
    [11, 1, 1],
    [12, 1, 0],
    [12, 0, 1],
    [13, 0, 1],
    [13, 1, 0],
    [14, 1, 1],
    [14, 0, 0],
    [15, 0, 0],
    [15, 1, 0],
    [16, 1, 1],
    [16, 0, 0],
    [17, 0, 1],
    [17, 1, 0],
    [18, 1, 0],
    [18, 0, 1],
    [19, 0, 0],
    [19, 1, 1],
    [20, 1, 0],
    [20, 0, 0],
    [21, 0, 1],
    [21, 1, 1],
    [22, 1, 0],
    [22, 0, 1],
    [23, 0, 1],
    [23, 1, 0],
    [24, 1, 0],
    [24, 0, 1],
    [25, 0, 0],
    [25, 1, 0],
    [26, 1, 1],
    [26, 0, 1],
    [27, 0, 0],
    [27, 1, 0],
    [28, 1, 0],
    [28, 0, 0],
    [29, 0, 0],
    [29, 1, 0],
    [30, 1, 0],
    [30, 0, 0],
    [31, 0, 0],
    [31, 1, 1],
    [32, 1, 0],
    [32, 0, 1],
    [33, 0, 1],
    [33, 1, 0],
    [34, 1, 1],
    [34, 0, 0],
    [35, 0, 0],
    [35, 1, 0],
    [36, 1, 1],
    [36, 0, 1],
    [37, 0, 1],
    [37, 1, 0],
    [38, 1, 0],
    [38, 0, 1],
    [39, 0, 0],
    [39, 1, 1],
    [40, 1, 0],
    [40, 0, 0],
    [41, 0, 0],
    [41, 1, 0],
    [42, 1, 0],
    [42, 2, 0],
    [41, 2, 0],
    [41, 3, 0],
    [40, 3, 0],
    [40, 2, 0],
    [39, 2, 1],
    [39, 3, 0],
    [38, 3, 1],
    [38, 2, 0],
    [37, 2, 0],
    [37, 3, 1],
    [36, 3, 1],
    [36, 2, 1],
    [35, 2, 0],
    [35, 3, 0],
    [34, 3, 0],
    [34, 2, 0],
    [33, 2, 0],
    [33, 3, 0],
    [32, 3, 0],
    [32, 2, 0],
    [31, 2, 1],
    [31, 3, 1],
    [30, 3, 0],
    [30, 2, 0],
    [29, 2, 0],
    [29, 3, 0],
    [28, 3, 0],
    [28, 2, 0],
    [27, 2, 0],
    [27, 3, 0],
    [26, 3, 1],
    [26, 2, 1],
    [25, 2, 0],
    [25, 3, 0],
    [24, 3, 0],
    [24, 2, 0],
    [23, 2, 1],
    [23, 3, 0],
    [22, 3, 0],
    [22, 2, 1],
    [21, 2, 1],
    [21, 3, 1],
    [20, 3, 0],
    [20, 2, 0],
    [19, 2, 1],
    [19, 3, 1],
    [18, 3, 1],
    [18, 2, 0],
    [17, 2, 0],
    [17, 3, 1],
    [16, 3, 1],
    [16, 2, 1],
    [15, 2, 0],
    [15, 3, 0],
    [14, 3, 0],
    [14, 2, 0],
    [13, 2, 0],
    [13, 3, 0],
    [12, 3, 0],
    [12, 2, 0],
    [11, 2, 1],
    [11, 3, 1],
    [10, 3, 0],
    [10, 2, 0],
    [9, 2, 1],
    [9, 3, 0],
    [8, 3, 1],
    [8, 2, 0],
    [7, 2, 0],
    [7, 3, 1],
    [6, 3, 1],
    [6, 2, 1],
    [5, 2, 0],
    [5, 3, 0],
    [4, 3, 1],
    [4, 2, 1],
    [3, 2, 0],
    [3, 3, 0],
    [2, 3, 0],
    [2, 2, 0],
    [1, 2, 1],
    [1, 3, 1],
    [0, 3, 0],
    [0, 4, 0],
    [1, 4, 1],
    [1, 5, 0],
    [2, 5, 1],
    [2, 4, 0],
    [3, 4, 0],
    [3, 5, 1],
    [4, 5, 0],
    [4, 4, 1],
    [5, 4, 0],
    [5, 5, 0],
    [6, 5, 1],
    [6, 4, 1],
    [7, 4, 0],
    [7, 5, 0],
    [8, 5, 0],
    [8, 4, 1],
    [9, 4, 0],
    [9, 5, 1],
    [10, 5, 0],
    [10, 4, 0],
    [11, 4, 1],
    [11, 5, 0],
    [12, 5, 1],
    [12, 4, 0],
    [13, 4, 0],
    [13, 5, 1],
    [14, 5, 0],
    [14, 4, 1],
    [15, 4, 0],
    [15, 5, 0],
    [16, 5, 1],
    [16, 4, 1],
    [17, 4, 0],
    [17, 5, 0],
    [18, 5, 0],
    [18, 4, 0],
    [19, 4, 1],
    [19, 5, 1],
    [20, 5, 0],
    [20, 4, 0],
    [21, 4, 1],
    [21, 5, 1],
    [22, 5, 0],
    [22, 4, 0],
    [23, 4, 0],
    [23, 5, 0],
    [24, 5, 0],
    [24, 4, 0],
    [25, 4, 0],
    [25, 5, 0],
    [26, 5, 1],
    [26, 4, 1],
    [27, 4, 0],
    [27, 5, 1],
    [28, 5, 1],
    [28, 4, 0],
    [29, 4, 0],
    [29, 5, 1],
    [30, 5, 0],
    [30, 4, 0],
    [31, 4, 1],
    [31, 5, 0],
    [32, 5, 1],
    [32, 4, 0],
    [33, 4, 0],
    [33, 5, 1],
    [34, 5, 0],
    [34, 4, 1],
    [35, 4, 0],
    [35, 5, 0],
    [36, 5, 1],
    [36, 4, 1],
    [37, 4, 0],
    [37, 5, 0],
    [38, 5, 0],
    [38, 4, 0],
    [39, 4, 0],
    [39, 5, 0],
    [40, 5, 0],
    [40, 4, 0]
  ];

  const x = Math.max(...input.map(item => item[0])) + 1;
  const y = Math.max(...input.map(item => item[1])) + 1;
  console.log(x, y);

  const grid = new Array(y);
  for (let i = 0; i < y; i++) {
    grid[i] = new Array(x).fill(0);
  }

  input.map(item => (grid[item[1]][item[0]] = item[2]));
  for (let i = 0; i < y; i++) {
    const row = grid[i].map(spot => (spot == 1 ? "□" : "■")).join("");
    console.log(row);
  }
}

function main2() {}

export { main, main2, HullPaintingRobot, Direction, Hull };

// const codes = input.one.split(",").map(x => parseInt(x));
// const res = main(codes);

// console.log(res);
//4248984
/*
8,1
9,0
9,1
10,0
9,2
11,1
12,1
11,2
15,1
.
.
.
6,1
6,0
7,0
8,0
10,1
14,0
16,1
13,3
14,3
*/
