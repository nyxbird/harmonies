# Harmonies
Harmonies is a tile-laying board game designed by Johan Benvenuto and published by Libellud. This is a terminal-based implentation of the game's solitaire mode.

## Installation
Harmonies requires [Cargo](https://doc.rust-lang.org/cargo/):
```
cargo install --git https://github.com/nyxbird/harmonies
``` 
After installing, run with `harmonies`. (See `harmonies -h` for information about flags.)

## Instructions
(Full instructions for the game can be found on [Libellud's site](https://www.libellud.com/en/our-games/harmonies/))  

On each turn, you'll be presented with three pools of 3 tiles each, which may be `t`runks, `l`eaves, `r`ocks, `b`ricks, `f`ields, or `w`ater. You can select a pool by entering in a number (`1`, `2`, or `3`).  

Next to the pools is a row of 4 cards, each representing an animal. Once on each turn, before or after selecting your pool of tiles, you can take an animal card by typing its name (or any unambiguous prefix of it), or remove one from the row by prefacing or appending a valid prefix with `!`. Once you take one of these actions, you may not take another until you finish placing a the tiles in your next pool.  

After selecting a pool, tiles can be placed onto your board by entering commands of the form `ta1`, consisting of the tile type (`t/l/r/b/f/w`), and its coordinates as indicated by the row and column above and to the left of the board. Tiles can be played onto empty spaces or on top of specific other tiles: `t` can be placed on `-t`; `l` on `-t` and `=t`; `r` on `-r` and `=r`; and `b` on `-t`, `-r`, and `-b`. `-`, `=`, and `≡` represent stacks of tiles of heights 1, 2, and 3, respectively.  

At any time, you may score points by matching the patterns on your animal cards, ignoring rotation. (i.e. a straight horizontal line on an animal card may be matched by a line in any direction on the board, as long as the tiles match.) To do so, enter the name (or a prefix) of the card's name, followed by the coordinates matching the tile on your board that corresponds to the capitalized letter in the pattern. This will score points equal to the leftmost space in the card's scoring row, and place a cube on the board in the capital's location, preventing the placement of another cube there. Note that capitalized letters may match lowercase letters in future patterns.  

You may only have up to 4 animal cards at any given time, and animal cards are removed only when their scoring spaces are all used. If, at the beginning of your turn, there are 2 or fewer empty spaces on your board, the game will end. You will be allowed to finish taking card actions, but you may not take tiles. Aim to maximize your score and ranking.  

At the end of the game, placed tiles will score points as follows:
- `t`runks score nothing on their own.
- `l`eaves score points based on their height (1 for 1, 3 for 2, 7 for 3).
- `r`ocks score points based on their height only if adjacent to other rocks (1 for 1, 3 for 2, 7 for 3).
- `b`ricks score 5 points if they are both at height 2 and adjacent to tiles of 3 different colors. Empty spaces do not count for this.
- Each group of 2 or more adjacent `f`ields will score 5 points.
- `w`ater's scoring depends on the map played on:
  - On the A side map (5x5), the longest shortest path between connected `w`ater tiles will be scored based on its length (0/2/5/8/11/15 for 1-6, plus an additional 4 for following tiles. see the provided rulebook for examples).
  - On the B side map (7x4), each group of spaces separated by water tiles will score 5 points.
During and after the game, your score will be given a rank from `F` to `H+` based on your score, map, and choice of spirits (see below).
You may end the game at any time by entering `q`.

## Expansions
### Spirit Animals
(Running the program with the -s flag grants access to spirit animals.)  

At the beginning of the game, you will be presented with two potential spirit animals. Enter a prefix of the name of one to select it. Spirits act like normal animal cards, but have more complicated patterns, and reward you with an additional scoring condition rather than a fixed point count.

| Name | Abbreviation | Condition |
| ---- | ------------ | --------- |
| Bttrfly | `f+:5` | Each group of `f`ield scores 5 |
| Cat | `=b+:4` | Each group of `b`uildings (2-high `b`ricks) scores 4 |
| Deer | `=/≡l:4` | Each `l`eaf of height 2 or 3 scores 4 |
| Drgnfly | `ww+:7` | Each group of 2 or more `w`ater tiles scores 7 |
| Goat | `=/≡r:4` | Each `r`ock of height 2 or 3 scores 4 |
| Lion | `f+:2,2,10` | Each group of `f`ields score 2. Groups of 3 or more score 10 instead |
| Marmot | `?r:3,3,1` | `r`ocks score based on height: 3 for 1 or 2, 1 for 3 |
| Stork | `=bb+:6` | Each group of 2 or more `b`uildings scores 6 |
| Turtle | `w:2` | Each `w`ater tiles scores 2 |
| Owl | `?l:3,3,1` | `l`eaves score based on height: 3 for 1 or 2, 1 for 3 |

### Pulse
(Running the program with the -p flag grants access to pulse cards.)  

This expansion adds 10 more (non-spirit) animal cards to the available pool. These cards each feature three colors of tiles, allowing for more complex patterns. 

