# pylint: disable=missing-module-docstring,missing-class-docstring,missing-function-docstring

from dataclasses import dataclass
from enum import IntEnum
from queue import PriorityQueue


def read_input() -> list[list[int]]:
    with open("input/day17.txt", encoding="utf-8") as f:
        return [[int(c) for c in line.strip()] for line in f.readlines()]


class Direction(IntEnum):
    UP = 0
    RIGHT = 1
    DOWN = 2
    LEFT = 3

    def turn_left(self) -> "Direction":
        return Direction((self.value - 1) % 4)

    def turn_right(self) -> "Direction":
        return Direction((self.value + 1) % 4)

    def mv(
        self, rows: int, cols: int, row: int, col: int, length: int
    ) -> tuple[int, int] | None:
        match self:
            case Direction.UP:
                if row >= length:
                    return row - length, col
            case Direction.RIGHT:
                if col + length < cols:
                    return row, col + length
            case Direction.DOWN:
                if row + length < rows:
                    return row + length, col
            case Direction.LEFT:
                if col >= length:
                    return row, col - length
        return None


@dataclass(order=True)
class State:
    row: int
    col: int
    direction: Direction
    path_cost: int


def mv(
    in_map: list[list[int]], min_moves: int, max_moves: int, state: State
) -> list[State]:
    next_states: list[State] = []
    rows = len(in_map)
    cols = len(in_map[0])
    left = state.direction.turn_left()
    right = state.direction.turn_right()
    left_cost = state.path_cost
    right_cost = state.path_cost
    for length in range(1, max_moves + 1):
        mv_left = left.mv(rows, cols, state.row, state.col, length)
        mv_right = right.mv(rows, cols, state.row, state.col, length)
        if mv_left is not None:
            next_row, next_col = mv_left
            left_cost += in_map[next_row][next_col]
            if length >= min_moves:
                next_states.append(State(next_row, next_col, left, left_cost))
        if mv_right is not None:
            next_row, next_col = mv_right
            right_cost += in_map[next_row][next_col]
            if length >= min_moves:
                next_states.append(State(next_row, next_col, right, right_cost))

    return next_states


def calculate_priority(heuristic_map: list[list[int]], state: State) -> int:
    return state.path_cost + heuristic_map[state.row][state.col]


def hash_position(in_map: list[list[int]], state: State) -> int:
    return (state.row * len(in_map[0]) + state.col) * 4 + state.direction.value


def precalculate_heuristic(in_map: list[list[int]]) -> list[list[int]]:
    rows = len(in_map)
    cols = len(in_map[0])
    to_process: PriorityQueue[(int, int, int)] = PriorityQueue()
    to_process.put((0, rows - 1, cols - 1))
    results: list[list[int | None]] = [[None for _ in range(cols)] for _ in range(rows)]
    to_find = len(results * len(results[0]))
    while to_find > 0:
        (cost, row, col) = to_process.get()
        if results[row][col] is not None:
            continue
        results[row][col] = cost
        to_find -= 1
        next_cost = cost + in_map[row][col]
        next_states = [
            (next_cost, row + r, col + c)
            for (r, c) in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            if row + r >= 0 and row + r < rows and col + c >= 0 and col + c < cols
        ]
        for next_state in next_states:
            if results[next_state[1]][next_state[2]] is None:
                to_process.put(next_state)
    assert all(result is not None for row in results for result in row)
    return results


def run(in_map: list[list[int]], min_moves: int, max_moves: int) -> int:
    heuristic_map = precalculate_heuristic(in_map)
    initial_states = [
        State(0, 0, direction, 0) for direction in [Direction.RIGHT, Direction.DOWN]
    ]
    been = [False for _ in range(len(in_map) * len(in_map[0]) * 4)]
    to_process: PriorityQueue[(int, State)] = PriorityQueue()
    for s in initial_states:
        to_process.put((calculate_priority(heuristic_map, s), s))

    while True:
        (_, state) = to_process.get()
        if state.row == len(in_map) - 1 and state.col == len(in_map[0]) - 1:
            return state.path_cost
        if been[hash_position(in_map, state)]:
            continue
        been[hash_position(in_map, state)] = True
        next_states = mv(in_map, min_moves, max_moves, state)
        for next_state in next_states:
            if not been[hash_position(in_map, next_state)]:
                to_process.put(
                    (calculate_priority(heuristic_map, next_state), next_state)
                )


def main():
    in_map = read_input()
    part_1 = run(in_map, 1, 3)
    print("Part 1:", part_1)
    part_2 = part_2 = run(in_map, 4, 10)
    print("Part 2:", part_2)


if __name__ == "__main__":
    main()
