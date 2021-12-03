class Submarine:
    depth = 0
    horizontal_pos = 0

    def down(self, depth):
        self.depth += depth

    def up(self, depth):
        self.depth -= depth

    def forward(self, distance):
        self.horizontal_pos += distance

    def get_depth(self):
        return self.depth

    def get_horizontal_pos(self):
        return self.horizontal_pos

    def dive(self, input):
        with open(input, "r") as f:
            for line in f:
                (direction, value) = line.split(" ")
                value = int(value)
                if direction == "down":
                    self.down(value)
                elif direction == "up":
                    self.up(value)
                elif direction == "forward":
                    self.forward(value)   
                else:
                    raise Exception(f"Unknown direction!")
                
def get_Star1():
    submarine = Submarine()
    submarine.dive("input.txt")
    result = submarine.get_depth() * submarine.get_horizontal_pos()

    print(f"Result for first star: {result}") 

class Aiming_Submarine:
    depth = 0
    horizontal_pos = 0
    aim = 0

    def down(self, depth):
        self.aim += depth

    def up(self, depth):
        self.aim -= depth

    def forward(self, distance):
        self.horizontal_pos += distance
        self.depth += distance * self.aim

    def get_depth(self):
        return self.depth

    def get_horizontal_pos(self):
        return self.horizontal_pos

    def dive(self, input):
        with open(input, "r") as f:
            for line in f:
                (direction, value) = line.split(" ")
                value = int(value)
                if direction == "down":
                    self.down(value)
                elif direction == "up":
                    self.up(value)
                elif direction == "forward":
                    self.forward(value)   
                else:
                    raise Exception(f"Unknown direction!")
                
def get_Star2():
    submarine = Aiming_Submarine()
    submarine.dive("input.txt")
    result = submarine.get_depth() * submarine.get_horizontal_pos()

    print(f"Result for second star: {result}") 

get_Star1()
get_Star2()