from collections import deque, namedtuple

Step = namedtuple("Step", ["x", "y", "direction"])


def move(step, direction):
    match direction:
        case "r": return Step(step.x+1, step.y, "r")
        case "d": return Step(step.x, step.y+1, "d")
        case "l": return Step(step.x-1, step.y, "l")
        case "u": return Step(step.x, step.y-1, "u")
        case _: raise ValueError("Invalid direction {direction}")


def turn(direction, mirror):
    match mirror:
        case '/':
            match direction:
                case "r": return "u"
                case "d": return "l"
                case "l": return "d"
                case "u": return "r"
                case _: raise ValueError("Invalid direction {direction}")

        case '\\':
            match direction:
                case "r": return "d"
                case "d": return "r"
                case "l": return "u"
                case "u": return "l"
                case _: raise ValueError("Invalid direction {direction}")

        case _: raise ValueError("Invalid mirror symbol {mirror}")


def explore(startx, starty, direction):
    queue = deque()

    queue.append(Step(startx, starty, direction))

    visited = set()

    while queue:
        step = queue.popleft()
        if step.x < 0 or step.y < 0 or step.x >= sx or step.y >= sy:
            continue

        if (step.x, step.y, step.direction) in visited:
            continue

        visited.add((step.x, step.y, step.direction))

        tile = field[step.y][step.x]
        match tile:
            case '.':
                queue.append(move(step, step.direction))

            case '\\' | "/":
                direction = turn(step.direction, tile)
                step1 = move(step, direction)
                queue.append(step1)

            case "-":
                if step.direction in {'l', 'r'}:
                    queue.append(move(step, step.direction))
                else:
                    # split to two beam
                    queue.append(move(step, "l"))
                    queue.append(move(step, "r"))

            case "|":
                if step.direction in {'u', 'd'}:
                    queue.append(move(step, step.direction))
                else:
                    # split to two beam
                    queue.append(move(step, "u"))
                    queue.append(move(step, "d"))

            case _: raise ValueError(f"Invalid tile {tile}")

    return len({(x, y) for (x, y, d) in visited})


with open('..\\..\\input\\day16.txt', 'rt') as f:
    field = []
    for line in f:
        field.append(line.strip())

sx = len(field[0])
sy = len(field)




print(explore(0, 0, "r"))

max_energyze = 0
for y in range(0, sy):
    max_energyze = max(max_energyze, explore(0, y, "r"))
    max_energyze = max(max_energyze, explore(sx-1, y, "l"))

for x in range(0, sx):
    max_energyze = max(max_energyze, explore(x, 0, "d"))
    max_energyze = max(max_energyze, explore(x, sy-1, "u"))


print(max_energyze)