const input = require("./3-1-input");

const parsedInput = input.value.split("\n").map(x => x.split(","));

function transformMovement(movement) {
  value = parseInt(movement.slice(1));
  switch (movement[0]) {
    case "R":
      return { direction: "x", value };
    case "U":
      return { direction: "y", value: -value };
    case "D":
      return { direction: "y", value: value };
    case "L":
      return { direction: "x", value: -value };
    default:
      break;
  }
}

function handleMovement(movement, start, line) {
  if (movement.direction === "x") {
    if (movement.value >= 0) {
      for (let i = 0; i < movement.value; i++) {
        start = [start[0] + 1, start[1]];
        line.push(start.join(","));
      }
    } else {
      for (let i = 0; i < -movement.value; i++) {
        start = [start[0] - 1, start[1]];
        line.push(start.join(","));
      }
    }
  } else {
    if (movement.value >= 0) {
      for (let i = 0; i < movement.value; i++) {
        start = [start[0], start[1] + 1];
        line.push(start.join(","));
      }
    } else {
      for (let i = 0; i < -movement.value; i++) {
        start = [start[0], start[1] - 1];
        line.push(start.join(","));
      }
    }
  }
  return { start, line };
}
function getVisitedIndices(list) {
  var line = [];
  var start = [0, 0];

  for (let i = 0; i < list.length; i++) {
    let movement = transformMovement(list[i]);
    let result = handleMovement(movement, start, line);
    line = result.line;
    start = result.start;
  }
  return line;
}
function getDistance(start, end) {
  return Math.abs(start[0] - end[0]) + Math.abs(start[1] - end[1]);
}

const getIntersection = (array1, array2) =>
  array1.filter(value => -1 !== array2.indexOf(value));

const result1 = getVisitedIndices(parsedInput[0]);
const result2 = getVisitedIndices(parsedInput[1]);
const intersection = getIntersection(result1, result2);
const steps = intersection.map(
  point => result1.indexOf(point) + result2.indexOf(point) + 2
);
console.log(Math.min(...steps));
