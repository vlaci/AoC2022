<div>

<div>

# [Advent of Code](https://adventofcode.com/)

- [\[About\]](https://adventofcode.com/2022/about)
- [\[Events\]](https://adventofcode.com/2022/events)
- <a href="https://teespring.com/stores/advent-of-code" target="_blank"
  rel="noopener">[Shop]</a>
- [\[Settings\]](https://adventofcode.com/2022/settings)
- [\[Log Out\]](https://adventofcode.com/2022/auth/logout)

<div class="user">

vlaci <span class="star-count">28\*</span>

</div>

</div>

<div>

#        <span class="title-event-wrap">λy.</span>[2022](https://adventofcode.com/2022)<span class="title-event-wrap"></span>

- [\[Calendar\]](https://adventofcode.com/2022)
- [\[AoC++\]](https://adventofcode.com/2022/support)
- [\[Sponsors\]](https://adventofcode.com/2022/sponsors)
- [\[Leaderboard\]](https://adventofcode.com/2022/leaderboard)
- [\[Stats\]](https://adventofcode.com/2022/stats)

</div>

</div>

<div id="sidebar">

<div id="sponsor">

<div class="quiet">

Our [sponsors](https://adventofcode.com/2022/sponsors) help make Advent
of Code possible:

</div>

<div class="sponsor">

<a href="https://careers.king.com/" target="_blank"
onclick="if(ga)ga(&#39;send&#39;,&#39;event&#39;,&#39;sponsor&#39;,&#39;sidebar&#39;,this.href);"
rel="noopener">King</a> - At King, we create unforgettable games (like
Candy Crush) that are loved around the world. Join us to bring moments
of magic to hundreds of millions of people every single day!

</div>

</div>

</div>

<div role="main">

## --- Day 14: Regolith Reservoir ---

The distress signal leads you to a giant waterfall! Actually, hang on -
the signal seems like it's coming from the waterfall itself, and that
doesn't make any sense. However, you do notice a little path that leads
_behind_ the waterfall.

Correction: the distress signal leads you behind a giant waterfall!
There seems to be a large cave system here, and the signal definitely
leads further inside.

As you begin to make your way deeper underground, you feel the ground
rumble for a moment. Sand begins pouring into the cave! If you don't
quickly figure out where the sand is going, you could quickly become
trapped!

Fortunately, your [familiarity](https://adventofcode.com/2018/day/17)
with analyzing the path of falling material will come in handy here. You
scan a two-dimensional vertical slice of the cave above you (your puzzle
input) and discover that it is mostly _air_ with structures made of
_rock_.

Your scan traces the path of each solid rock structure and reports the
`x,y` coordinates that form the shape of the path, where `x` represents
distance to the right and `y` represents distance down. Each path
appears as a single line of text in your scan. After the first point of
each path, each point indicates the end of a straight horizontal or
vertical line to be drawn from the previous point. For example:

    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9

This scan means that there are two paths of rock; the first path
consists of two straight lines, and the second path consists of three
straight lines. (Specifically, the first path consists of a line of rock
from `498,4` through `498,6` and another line of rock from `498,6`
through `496,6`.)

The sand is pouring into the cave from point `500,0`.

Drawing rock as `#`, air as `.`, and the source of the sand as `+`, this
becomes:

      4     5  5
      9     0  0
      4     0  3
    0 ......+...
    1 ..........
    2 ..........
    3 ..........
    4 ....#...##
    5 ....#...#.
    6 ..###...#.
    7 ........#.
    8 ........#.
    9 #########.

Sand is produced _one unit at a time_, and the next unit of sand is not
produced until the previous unit of sand _comes to rest_. A unit of sand
is large enough to fill one tile of air in your scan.

A unit of sand always falls _down one step_ if possible. If the tile
immediately below is blocked (by rock or sand), the unit of sand
attempts to instead move diagonally _one step down and to the left_. If
that tile is blocked, the unit of sand attempts to instead move
diagonally _one step down and to the right_. Sand keeps moving as long
as it is able to do so, at each step trying to move down, then
down-left, then down-right. If all three possible destinations are
blocked, the unit of sand _comes to rest_ and no longer moves, at which
point the next unit of sand is created back at the source.

So, drawing sand that has come to rest as `o`, the first unit of sand
simply falls straight down and then stops:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    ......o.#.
    #########.

The second unit of sand then falls straight down, lands on the first
one, and then comes to rest to its left:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ........#.
    .....oo.#.
    #########.

After a total of five units of sand have come to rest, they form this
pattern:

    ......+...
    ..........
    ..........
    ..........
    ....#...##
    ....#...#.
    ..###...#.
    ......o.#.
    ....oooo#.
    #########.

After a total of 22 units of sand:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ....#ooo#.
    ..###ooo#.
    ....oooo#.
    ...ooooo#.
    #########.

Finally, only two more units of sand can possibly come to rest:

    ......+...
    ..........
    ......o...
    .....ooo..
    ....#ooo##
    ...o#ooo#.
    ..###ooo#.
    ....oooo#.
    .o.ooooo#.
    #########.

Once all `24` units of sand shown above have come to rest, all further
sand flows out the bottom, falling into the endless void. Just for fun,
the path any new sand takes before falling forever is shown here with
`~`:

    .......+...
    .......~...
    ......~o...
    .....~ooo..
    ....~#ooo##
    ...~o#ooo#.
    ..~###ooo#.
    ..~..oooo#.
    .~o.ooooo#.
    ~#########.
    ~..........
    ~..........
    ~..........

Using your scan, simulate the falling sand. _How many units of sand come
to rest before sand starts flowing into the abyss below?_

Your puzzle answer was `897`.

## --- Part Two ---

You realize you misread the scan. There isn't an <span
title="Endless Void is my C cover band.">endless void</span> at the
bottom of the scan - there's floor, and you're standing on it!

You don't have time to scan the floor, so assume the floor is an
infinite horizontal line with a `y` coordinate equal to _two plus the
highest `y` coordinate_ of any point in your scan.

In the example above, the highest `y` coordinate of any point is `9`,
and so the floor is at `y=11`. (This is as if your scan contained one
extra rock path like `-infinity,11 -> infinity,11`.) With the added
floor, the example above now looks like this:

            ...........+........
            ....................
            ....................
            ....................
            .........#...##.....
            .........#...#......
            .......###...#......
            .............#......
            .............#......
            .....#########......
            ....................
    <-- etc #################### etc -->

To find somewhere safe to stand, you'll need to simulate falling sand
until a unit of sand comes to rest at `500,0`, blocking the source
entirely and stopping the flow of sand into the cave. In the example
above, the situation finally looks like this after `93` units of sand
come to rest:

    ............o............
    ...........ooo...........
    ..........ooooo..........
    .........ooooooo.........
    ........oo#ooo##o........
    .......ooo#ooo#ooo.......
    ......oo###ooo#oooo......
    .....oooo.oooo#ooooo.....
    ....oooooooooo#oooooo....
    ...ooo#########ooooooo...
    ..ooooo.......ooooooooo..
    #########################

Using your scan, simulate the falling sand until the source of the sand
becomes blocked. _How many units of sand come to rest?_

Your puzzle answer was `26683`.

Both parts of this puzzle are complete! They provide two gold stars:
\*\*

At this point, you should [return to your Advent
calendar](https://adventofcode.com/2022) and try another puzzle.

If you still want to see it, you can
<a href="https://adventofcode.com/2022/day/14/input" target="_blank">get
your puzzle input</a>.

You can also <span class="share">\[Share<span class="share-content">on
<a
href="https://twitter.com/intent/tweet?text=I%27ve+completed+%22Regolith+Reservoir%22+%2D+Day+14+%2D+Advent+of+Code+2022&amp;url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F14&amp;related=ericwastl&amp;hashtags=AdventOfCode"
target="_blank" rel="noopener">Twitter</a> <a href="javascript:void(0);"
onclick="var mastodon_instance=prompt(&#39;Mastodon Instance / Server Name?&#39;); if(typeof mastodon_instance===&#39;string&#39; &amp;&amp; mastodon_instance.length){this.href=&#39;https://&#39;+mastodon_instance+&#39;/share?text=I%27ve+completed+%22Regolith+Reservoir%22+%2D+Day+14+%2D+Advent+of+Code+2022+%23AdventOfCode+https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F14&#39;}else{return false;}"
target="_blank" rel="noopener">Mastodon</a></span>\]</span> this puzzle.

</div>
