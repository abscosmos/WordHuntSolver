# Word Hunt Solver
A lightweight and fast command-line solver for the iMessage GamePigeon Word Hunt game, built with Rust.  

## Word Hunt?
In Word Hunt, both players are given two identical 4x4 boards of letters, and must create as many words as possible within 90 seconds.
Letters can be connected horizonally, vertially, or diagonally, and words have to be at least three letters long.
A word's score is determined solely by its length, with longer words earning more points.
The player with the most points at the end of the game wins. The solver doesn't currently support maps 2-4, but it should be simple to implement.

## Usage
1. Download the latest release from [releases/latest](../../releases/latest), or download the source code & compile using `cargo b -r`.
2. Run the executable, then start a Word Hunt game and copy the letters from the board in the following format:

Board:
```
a b c d
e f g h
i j k l
m n o p
```

Input:
```
abcdefghijklmnop
```
---
3. The solver will display the first solution. Start with the double arrow, which denotes the first letter, and follow the arrows until you reach `⊗`.

For example, to find the word "inroads" on the following board:
```
c a t d
w o f o
i r d g
n o a s
```
The solver will display the following:
```
* * * *
* * * *
⇓ ↓ ↘ *
↗ → ↑ ⊗
```
Start with the `⇓`, and follow the arrows until you reach the `⊗`, in this order:
```
* * * *
* * * *
1 3 6 *
2 4 5 7
```
---
4. To see the next word, press `Enter`, or if you've run out of time and would like to restart, enter `new`. To stop the program, enter `stop`.

## How It Works
The program loads in a list of words from file into a HashSet, where they can be filtered to only keep words with 2-14 letters and no characters outside A->Z.
A GameBoard is then created from the input string, and HashSet of all words is further filtered to only keep words that can be built from the set of letters in the game board.
The HashSet is then converted into a prefix tree, and the GameBoard recursively searches all possible paths, stopping early if the current path isn't a prefix of any word in the tree.
Duplicate paths to the same word are filtered out, and the results are displayed the user.

## Speed
As per [v1.0.0](../../releases/v1.0.0), on my machine, over 10,000 tests, it took an average of 34.2ms and 24.9mb per solve, on a AMD R9 7950X.
Improving the recursive method & how the word list is cloned could speed it up and use less memory, but for now I'm satisfied with its performance.

## Dictionary
- Collins Scrabble Words from https://github.com/jesstess/Scrabble
