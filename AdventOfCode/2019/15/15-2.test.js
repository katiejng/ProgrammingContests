import { main, calculatePositionInfo, getNextStep } from "./15-2";
import input from "./in15-2";
import IntCodeComputer from "./IntCode";

test("test2 calculatePositionInfo ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = calculatePositionInfo(computer, 1);
  expect(result).toBe(0);
  expect(computer.outputs).toStrictEqual([]);
});
test("test2 calculatePositionInfo ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = calculatePositionInfo(computer, 2);
  expect(result).toBe(0);
  expect(computer.outputs).toStrictEqual([]);
});
test("test2 calculatePositionInfo ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = calculatePositionInfo(computer, 3);
  expect(result).toBe(0);
  expect(computer.outputs).toStrictEqual([]);
});
test("test2 calculatePositionInfo ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = calculatePositionInfo(computer, 4);
  expect(result).toBe(1);
  expect(computer.outputs).toStrictEqual([]);
});
test("test2 getNextStep ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = getNextStep(2, computer);
  expect(result).toStrictEqual([
    { direction: 1, computer: computer },
    { direction: 2, computer: computer },
    { direction: 3, computer: computer }
  ]);
  expect(computer.outputs).toStrictEqual([]);
});

test("test2 getNextStep ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = getNextStep(1, computer);
  expect(result).toStrictEqual([
    { direction: 1, computer: computer },
    { direction: 3, computer: computer },
    { direction: 4, computer: computer }
  ]);
  expect(computer.outputs).toStrictEqual([]);
});

test("test2 getNextStep ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = getNextStep(3, computer);
  expect(result).toStrictEqual([
    { direction: 1, computer: computer },
    { direction: 2, computer: computer },
    { direction: 4, computer: computer }
  ]);
  expect(computer.outputs).toStrictEqual([]);
});

test("test2 getNextStep ", () => {
  const codes = input.value.split(",").map(x => parseInt(x));
  const computer = new IntCodeComputer("A", codes, []);
  const result = getNextStep(4, computer);
  expect(result).toStrictEqual([
    { direction: 1, computer: computer },
    { direction: 2, computer: computer },
    { direction: 4, computer: computer }
  ]);
  expect(computer.outputs).toStrictEqual([]);
});
test("test2 main ", () => {
  const result = main();
  expect(result).toBe(1);
});
