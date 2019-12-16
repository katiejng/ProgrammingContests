import input from "./in10-2";
// indices are map[y][x] !!!

function calcAngles(map: Array<Array<string>>, x1, y1, x, y) {
  const angles = [];
  for (let y2 = 0; y2 < y; y2++) {
    // for each row
    for (let x2 = 0; x2 < x; x2++) {
      // for each col
      if (map[y2][x2] === "#") {
        // deal with star
        if (y2 === y1 && x1 == x2) {
          // add left /right/same

          continue;
        } else {
          // calculate angle
          const xDelta = x2 - x1;
          const yDelta = y2 - y1;
          const angle = Math.atan2(yDelta, xDelta);
          const distance = Math.sqrt(Math.pow(xDelta, 2) + Math.pow(yDelta, 2));
          angles.push({ angle, distance, x2, y2 });
        }
      } else {
        // if not star, skip
        continue;
      }
    }
  }
  return angles
    .map(angle => {
      // if angle < -pi /2, make it 2PI - angle
      if (angle.angle < -(Math.PI / 2)) {
        angle.angle = 2 * Math.PI + angle.angle;
      }
      return angle;
    })
    .sort((a, b) => a.angle - b.angle || a.distance - b.distance);
}
function main(map: Array<Array<string>>) {
  const y = map.length;
  const x = map[0].length;
  const result = new Array(map.length);
  for (let i = 0; i < y; i++) {
    result[i] = new Array(x).fill(0);
  }
  let maxVisible = 0;
  let resultIndex = [0, 0]; //x,y

  for (let y1 = 0; y1 < y; y1++) {
    // for each row
    for (let x1 = 0; x1 < x; x1++) {
      // for each col
      if (map[y1][x1] === "#") {
        // deal with star
        // calculate angles
        const angles = new Set(
          calcAngles(map, x1, y1, x, y).map(angles => angles.angle)
        );
        const uniqueAngles = new Set(angles);
        const visible = uniqueAngles.size;
        if (visible > maxVisible) {
          maxVisible = visible;
          resultIndex = [x1, y1];
        }
        result[y1][x1] = visible;
      } else {
        // if not star, skip
        continue;
      }
    }
  }
  return resultIndex;
}

function main2(map: Array<Array<string>>, xMarker: number, yMarker: number) {
  const y = map.length;
  const x = map[0].length;
  const result = new Array(map.length);
  for (let i = 0; i < y; i++) {
    result[i] = new Array(x).fill(0);
  }

  const angles = calcAngles(map, xMarker, yMarker, x, y);
  const order = [];
  let index = 0;
  let lastAngle = { angle: -1 };
  let looped = true;
  while (angles.length > 0) {
    // if index > list, go to start
    if (index >= angles.length) {
      console.log("LOOPING");
      looped = true;
      index = 0;
    }
    // if current angle is the last angle, go to next pointer, or just looped

    if (lastAngle.angle == angles[index].angle && !looped) {
      // skip
      index++;
      continue;
    }
    // deal
    lastAngle = angles[index];
    order.push(lastAngle);
    angles.splice(index, 1);

    looped = false;
  }

  console.log(
    order
      .map((order, index) => `(${index + 1}, ${order.x2}, ${order.y2})`)
      .join("")
  );

  return order[199];
}

export { main, main2 };

// const codes = input.one.split(",").map(x => parseInt(x));
// const res = main(codes);

// console.log(res);
//4248984
/*
8,1
9,0
9,1
10,0
9,2
11,1
12,1
11,2
15,1
.
.
.
6,1
6,0
7,0
8,0
10,1
14,0
16,1
13,3
14,3
*/
