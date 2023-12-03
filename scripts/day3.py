import pathlib
import sys
import math

def get_file_content(path: pathlib.Path):
    with open(path, "r") as f:
        data = f.readlines()
    return [line.strip("\n") for line in data]

def problem_one(path: pathlib.Path):
    content: list[str] = get_file_content(path)
    coors: list[tuple[int,int]] = [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
    valid: list[str] = []
    schem = False

    for y, line in enumerate(content):
        tmp = ""
        for x, char in enumerate(line):
            if char == '.':
                continue
            else:
                if char.isdigit():
                    tmp += char
                    for coor in coors:
                        try:
                            result = content[y + coor[0]][x + coor[1]]
                            if (result != '.') and (not result.isdigit()):
                                # print(f"valid num: {char}")
                                schem = True
                                break
                        except:
                            continue
            if x != len(line) - 1: 
                if not line[x + 1].isdigit():
                    if schem:
                        valid.append(tmp)
                    tmp = ""
                    schem = False
            else:
                if schem:
                    valid.append(tmp)
                    schem = False

    final_sum = sum(list(map(lambda x: int(x), valid)))
    print(final_sum)

def problem_two(path: pathlib.Path):
    content: list[str] = get_file_content(path)
    coors: list[tuple[int,int]] = [(-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1)];
    valid: list[int] = []

    for y, line in enumerate(content):
        for x, char in enumerate(line):
            if char == '*':
                arr: set[str] = set([])
                for coor in coors:
                    try:
                        nyidx, nxidx = y + coor[0], x + coor[1]
                        result = content[nyidx][nxidx]
                        if result.isdigit():
                            tmp = result
                            sidx = nxidx - 1
                            if (sidx > 0) and (sidx <= len(line) - 1):
                                while content[nyidx][sidx].isdigit():
                                    tmp = content[nyidx][sidx] + tmp
                                    sidx -= 1
                                    if (sidx < 0) or (sidx > len(line) - 1):
                                        break
                            sidx = nxidx + 1
                            if (sidx > 0) and (sidx <= len(line) - 1):
                                while content[nyidx][sidx].isdigit():
                                    tmp = tmp + content[nyidx][sidx]
                                    sidx += 1
                                    if (sidx < 0) or (sidx > len(line) - 1):
                                        break
                                arr.add(tmp)
                    except:
                        continue
                if len(arr) == 2:
                    value = math.prod(list(map(lambda x: int(x), arr)))
                    valid.append(value)

    final_sum = sum(list(map(lambda x: int(x), valid)))
    print(final_sum)

if __name__ == "__main__":
    path_str = sys.argv[1]
    # problem_one(pathlib.Path(path_str))
    problem_two(pathlib.Path(path_str))

