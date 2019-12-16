enum CommandType {
  ZERO = 0,
  ADDITION = 1,
  MULTIPLICATION,
  SAVE,
  OUTPUT,
  JUMP_IF_TRUE,
  JUMP_IF_FALSE,
  LESS_THAN,
  EQUALS,
  SETRELATIVEBASE,
  EXIT
}

class Command {
  commandType: CommandType;
  immediate: Array<number>;
  values: Array<number>;
  indices: Array<number>;

  constructor(opcode: number) {
    this.parseInput(opcode);
    this.values = [];
    this.indices = [];
  }

  parseInput(command: number) {
    if (command == 99) {
      this.commandType = 99;
      return;
    }
    const splitCommand = command
      .toString()
      .split("")
      .map(x => parseInt(x));
    while (splitCommand.length < 5) {
      splitCommand.splice(0, 0, 0);
    }

    this.commandType = splitCommand[4];

    this.immediate = [splitCommand[2], splitCommand[1], splitCommand[0]];
  }

  parseOpcode(
    start: number,
    n: number,
    codes: CodeMemory,
    relativeBase: number
  ) {
    for (let i = 0; i < n; i++) {
      let index = codes.get(start + i + 1);
      switch (this.immediate[i]) {
        case 0:
          this.values.push(codes.get(index));
          this.indices.push(index);

          break;
        case 1:
          this.values.push(index);
          this.indices.push(index);

          break;
        case 2:
          this.indices.push(relativeBase + index);

          this.values.push(codes.get(relativeBase + index));
      }
    }
  }
}

class CodeMemory {
  map: any;
  constructor(list: Array<number>) {
    this.map = {};
    for (let i = 0; i < list.length; i++) {
      this.set(i, list[i]);
    }
  }

  set(index, value) {
    this.map[index] = value;
  }

  get(index) {
    const item = this.map[index];
    return item ? item : 0;
  }
}

export default class IntCodeComputer {
  codes: CodeMemory;
  isDead: Boolean;
  name: String;
  sp: number;
  inputs: Array<number>;
  outputs: Array<number>;
  relativeBase: number;

  constructor(name: string, codes: Array<number>, inputs: Array<number>) {
    this.codes = new CodeMemory(codes);
    this.name = name;
    this.isDead = false;
    this.sp = 0;
    this.inputs = inputs;
    this.outputs = [];
    this.relativeBase = 0;
  }

  printAll() {
    console.log({
      ...this,
      codes: this.codes.toString()
    });
  }

  handleAddition(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes, this.relativeBase);
    this.codes.set(command.indices[2], command.values[0] + command.values[1]);
    this.sp = start + 4;
    return 0;
  }

  handleMultiplication(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes, this.relativeBase);
    this.codes.set(command.indices[2], command.values[0] * command.values[1]);
    this.sp = start + 4;
    return 0;
  }

  handleInput(start: number, command: Command) {
    if (this.inputs.length === 0) {
      return -1;
    }
    command.parseOpcode(start, 1, this.codes, this.relativeBase);
    this.codes.set(command.indices[0], this.inputs.shift());
    this.sp = start + 2;
    return 0;
  }

  handleOutput(start: number, command: Command) {
    command.parseOpcode(start, 1, this.codes, this.relativeBase);
    this.outputs.push(command.values[0]);

    this.sp = start + 2;
    return 0;
  }

  handleJumpIfTrue(start: number, command: Command) {
    command.parseOpcode(start, 2, this.codes, this.relativeBase);

    if (command.values[0] !== 0) {
      this.sp = command.values[1];
      return 0;
    }
    this.sp = start + 3;
    return 0;
  }

  handleJumpIfFalse(start: number, command: Command) {
    command.parseOpcode(start, 2, this.codes, this.relativeBase);

    if (command.values[0] === 0) {
      this.sp = command.values[1];
      return 0;
    }
    this.sp = start + 3;
    return 0;
  }

  handleLessThan(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes, this.relativeBase);

    if (command.values[0] < command.values[1]) {
      this.codes.set(command.indices[2], 1);
    } else {
      this.codes.set(command.indices[2], 0);
    }

    this.sp = start + 4;
    return 0;
  }

  handleEquals(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes, this.relativeBase);

    if (command.values[0] == command.values[1]) {
      this.codes.set(command.indices[2], 1);
    } else {
      this.codes.set(command.indices[2], 0);
    }

    this.sp = start + 4;
    return 0;
  }

  handleSetRelativeBase(start: number, command: Command) {
    command.parseOpcode(start, 1, this.codes, this.relativeBase);
    this.relativeBase = this.relativeBase + command.values[0];
    this.sp = start + 2;
    return 0;
  }

  printOutput() {
    console.log("--------------\nOUTPUT: ", this.outputs, "\n--------------");
  }

  processList() {
    while (true) {
      const command = new Command(this.codes.get(this.sp));
      let result;
      switch (command.commandType) {
        case CommandType.ZERO:
          result = -1;
          console.log("got 0");
          break;
        case CommandType.ADDITION:
          result = this.handleAddition(this.sp, command);
          break;
        case CommandType.MULTIPLICATION:
          result = this.handleMultiplication(this.sp, command);
          break;
        case CommandType.SAVE:
          result = this.handleInput(this.sp, command);
          break;
        case CommandType.OUTPUT:
          result = this.handleOutput(this.sp, command);
          break;
        case CommandType.JUMP_IF_TRUE:
          result = this.handleJumpIfTrue(this.sp, command);
          break;
        case CommandType.JUMP_IF_FALSE:
          result = this.handleJumpIfFalse(this.sp, command);
          break;
        case CommandType.LESS_THAN:
          result = this.handleLessThan(this.sp, command);
          break;
        case CommandType.EQUALS:
          result = this.handleEquals(this.sp, command);
          break;
        case CommandType.SETRELATIVEBASE:
          result = this.handleSetRelativeBase(this.sp, command);
          break;
        default:
          //EXIT = 99 but it's 9 :(
          this.isDead = true;
          return;
      }

      // console.log(command);

      if (result === -1) {
        return;
      }
    }
  }
}

export { IntCodeComputer };
