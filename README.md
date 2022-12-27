# Genius Invokation TCG

<div align="center">
    <a href="https://crates.io/crates/genius-invokation">
        <img src="https://img.shields.io/crates/v/genius-invokation" alt="crates.io">
    </a>
</div>

This is a simple crate with an abstraction for the elements in Genius Invokation TCG, the card
game from Genshin Impact.

## Usage

### Creating a `Deck`

You can use the public enums to create `CharacterCard`s and `ActionCard`s (hint: All the action
card subtypes implement `Into<ActionCard>`). From those, you can convert them into a `Card` type
(which is an enum wrapper around those two main types), and create a `Deck` using `from_iter`.

Of course, if you somehow already have 3 character cards and 30 action cards exactly, you can
use the `from_exact` method, which is faster than the iterator method, since it skips allocation
and just verifies important properties (such as not having duplicate character cards or having
talent cards without its matching character card).

Alternatively, if you've already built a deck in the deck builder website, you can enable the
**deck-url** feature, which gives you access to the `deck_from_url` function, which receives a
url string that looks like this:

```
https://genshin.hotgames.gg/tcg/deck-builder?deck=1.6.MC.MD.MF.MG.MH.MI.MJ.MK.ML.MM.MN.MO.MP.MV.MY.e.g8.gB.gD.gF.gb.ge.gh.gk.gt.gv.gx.gz.wj.wl.wm&ver=1&lang=en&author=DefaultDeck
```

That function returns a card iterator, which you can then use to create a deck with `from_iter`.

### Extracting info

Now, let's say you wish to analyze some information in a huge list of decks. Assuming you have
all the deck builder urls in a file, line by line, you may [include it](https://doc.rust-lang.org/std/macro.include_str.html)
or simply [read to string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html), and once
that's done, it's as simple as

```rs
    let iter = decks.lines()
        .map(|url| Deck::from_iter(deck_from_url(url).unwrap()));
```

Of course, that will yield results, so you can also unwrap it if you're 100% sure none of those
decks are invalid.

Let's say you want to count how many decks include a specific card, for example,
[`Strategize`](https://genshin-impact.fandom.com/wiki/Strategize), you could simply run

```rs
    iter.filter(|deck| deck.contains(NormalEventCard::Strategize.into())).count()
```

Now let's say you only want to look at the companion cards, maybe you wish to count them up and
see which are the most prominent:

```rs
    iter.map(|deck| deck.iter().filter_map(|card| card.companion())).flatten()
```

You now have an `Iterator<Item=CompanionCard>`. Do note that this includes duplicate cards, so if
a player has two of the same card, the iterator will yield that card twice. If you want to only
look at the unique instances of a card, run `deck.iter_unique()` instead.

Of course, `companion()` isn't the only method available, it's only one among all the other
helper methods in the `PlayingCard` trait.

## *Work in progress!*

So far, I still have to figure a lot of things out, such as:

1. Having a nice implementation for converting between strings and enum variants. I could
make them manually matching every single variant, but that's boring and tricky to maintain.

2. Serde implementations, especially if APIs wish to use this crate in the future.

3. Figuring out how to make documentation for everything. Everytime I try to make documentation
for something like the `CharacterCard` enum, I just see myself being either redundant or trying
to explain how the game works, both of which are not what documentation should be.

4. Figuring out if I should include "special" kinds of cards that cannot be in the deck. This
includes cards such as the Abyss Mage or Hilichurl cards, but also the Keqing special card
[Lightning Stiletto](https://genshin-impact.fandom.com/wiki/Lightning_Stiletto), which can be
part of your hand during the game, but cannot be added to your actual deck.

5. Maybe including a `description` feature that adds a description method for each card? This
would reeaally increase the size of the crate (since it's a lot of text stored statically), but
maybe a useful feature to add.

6. Adding more items to this list.