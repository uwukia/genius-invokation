# Genius Invokation TCG

<div align="center">
    <a href="https://crates.io/crates/genius-invokation">
        <img src="https://img.shields.io/crates/v/genius-invokation" alt="crates.io">
    </a>
</div>

This is a simple crate with an abstraction for the elements in Genius Invokation TCG, the card
game from Genshin Impact.

*Work in progress!*

So far, I still have to figure a lot of things out, such as:

1. Having a nice implementation for converting between strings and enum variants. I could
make them manually matching every single variant, but that's boring and tricky to maintain.

2. Serde implementations, especially if APIs wish to use this crate in the future.

3. Figuring out how to make documentation for everything. Everytime I try to make documentation
for something like the `CharacterCard` enum, I just see myself being either redundant or trying
to explain how the game works, both of which are not what documentation should be.

4. Put some actual effort into making a nicer code for `deck_url`. Cause right now, it is
absolutely sucky.

5. Figuring out if I should include "special" kinds of cards that cannot be in the deck. This
includes cards such as the Abyss Mage or Hilichurl cards, but also the Keqing special card
[Lightning Stiletto](https://genshin-impact.fandom.com/wiki/Lightning_Stiletto), which can be
part of your hand during the game, but cannot be added to your actual deck.

6. Adding more items to this list.