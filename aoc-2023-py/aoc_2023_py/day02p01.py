
import pathlib

HERE = pathlib.Path(__file__).parent.absolute()

with open(HERE / ".." / "inputs" / "day02p01-example.txt") as fp:
    lines = fp.readlines()

    for line in lines:
        game_info, rounds_info = line.split(": ")

        game_id = int(game_info.split(" ")[-1])

        for round in rounds_info.split("; "):
            cube_amount, cube_color = round.split(", ")
