/* eslint-disable camelcase */
const time = `G'DAY MATE!

IMPOHT ME FUNC GimmeTime;

GIMME "the time in sydney is: " + GimmeTime();
`

const fibonacci = `G'DAY MATE!

THE HARD YAKKA FOR fibonacci IS ( x ) <
    YA RECKON x <= 1 ? BAIL x;

	BAIL fibonacci(x - 1) + fibonacci(x - 2);
>

GIMME fibonacci(10);

CHEERS C***!`

const dreamtime = `WHATS THE CRAIC?

GIVE'IS A HitTheSack;
GIVE'IS A TakeAPunt;

do me a wee favor dreamtime IS () <
	NO WORRIES "gonna go for a kip here";

	I RECKON I'LL HAVE A DANDER UNTIL (nope) <
	    NO WORRIES, ITS "zZz...";

		HitTheSack(1000);

		YA RECKON TakeAPunt(0, 6) == 0 ? LOADA BALLS
	>

	NO WORRIES  "that wee nap was a belter mate!";
>

dreamtime();

LATER MATE

`

const random_beer = `G'DAY MATE!

IMPOHT ME FUNC TakeAPunt;

THE HARD YAKKA FOR randomBeer IS () <
	YA RECKON TakeAPunt(0, 3) IS A <
		0 ~ BAIL "Fosters";
		1 ~ BAIL "Coopers";
		2 ~ BAIL "Pilsner";
	>
>

YA RECKON randomBeer() IS A <
	"Fosters"    ~ GIMME "Flamin' hell!";
	"Coopers"    ~ GIMME "You Beauty!";
	somethinElse ~ GIMME "Yeah, dunno that one: " + somethinElse;
>

CHEERS C***!`

export const examples: Record<string, string> = {
  fibonacci,
  dreamtime,
  time,
  random_beer
}
