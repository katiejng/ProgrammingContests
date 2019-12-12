import input from "./in8-2";
console.log(input.small);

function count(layer: string) {
  const result = new Array(10).fill(0);
  layer.split("").forEach(letter => result[parseInt(letter)]++);
  return result;
}

function getMin(layers: any) {
  let minValue = layers[0][0];
  let minIndex = 0;

  for (let i = 0; i < layers.length; i++) {
    if (layers[i][0] < minValue) {
      minValue = layers[i][0];
      minIndex = i;
    }
  }
  return layers[minIndex];
}

function getLayerWithLeast0s(image: string, height: number, width: number) {
  const pixels = height * width;
  const totalPixels = image.length;
  const layers = totalPixels / pixels;

  const results = [];
  for (let i = 0; i < layers; i++) {
    results.push(count(image.slice(i * pixels, (i + 1) * pixels)));
  }
  const minLayer = getMin(results);
  return minLayer[1] * minLayer[2];
}

function getImage(image: string, width: number, height: number) {
  const pixels = height * width;
  const totalPixels = image.length;
  const layers = totalPixels / pixels;

  const results = new Array(height).fill(new Array(width).fill(2));
  // for (let i = 0; i < layers; i++) {
  //   results.push(count(image.slice(i * pixels, (i + 1) * pixels)));
  // }
  return results;
}

function printImage(image: Array<Array<number>>) {
  for (let i = 0; i < image.length; i++) {
    console.log(image[i].join(""));
  }
}

// const res = getLayerWithLeast0s(input.value, 25, 6);
// console.log(res);

const res = getImage(input.value, 25, 6);
printImage(res);
