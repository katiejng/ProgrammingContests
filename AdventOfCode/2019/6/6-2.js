const input = require('./in6-2');

function buildGraph(list){
    let graph = {}
    for (let i = 0; i < list.length; i++){
        if ( list[i][0] in graph){
            graph[list[i][0]].push(list[i][1])
        }else{
            graph[list[i][0]] = [list[i][1]]
        }

        if ( list[i][1] in graph){
            graph[list[i][1]].push(list[i][0])
        }else{
            graph[list[i][1]] = [list[i][0]]
        }

    }
    return graph
}

function getSum(graph){
    let sum = 0
    let row = graph.COM
    let level = 1

    while (row.length > 0){
        sum += row.length * level
        let newRow = []
        for (let i = 0; i < row.length; i++){
            // add to newRow
            console.log(row[i])
            const temp = graph[row[i]]
            if (temp){
                newRow.push(...temp)
            }
        }

        level +=1
        row = newRow
    }

    return sum
}

const getItemsInYNotInX = (y,x) => y.filter( function( el ) {
    return x.indexOf( el ) < 0;
  });

function getShortestPath(start, end, graph){
    let distance = 0
    let row = graph[start]
    let visited = [start]

    while (row.length>0){
        if (row.indexOf(end) >= 0){
            return distance -1
        }
        distance += 1
        let newRow = []
        visited.push(...row)

        for (let i = 0; i < row.length; i++){
            const temp = graph[row[i]]

            if (temp){
                let notVisited = getItemsInYNotInX(temp, visited)
                newRow.push(...notVisited)
            }
        }
        row = newRow
    }
    return distance
}


const parsedInput = input.value.split('\n').map(orbit => orbit.split(")"))
const graph = buildGraph(parsedInput)

const result = getShortestPath("YOU", "SAN", graph)

console.log(graph, result)