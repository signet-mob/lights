Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

turn on 0,0 through 999,999 would turn on (or leave on) every light.
toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
After following the instructions, how many lights are lit?

Function parameters are a range closed on both ends
Ranges create sub-rectanges, example in 13 turns off 499,499 499,500 500,499 500,500
row,col with origin at upper left

# ZOMBIES

## Zero case

- [x] No lights turned on
- [x] able to check if any light is on or off

## One case

- [x] able to turn one light on
- [x] able to turn one light off
- [x] able to toggle one light
- [ ] check (row, column) within boundaries
- [ ] what to do if (row, column) outside of boundaries?
  - [ ] return error, ok otherwise
- [ ] able to check how many lights are on

## Many case

- [ ] able to turn multiple lights on
- [ ] able to turn multiple lights off
- [ ] able to toggle multiple lights
- [ ] able to control lights in one row
- [ ] able to control lights in 2 dimensional range (row, column) pair
- [ ] what to do if (row, column) pair are reversed?
  - [ ] return error

# Retrospective

## What went well?

- requirements written first made it easier
- turns went smooth
- zombies approach works
- like how easily people asked what they didn't know
- did a good job of suggesting improvements
  - waited for gaps in conversation
- cool kata!

## What could be done better?

- not enough eggs
- continuously annoyed by VIM, can we automate settings based on driver?
- more detail on requirements vs asking while coding?
