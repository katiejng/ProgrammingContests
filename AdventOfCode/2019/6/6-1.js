const input = require("./in6-1");

function buildGraph(list) {
  let graph = {};
  for (let i = 0; i < list.length; i++) {
    if (list[i][0] in graph) {
      graph[list[i][0]].push(list[i][1]);
    } else {
      graph[list[i][0]] = [list[i][1]];
    }
  }
  return graph;
}

function getSum(graph) {
  let sum = 0;
  let row = graph.COM;
  let level = 1;

  while (row.length > 0) {
    sum += row.length * level;
    let newRow = [];
    for (let i = 0; i < row.length; i++) {
      // add to newRow
      console.log(row[i]);
      const temp = graph[row[i]];
      if (temp) {
        newRow.push(...temp);
      }
    }

    level += 1;
    row = newRow;
  }

  return sum;
}

const parsedInput = input.value.split("\n").map(orbit => orbit.split(")"));
const graph = buildGraph(parsedInput);

const result = getSum(graph);

console.log(result);
