# WORDFIGHT

WORDFIGHT is a simple text-based "fighting" game. The only mechanic is spelling.

## Gameplay

Two players battle by typing any substring of a valid English word into the shared input space of a fixed size (for the time being, that size is 7 characters). Each player's word extends from one "side" of the input, and both perspectives are shown to both players.

When there are no empty spaces, the words "strike" each other and a point is awarded based on which of the two striking letters appears later in the alphabet. Both words are cleared and play resumes.

If both players make an input on the same frame, both attempting to occupy the last empty "striking" spot, both players' words are cleared and no points are awarded.

Players cannot type non-word inputs, but this doesn't mean the player has to finish typing the word. This allows for a form of "footsies": a player can type "pa" and decide whether to continue with "paltry" (with strong letters in the 4,5,6 positions) or "patro(-nize)" (with strong letters in the 3,4,5 positions). Note that in a 7-size "arena", if two players have already typed "pa", a few interactions can occur based on how players react (assuming these are the only two words players are choosing between):

- "patro" beats "pa" (O > A)
- "patr" loses to "pat" (R < T)
- "patr" beats "pal" (R > L)
- "paltry" beats "p" (Y > P) (if player 2 tries backspace)

So the game will generally be played around figuring out what words your opponent has in mind to play and trying to evade or counter them.

## Design

This is an attempt at creating an abstract/minimalist game that echoes the game design of [traditional 2D fighting games](https://en.wikipedia.org/wiki/Fighting_game#Game_design). In a similar way to those games, players are rewarded for doing research to find various ways of pressuring and attacking opponents, as well as creating spacing and timing traps that can punish enemy players for performing a predicted behavior. Feel free to scour the word list for combos to get the upper hand -- and let us know your favorites!

The choice to award points based on position in the alphabet is somewhat arbitary. Although we could instead score based on letter rarity or some other comparison, alphabet position is generally easiest to understand. We may explore variants in the future depending on interest.

## Tech Stack

This is built using Rust, with Bevy running the game loop and using Leptos/Trunk for a frontend.
