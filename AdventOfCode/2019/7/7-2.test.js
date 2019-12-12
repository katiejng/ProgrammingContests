import { permute, doItLikeBefore, doIt } from "./7-2";
import input from "./in7-2";
test("test permute", () => {
  expect(permute([1, 2])).toStrictEqual([
    [1, 2],
    [2, 1]
  ]);
});

test("7-1 case one", () => {
  const list = input.one.split(",").map(x => parseInt(x));

  const result = doItLikeBefore([4, 3, 2, 1, 0], list);
  expect(result).toStrictEqual([43210]);
});

test("7-1 case two", () => {
  const list = input.two.split(",").map(x => parseInt(x));

  const result = doItLikeBefore([0, 1, 2, 3, 4], list);
  expect(result).toStrictEqual([54321]);
});

test("7-1 case three", () => {
  const list = input.three.split(",").map(x => parseInt(x));

  const result = doItLikeBefore([1, 0, 4, 3, 2], list);
  expect(result).toStrictEqual([65210]);
});

test("7-1 case four", () => {
  const list = input.four.split(",").map(x => parseInt(x));

  const result = doIt([9, 8, 7, 6, 5], list);
  expect(result).toStrictEqual([139629729]);
});

test("7-1 case five", () => {
  const list = input.five.split(",").map(x => parseInt(x));

  const result = doIt([9, 7, 8, 5, 6], list);
  expect(result).toStrictEqual([18216]);
});
// test("7-1 case real", () => {
//   const list = input.value.split(",").map(x => parseInt(x));

//   const result = doIt([9, 8, 7, 6, 5], list);
//   expect(result).toStrictEqual([139629729]);
// });
