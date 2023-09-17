#!/usr/bin/env python3

# python game that I decided to make in under 1 hour

import pygame
import time
import collections
import random

screen_width = 600
screen_height = 400

cell_size = 18
cell_gap = 2
tot_size = cell_size + cell_gap

grid_width = int(screen_width / tot_size)
grid_height = int(screen_height / tot_size)

direction = (1, 0)

delay = 0.200

apples = []

pos = (0, 0)
poses = collections.deque()
poses.append(pos)


def add_apple():
    global apples
    apple = (random.randrange(0, grid_width), random.randrange(0, grid_height))

    while apple in apples:
        apple = (random.randrange(0, grid_width), random.randrange(0, grid_height))

    apples.append(apple)


def draw_cell(color, pos, screen):
    screen.fill(
        color,
        pygame.Rect(
            cell_gap + pos[0] * tot_size,
            cell_gap + pos[1] * tot_size,
            cell_size,
            cell_size,
        ),
    )


def reset():
    global pos, poses, direction, delay
    delay = 0.2
    pos = (0, 0)
    poses.clear()
    poses.append(pos)
    direction = (1, 0)


def speed_up():
    global delay
    delay *= 0.98


def move(d):
    global direction, pos, poses, apples
    direction = d
    pos = (pos[0] + direction[0], pos[1] - direction[1])

    if pos in apples:
        apples.remove(pos)
        add_apple()
        speed_up()
    else:
        poses.popleft()
        if (
            pos in poses
            or pos[0] < 0
            or pos[1] < 0
            or pos[0] >= grid_width
            or pos[1] >= grid_height
        ):
            reset()
            return
    poses.append(pos)


def render(screen):
    screen.fill((0, 0, 0))

    for p in poses:
        draw_cell((0, 255, 0), p, screen)

    draw_cell((255, 255, 0), pos, screen)

    for apple in apples:
        draw_cell((255, 0, 0), apple, screen)


prev_time = time.time()


def main():
    global pos, poses, direction, prev_time
    pygame.init()

    pygame.display.set_caption("snake")

    screen = pygame.display.set_mode((screen_width, screen_height))

    running = True

    for _ in range(0, 10):
        add_apple()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if direction[0] == 0:
                    if event.key == pygame.K_LEFT:
                        direction = (-1, 0)
                        move(direction)
                        prev_time = time.time()
                        render(screen)
                    elif event.key == pygame.K_RIGHT:
                        direction = (1, 0)
                        move(direction)
                        render(screen)
                        prev_time = time.time()
                else:
                    if event.key == pygame.K_DOWN:
                        direction = (0, -1)
                        move(direction)
                        render(screen)
                        prev_time = time.time()
                    elif event.key == pygame.K_UP:
                        direction = (0, 1)
                        move(direction)
                        render(screen)
                        prev_time = time.time()
            if event.type == pygame.QUIT:
                running = False

        now = time.time()

        if now - prev_time > delay:
            prev_time += delay
            move(direction)
            render(screen)

        pygame.display.update()


if __name__ == "__main__":
    main()
