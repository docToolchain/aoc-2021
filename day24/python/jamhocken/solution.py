def process_input(file_contents):
    commands = list()
    
    for line in file_contents:
        line = line.strip()
        command = line[0:3]
        var1 = line[4]
        if command != "inp":
            var2 = line[6:len(line)]
            if var2[0] not in ["w","x","y","z"]:
                var2 = int(var2)
            commands.append((command,var1,var2))
        else:
            commands.append((command,var1))

    return commands

def main():
    with open("input.txt",'r') as alu_file:
        alu_lines = alu_file.readlines()

    commands = process_input(alu_lines)

    for x_str in ["97919997299495","51619131181131"]:
        i = 0
        variables = {"w":0,"x":0,"y":0,"z":0}

        for command in commands:
            if command[0] == "inp":
                variables[command[1]] = int(x_str[i])
                i += 1
            elif command[0] == "add":
                if isinstance(command[2], int):
                    variables[command[1]] = variables[command[1]] + command[2]
                else:
                    variables[command[1]] = variables[command[1]] + variables[command[2]]
            elif command[0] == "mul":
                if isinstance(command[2], int):
                    variables[command[1]] = variables[command[1]] * command[2]
                else:
                    variables[command[1]] = variables[command[1]] * variables[command[2]]
            elif command[0] == "div":
                if isinstance(command[2], int):
                    variables[command[1]] = variables[command[1]] // command[2]
                else:
                    variables[command[1]] = variables[command[1]] // variables[command[2]]
            elif command[0] == "mod":
                if isinstance(command[2], int):
                    variables[command[1]] = variables[command[1]] % command[2]
                else:
                    variables[command[1]] = variables[command[1]] % variables[command[2]]
            else:
                if isinstance(command[2], int):
                    if variables[command[1]] == command[2]:
                        variables[command[1]] = 1
                    else:
                        variables[command[1]] = 0
                else:
                    if variables[command[1]] == variables[command[2]]:
                        variables[command[1]] = 1
                    else:
                        variables[command[1]] = 0

        print(variables["z"])

main()
