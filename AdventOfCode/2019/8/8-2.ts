import input from "./in8-2";

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

  const results = new Array(height * width).fill("2");
  for (let i = 0; i < layers; i++) {
    let layer = image.slice(i * pixels, (i + 1) * pixels);
    for (let j = 0; j < pixels; j++) {
      if (layer[j] !== "2" && results[j] === "2") {
        results[j] = layer[j];
      }
    }
  }
  return results;
}

function printImage(image: Array<string>, width: number, height: number) {
  for (let i = 0; i < height; i++) {
    console.log(
      image
        .slice(i * width, (i + 1) * width)
        .join("")
        .replace(/2/g, " ")
        .replace(/0/g, "■")
        .replace(/1/g, "□")
    );
  }
}

// const res = getLayerWithLeast0s(input.value, 25, 6);
// console.log(res);

let width = 25;
let height = 6;
const res = getImage(input.value, width, height);
printImage(res, width, height);
