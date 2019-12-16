import { main, main2, Hull, HullPaintingRobot, Direction } from "./11-2";
import input from "./in11-2";
import IntCodeComputer from "./IntCode";
test("", () => {});

// test("test2 four ", () => {
//   const result = main();
//   expect(result).toStrictEqual(0);
// });

// test("robot processdirection", () => {
//   const program = input.value.split(",").map(num => parseInt(num));
//   const hull = new Hull();
//   const computer = new IntCodeComputer("A", program, []);
//   const robot = new HullPaintingRobot(computer, hull);
//   robot.processDirection(0);
//   expect(robot.direction).toBe(Direction.LEFT);
// });

// test("robot processdirection", () => {
//   const program = input.value.split(",").map(num => parseInt(num));
//   const hull = new Hull();
//   const computer = new IntCodeComputer("A", program, []);
//   const robot = new HullPaintingRobot(computer, hull);
//   robot.processDirection(1);
//   expect(robot.direction).toBe(Direction.RIGHT);
//   robot.processDirection(1);
//   expect(robot.direction).toBe(Direction.DOWN);
//   robot.processDirection(1);
//   expect(robot.direction).toBe(Direction.LEFT);
//   robot.processDirection(1);
//   expect(robot.direction).toBe(Direction.UP);
// });
