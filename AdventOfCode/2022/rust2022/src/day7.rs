use std::collections::HashMap;

#[derive(Debug)]
pub struct File {
    name: String,
    file_size: usize,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
}

#[derive(Debug)]
pub struct CdCommand {
    input: String,
}

#[derive(Debug)]
pub struct LsCommand {
    items: Vec<Item>,
}

#[derive(Debug)]
enum Item {
    File(File),
    Directory(Directory),
}

impl Item {
    fn name(&self) -> String {
        match self {
            Item::Directory(directory) => directory.name.clone(),
            Item::File(file) => file.name.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    CD(CdCommand),
    LS(LsCommand),
}

#[derive(Debug)]

struct Node {
    path: String,
    children: Vec<String>,
    size: usize,
}
#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    let mut line_index = 0;
    let input_lines: Vec<&str> = input.lines().collect();

    loop {
        if line_index >= input_lines.len() {
            return commands;
        }
        let split_line: Vec<&str> = input_lines[line_index].split_whitespace().collect();

        if split_line[0] == "$" {
            // is a command
            if split_line[1] == "cd" {
                commands.push(Command::CD(CdCommand {
                    input: String::from(split_line[2]),
                }));

                line_index += 1;
            } else if split_line[1] == "ls" {
                let mut items: Vec<Item> = vec![];

                line_index += 1;

                loop {
                    if line_index >= input_lines.len() {
                        break;
                    }
                    let split_output: Vec<&str> =
                        input_lines[line_index].split_whitespace().collect();

                    if split_output[0] == "$" {
                        break;
                    }

                    let result: Item = match split_output[0] {
                        "dir" => Item::Directory(Directory {
                            name: String::from(split_output[1]),
                        }),
                        _ => Item::File(File {
                            file_size: split_output[0].trim().parse().unwrap(),
                            name: String::from(split_output[1]),
                        }),
                    };

                    items.push(result);

                    line_index += 1
                }

                commands.push(Command::LS(LsCommand { items }));
            }
        }
    }
}

fn create_folder_structure(input: &[Command]) -> HashMap<String, Node> {
    let mut files: HashMap<String, Node> = HashMap::new();

    let mut current_path: Vec<String> = vec![];

    input.into_iter().for_each(|command| match command {
        Command::LS(ls_command) => {
            // create the node

            let node_path: String = current_path.join("/");

            let node = Node {
                path: node_path.to_string(),
                children: ls_command
                    .items
                    .iter()
                    .map(|item| vec![node_path.to_string(), item.name()].join("/"))
                    .collect(),
                size: 0,
            };

            ls_command.items.iter().for_each(|item| match item {
                Item::File(file) => {
                    let file_path = vec![node_path.to_string(), item.name()].join("/");
                    let node = Node {
                        path: file_path.to_string(),
                        size: file.file_size,
                        children: vec![],
                    };
                    files.insert(file_path, node);
                    return;
                }
                Item::Directory(_) => return,
            });

            files.insert(node_path, node);
            return;
        }
        Command::CD(cd_command) => match cd_command.input.as_str() {
            ".." => {
                current_path.pop();
            }
            _ => current_path.push(cd_command.input.to_string()),
        },
    });

    files
}

fn get_folder_sizes(
    results: &mut HashMap<String, usize>,
    files: &HashMap<String, Node>,
    current_path: String,
) -> usize {
    let current_file = files.get(&current_path).unwrap();

    if current_file.children.len() == 0 {
        // results.insert(current_file.path.to_string(), current_file.size);
        return current_file.size;
    }

    let children_size: usize = current_file
        .children
        .iter()
        .map(|child| get_folder_sizes(results, files, child.to_string()))
        .sum();

    let total = current_file.size + children_size;

    results.insert(current_file.path.to_string(), total);
    return total;
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Command]) -> usize {
    let folder_structure = create_folder_structure(input);

    let mut results: HashMap<String, usize> = HashMap::new();
    get_folder_sizes(&mut results, &folder_structure, "/".to_string());

    results.values().filter(|&&v| v <= 100000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Command]) -> usize {
    let folder_structure = create_folder_structure(input);

    let mut results: HashMap<String, usize> = HashMap::new();

    get_folder_sizes(&mut results, &folder_structure, "/".to_string());

    let mut results_values: Vec<usize> = results.into_values().collect();
    results_values.sort();

    let free_space = 70000000 - results_values.last().unwrap().clone();
    let space_to_clear = 30000000 - free_space;
    results_values
        .into_iter()
        .find(|&item| item >= space_to_clear)
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUTS: &[&str] = &["$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"];

    #[test]
    fn part1_test_1() {
        const TEST_RESULTS: &[usize] = &[95437];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part1(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }

    #[test]
    fn part2_test_1() {
        const TEST_RESULTS: &[usize] = &[24933642];
        (0..TEST_INPUTS.len()).for_each(|index| {
            let parsed_input = input_generator(TEST_INPUTS[index]);
            let result = solve_part2(&parsed_input);
            let expected = TEST_RESULTS[index];
            assert_eq!(result, expected);
        })
    }
}
