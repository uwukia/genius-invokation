# Genius Invokation TCG

This is a simple crate with an abstraction for the elements in Genius Invokation TCG, the card
game from Genshin Impact.

*Work in progress!*

So far, I still have to figure a lot of things out, such as:

1. Having a nice implementation for converting between strings and enum variants. I could
make them manually matching every single variant, but that's boring and tricky to maintain. 

2. Serde implementations, especially if APIs wish to use this crate in the future.

3. Obviously, actually writing all the enum variants of action cards, and creating all the other
types of info for cards, such as dice cost (energy cost when present), etc.

4. Adding more items to this list.