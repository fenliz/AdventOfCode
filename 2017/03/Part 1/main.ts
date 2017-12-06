var input: number = 368078;

var distanceFromMiddle: number = Math.ceil(Math.sqrt(input / 4) - 0.5);
var firstNumberOnThisIteration: number = Math.pow(1 + (distanceFromMiddle - 1) * 2, 2);
var lengthOfASideOnThisIteration: number = distanceFromMiddle * 2;

var howFarWasReachedOnTheSide: number = (input - firstNumberOnThisIteration) % lengthOfASideOnThisIteration;
var howFarOffCenter: number = Math.abs((lengthOfASideOnThisIteration / 2) - howFarWasReachedOnTheSide);

var manhattanDistance: number = distanceFromMiddle + howFarOffCenter;
console.log(manhattanDistance);