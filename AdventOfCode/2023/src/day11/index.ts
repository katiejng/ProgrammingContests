import { Point } from "../utils/points";

enum Type {
  Galaxy = "#",
  Space = ".",
  Empty = "X",
}

const parseInput = (input: string) => {
  return {
    world: input
      .split("\n")
      .map((line) =>
        line.split("").map((item) => (item === "#" ? Type.Galaxy : Type.Space))
      ),
  };
};

export const day11part1 = (input: string) => {
  const { world } = parseInput(input);

  const emptyRowIndexes = getEmptyRows(world);
  const emptyColumnIndexes = getEmptyColumns(world);

  let expandedWorld: Type[][] = world;

  for (let i = emptyRowIndexes.length - 1; i >= 0; i--) {
    expandedWorld = insertRow(emptyRowIndexes[i], expandedWorld);
  }
  for (let i = emptyColumnIndexes.length - 1; i >= 0; i--) {
    expandedWorld = insertColumn(emptyColumnIndexes[i], expandedWorld);
  }

  // printMap(expandedWorld);

  const galaxyPositions: Point[] = expandedWorld
    .flatMap((row, rowIndex) =>
      row.map((item, columnIndex) => {
        if (item === Type.Galaxy) {
          return {
            r: rowIndex,
            c: columnIndex,
          };
        }
      })
    )
    .filter((row): row is Point => row !== undefined);

  const results: number[] = [];
  for (let i = 0; i < galaxyPositions.length - 1; i++) {
    for (let j = i + 1; j < galaxyPositions.length; j++) {
      const p1 = galaxyPositions[i];
      const p2 = galaxyPositions[j];
      const orthoganalDistance = Math.abs(p1.r - p2.r) + Math.abs(p1.c - p2.c);
      results.push(orthoganalDistance);
    }
  }

  return results.reduce((acc, curr) => acc + curr, 0);
};
const printMap = (map: Type[][]) => {
  console.log(map.map((row) => row.join("")).join("\n"));
};
export const day11part2 = (input: string, expansionFactor = 1000000) => {
  const { world } = parseInput(input);

  const emptyRowIndexes = getEmptyRows(world);
  const emptyColumnIndexes = getEmptyColumns(world);

  let expandedWorld: Type[][] = world;

  // printMap(expandedWorld);

  const galaxyPositions: Point[] = expandedWorld
    .flatMap((row, rowIndex) =>
      row.map((item, columnIndex) => {
        if (item === Type.Galaxy) {
          return {
            r: rowIndex,
            c: columnIndex,
          };
        }
      })
    )
    .filter((row): row is Point => row !== undefined);

  const results: number[] = [];
  for (let i = 0; i < galaxyPositions.length - 1; i++) {
    for (let j = i + 1; j < galaxyPositions.length; j++) {
      const p1 = galaxyPositions[i];
      const p2 = galaxyPositions[j];

      const expandedRowIndices = emptyRowIndexes.filter(
        (rowIndex) =>
          rowIndex > Math.min(p1.r, p2.r) && rowIndex < Math.max(p1.r, p2.r)
      );
      const expandedColumnIndices = emptyColumnIndexes.filter(
        (columnIndex) =>
          columnIndex > Math.min(p1.c, p2.c) &&
          columnIndex < Math.max(p1.c, p2.c)
      );

      const addedRows = expandedRowIndices.length * (expansionFactor - 1);
      const addedCols = expandedColumnIndices.length * (expansionFactor - 1);

      const orthoganalDistance = Math.abs(p1.r - p2.r) + Math.abs(p1.c - p2.c);
      results.push(orthoganalDistance + addedCols + addedRows);
    }
  }

  return results.reduce((acc, curr) => acc + curr, 0);
};

const getEmptyRows = (world: Type[][]) => {
  const emptyRowIndexes: number[] = [];
  world.forEach((row, index) => {
    if (row.some((item) => item === Type.Galaxy)) {
      return;
    }
    emptyRowIndexes.push(index);
  });

  return emptyRowIndexes;
};

const getEmptyColumns = (world: Type[][]) => {
  const emptyColumnIndexes: number[] = [];
  const cols = world[0].length;
  for (let i = 0; i < cols; i++) {
    if (world.some((row) => row[i] === Type.Galaxy)) {
      continue;
    }
    emptyColumnIndexes.push(i);
  }

  return emptyColumnIndexes;
};

const insertRow = (row: number, world: Type[][]) => {
  const bounds = {
    r: world.length,
    c: world[0].length,
  };
  return [
    ...world.slice(0, row),
    new Array(bounds.c).fill(Type.Empty),
    ...world.slice(row, bounds.r),
  ];
};

const insertColumn = (column: number, world: Type[][]) => {
  const bounds = {
    r: world.length,
    c: world[0].length,
  };
  return world.map((row) => [
    ...row.slice(0, column),
    Type.Empty,
    ...row.slice(column, bounds.c),
  ]);
};
