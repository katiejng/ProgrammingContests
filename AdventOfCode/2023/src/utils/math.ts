// lowest common multiple of an array
export const lcm = (numbers: number[]) => {
  return numbers.reduce((acc, curr) => {
    return (acc * curr) / gcd(acc, curr);
  });
};

const gcd = (a: number, b: number): number => {
  if (b === 0) {
    return a;
  }

  return gcd(b, a % b);
};
export const solveQuadraticFormula = (a: number, b: number, c: number) => {
  const discriminant = Math.pow(b, 2) - 4 * a * c;
  const sqrtDiscriminant = Math.sqrt(discriminant);
  const solution1 = (-b + sqrtDiscriminant) / (2 * a);
  const solution2 = (-b - sqrtDiscriminant) / (2 * a);
  return [solution1, solution2];
};
