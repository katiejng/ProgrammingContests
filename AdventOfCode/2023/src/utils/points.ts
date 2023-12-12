export type Point = { r: number; c: number };

export const addPoints = (a: Point, b: Point) => {
  return { r: a.r + b.r, c: a.c + b.c };
};

export const addDirection = (point: Point, direction: string) => {
  switch (direction) {
    case "N":
      return addPoints(point, { r: -1, c: 0 });
    case "S":
      return addPoints(point, { r: 1, c: 0 });
    case "E":
      return addPoints(point, { r: 0, c: 1 });
    case "W":
      return addPoints(point, { r: 0, c: -1 });
    default:
      throw new Error("Invalid direction");
  }
};

export const isValidPoint = (point: Point, bounds: Point) => {
  return (
    point.r >= 0 && point.r < bounds.r && point.c >= 0 && point.c < bounds.c
  );
};
