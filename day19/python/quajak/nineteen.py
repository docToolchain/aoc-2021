from typing import List, Tuple

f = open("nineteen.txt", "r")
lines = [x.strip() for x in f.readlines()]

class Scanner:
    def __init__(self, id = int):
        self.raw_beacons = []
        self.beacons = set()
        self.id = id
        self.all = []
        self.offset = None
        
    def _rotate_coordinate(self, coord: List[int], rx: int, ry: int, rz: int) -> Tuple[int]:
        vec = tuple(coord)
        for i in range(rx):
            vec = (vec[0], -1 * vec[2], vec[1])
            
        for i in range(ry):
            vec = (vec[2], vec[1], vec[0] * -1)
            
        for i in range(rz):
            vec = (vec[1] * -1, vec[0], vec[2])
        return vec
    
    def generate_rotations(self) -> List[List[Tuple[int]]]:
        if len(self.all) != 0:
            return self.all
        
        for rx in range(4):
            for ry in range(4):
                for rz in range(4):
                    self.all.append([self._rotate_coordinate(coord, rx, ry, rz) for coord in self.raw_beacons])
        self.all = [list(x) for x in set(tuple(x) for x in self.all)]
        return self.all
        
scanners : List[Scanner] = []
current_scanner = None
for line in lines:
    if line.startswith("---"):
        line = line.replace("-","").strip().split()[1]
        current_scanner = Scanner(int(line))
        scanners.append(current_scanner)
    elif len(line) != 0:
        coords = [int(w) for w in line.split(",")]
        current_scanner.raw_beacons.append((coords[0], coords[1], coords[2]))
        
scanners[0].offset = (0, 0, 0)
scanners[0].beacons = set(scanners[0].raw_beacons)
print(len(scanners[0].generate_rotations()))

unlocalized = [scanner for scanner in scanners if scanner.offset is None]

while len(unlocalized) != 0:
    for scanner in unlocalized:
        for other in scanners:
            if other.offset is not None:
                for variation in scanner.generate_rotations():
                    for base in other.beacons: # [0,0,0] aligned and orientated
                        for pos_match in variation:
                            dx = base[0] - pos_match[0]
                            dy = base[1] - pos_match[1]
                            dz = base[2] - pos_match[2]
                            
                            matches = []
                            for to_check in variation:
                                moved = (to_check[0] + dx, to_check[1] + dy, to_check[2] + dz)
                                if abs(moved[0] - other.offset[0]) > 1000 or abs(moved[1] - other.offset[1]) > 1000 or abs(moved[2] - other.offset[2]) > 1000:
                                    continue
                                if moved in other.beacons:
                                    matches.append(moved)
                                else:
                                    break
                            if len(matches) >= 12:
                                scanner.beacons = set([(beacon[0] + dx, beacon[1] + dy, beacon[2] + dz) for beacon in variation])
                                scanner.offset = (dx, dy, dz)
                                print(f"Found scanner {scanner.id} at " + str(scanner.offset))
                                break
                        if scanner.offset is not None:
                            break
                    if scanner.offset is not None:
                        break
                if scanner.offset is not None:
                    break
    unlocalized = [scanner for scanner in scanners if scanner.offset is None]
    print(len(unlocalized), " left to position")
    

beacons = set()
for scanner in scanners:
    for beacon in scanner.beacons:
        beacons.add(beacon)    
print(len(beacons))


dis = 0
for a in scanners:
    for b in scanners:
        d = abs(a.offset[0] - b.offset[0]) + abs(a.offset[1] -b.offset[1]) + abs(a.offset[2] - b.offset[2])
        if d > dis:
            dis = d
            
print(dis)