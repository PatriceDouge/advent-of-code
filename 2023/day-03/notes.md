given this example input: 
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

return sum of nums that are adjacent to any symbol, either on the same line or diagonal. 

for the given example, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

first we're going to turn the input into a 2d array,

[-1,0]
[-1,-1]
[-1,1]
[1,0]
[1,-1]
[1,1]
[0,1]
[0,-1]

