# Baghchal

A Rust implementation of Baghchal, a traditional board game from Nepal. The game is played between two players: one controlling four tigers and the other controlling twenty goats.

## Game Rules

- The game is played on a 5×5 grid board
- Tigers start in the four corners
- Goats are placed one by one (20 in total)
- Tigers can move to adjacent points and capture goats by jumping over them
- Tigers win by capturing 5 goats
- Goats win by blocking all tiger moves

## Features

- Colored terminal display (tigers in red, goats in yellow)
- Move validation
- Diagonal movement restrictions on valid intersections
- Goat capture mechanics
- Quit functionality ('q' or 'quit')

## Building and Running

Requires Rust and Cargo.
