import { main } from "./9-2";
import input from "./in9-2";

test("", () => {});
// test("one", () => {
//   const codes = input.one.split(",").map(x => parseInt(x));
//   const res = main(codes);
//   const expected = [
//     109, //  change rel base to 1
//     1,
//     204, // output relative base of 1-1 = 0 = 109
//     -1,
//     1001, // Add 1 to position 100 and store in pos 100
//     100,
//     1,
//     100,
//     1008,
//     100,
//     16,
//     101,
//     1006,
//     101,
//     0,
//     99
//   ];
//   expect(res).toStrictEqual(expected);
// });

// test("two", () => {
//   const codes = input.two.split(",").map(x => parseInt(x));
//   const res = main(codes);
//   const expected = [1219070632396864];
//   expect(res).toStrictEqual(expected);
// });

// test("three", () => {
//   const codes = input.three.split(",").map(x => parseInt(x));
//   const res = main(codes);
//   const expected = [1125899906842624];
//   expect(res).toStrictEqual(expected);
// });

// test("real", () => {
//   const codes = input.value.split(",").map(x => parseInt(x));
//   const myInput = [2];
//   const res = main(codes, myInput);
//   const expected = [-1];
//   expect(res).toStrictEqual(expected);
// });

// test("one", () => {
//   const codes = [109, -1, 4, 1, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [-1];
//   expect(res).toStrictEqual(expected);
// });
// test("two", () => {
//   const codes = [109, -1, 104, 1, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [1];
//   expect(res).toStrictEqual(expected);
// });
// test("three", () => {
//   const codes = [109, -1, 204, 1, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [109];
//   expect(res).toStrictEqual(expected);
// });
// test("four", () => {
//   const codes = [109, 1, 9, 2, 204, -6, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [204];
//   expect(res).toStrictEqual(expected);
// });
// test("five", () => {
//   const codes = [109, 1, 109, 9, 204, -6, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [204];
//   expect(res).toStrictEqual(expected);
// });
// test("six", () => {
//   const codes = [109, 1, 209, -1, 204, -106, 99];
//   const myInput = [];
//   const res = main(codes, myInput);
//   const expected = [204];
//   expect(res).toStrictEqual(expected);
// });
// test("seven", () => {
//   const codes = [109, 1, 3, 3, 204, 2, 99];
//   const myInput = [123];
//   const res = main(codes, myInput);
//   const expected = [123];
//   expect(res).toStrictEqual(expected);
// });
// test("eight", () => {
//   const codes = [109, 1, 203, 2, 204, 2, 99];
//   const myInput = [123];
//   const res = main(codes, myInput);
//   const expected = [123];
//   expect(res).toStrictEqual(expected);
// });

// test("input", () => {
//   const codes = [203, 5, 4, 5, 99, 12345];
//   const myInput = [1];
//   const res = main(codes, myInput);
//   const expected = [1];
//   expect(res).toStrictEqual(expected);
// });
