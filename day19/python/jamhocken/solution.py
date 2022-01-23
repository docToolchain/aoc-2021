import regex as re
import collections

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    scanners = dict()
    scanner_pattern = re.compile("(\d+)")
    beacon_pattern = re.compile("(-?\d+),(-?\d+),(-?\d+)")
    for line in lines_stripped:
        if len(line)>0 and line[1] == "-":
            scanner_match = re.search(scanner_pattern,line)
            scanner = int(scanner_match.group(0))
            beacons = list()
        elif len(line)>0:
            beacon_match = re.match(beacon_pattern,line)
            beacon = [int(beacon_match.group(i)) for i in [1,2,3]]
            beacons.append(beacon)
        else:
            scanners[scanner] = beacons
    
    scanners[scanner] = beacons
        
    return scanners

def find_distance(scanners):
    distance_dict = dict()
    for key,value in scanners.items():
        distances = dict()
        for i in range(len(value)):
            for j in range(i+1,len(value)):
                distances[(i,j)] = [abs(value[i][k]-value[j][k]) for k in [0,1,2]]
        distance_dict[key] = distances

    return distance_dict

def find_matches(mapped,unmapped,distances,scanners,scan_translation):
    new_unmapped = unmapped.copy()
    new_mapped = mapped.copy()
    for key_unmapped in unmapped:
        # find all distances between beacons that match for both scanners
        for key_mapped in mapped:
            matches = list()
            beacon_unmapped = set()
            beacon_mapped = set()
            for pair_mapped,distance_mapped in distances[key_mapped].items():
                for pair_unmapped,distance_unmapped in distances[key_unmapped].items():
                    if set(distance_mapped) == set(distance_unmapped):
                        matches.append([pair_mapped,pair_unmapped])
                        beacon_unmapped.update(pair_unmapped)
                        beacon_mapped.update(pair_mapped)
            #find the matching pairs of beacons
            beacon_mapping = dict()
            for beacon in beacon_mapped:
                other_beacon = set()
                for match in matches:
                    if beacon in match[0]:
                        if not other_beacon:
                            other_beacon = set(match[1])
                        else:
                            other_beacon = other_beacon.intersection(match[1])
                beacon_mapping[beacon] = other_beacon.pop()

            # If there are at least 12 beacon matches
            if len(beacon_mapping)>11:
                #First we sort out the x,y,z coordinates so that both scanners have the same definition
                flagx = sum([1 if distances[key_mapped][matches[j][0]][0] == distances[key_unmapped][matches[j][1]][0] else 0 for j in range(len(matches))])
                flagy = sum([1 if distances[key_mapped][matches[j][0]][0] == distances[key_unmapped][matches[j][1]][1] else 0 for j in range(len(matches))])
                flagz = sum([1 if distances[key_mapped][matches[j][0]][0] == distances[key_unmapped][matches[j][1]][2] else 0 for j in range(len(matches))])
                if flagy >= flagx and flagy >= flagz:
                    distances[key_unmapped] = {key:[value[1],value[0],value[2]] for key,value in distances[key_unmapped].items()}
                    scanners[key_unmapped] = [[value[1],value[0],value[2]] for value in scanners[key_unmapped]]
                elif flagz >= flagx and flagz >= flagy:
                    distances[key_unmapped] = {key:[value[2],value[1],value[0]] for key,value in distances[key_unmapped].items()}
                    scanners[key_unmapped] = [[value[2],value[1],value[0]] for value in scanners[key_unmapped]]
                flagy = sum([1 if distances[key_mapped][match[0]][1] == distances[key_unmapped][match[1]][1] else 0 for match in matches])
                flagz = sum([1 if distances[key_mapped][match[0]][1] == distances[key_unmapped][match[1]][2] else 0 for match in matches])                
                if flagz >= flagy:
                    distances[key_unmapped] = {key:[value[0],value[2],value[1]] for key,value in distances[key_unmapped].items()}
                    scanners[key_unmapped] = [[value[0],value[2],value[1]] for value in scanners[key_unmapped]]
                # Then we see if any x,y,z is inverted
                
                # First though we have to reorder our matching list to keep beacon pairs consistent
                temp_matches = list()
                for match in matches:
                    if match[1][0] != beacon_mapping[match[0][0]]:
                        temp_matches.append([match[0],(match[1][1],match[1][0])])
                    else:
                        temp_matches.append(match)
                matches = temp_matches
                invertx = sum([1 if scanners[key_mapped][match[0][0]][0] - scanners[key_mapped][match[0][1]][0] == scanners[key_unmapped][match[1][0]][0] - scanners[key_unmapped][match[1][1]][0] else -1 for match in matches])
                inverty = sum([1 if scanners[key_mapped][match[0][0]][1] - scanners[key_mapped][match[0][1]][1] == scanners[key_unmapped][match[1][0]][1] - scanners[key_unmapped][match[1][1]][1] else -1 for match in matches])
                invertz = sum([1 if scanners[key_mapped][match[0][0]][2] - scanners[key_mapped][match[0][1]][2] == scanners[key_unmapped][match[1][0]][2] - scanners[key_unmapped][match[1][1]][2] else -1 for match in matches])

                if invertx < 0:
                    scanners[key_unmapped] = [[-1*value[0],value[1],value[2]] for value in scanners[key_unmapped]]
                if inverty < 0:
                    scanners[key_unmapped] = [[value[0],-1*value[1],value[2]] for value in scanners[key_unmapped]]
                if invertz < 0:
                    scanners[key_unmapped] = [[value[0],value[1],-1*value[2]] for value in scanners[key_unmapped]]

                # And finally we shift the scanner to overlap with the other one
                translation = [(scanners[key_mapped][match[0][0]][0] - scanners[key_unmapped][match[1][0]][0],scanners[key_mapped][match[0][0]][1] - scanners[key_unmapped][match[1][0]][1],scanners[key_mapped][match[0][0]][2] - scanners[key_unmapped][match[1][0]][2]) for match in matches]
                translation = collections.Counter(translation).most_common(1)[0][0]

                scanners[key_unmapped] = [[value[0]+translation[0],value[1]+translation[1],value[2]+translation[2]] for value in scanners[key_unmapped]]
                
                if key_unmapped in new_unmapped:
                    new_unmapped.remove(key_unmapped)
                new_mapped.add(key_unmapped)
                scan_translation.append(translation)
    
    return new_mapped, new_unmapped

def main():
    with open("input.txt",'r') as beacon_file:
        beacon_lines = beacon_file.readlines()

    scanners = process_input(beacon_lines)
    distances = find_distance(scanners)

    keys = list(scanners.keys())    
    mapped = {keys[0]}
    unmapped = set(keys[1:])
    translation = list()
    
    while unmapped:
        (new_mapped, new_unmapped) = find_matches(mapped,unmapped,distances,scanners,translation)           
        mapped = new_mapped
        unmapped = new_unmapped
    
    # Star 1
    beacons = set()
    for key,value in scanners.items():
        for beacon in value:
            beacons.add(tuple(beacon))
    print(len(beacons))

    # Star 2
    max_distance = 0
    for i in range(len(translation)):
        for j in range(i+1,len(translation)):
            max_distance = max(max_distance,sum([abs(translation[i][k]-translation[j][k]) for k in [0,1,2]]))

    print(max_distance)        
main()
