---
id: CONT-MISSIONS
title: Start missions — 10 animals
aspect: content
module: missions
status: draft
depends_on: [GAME-RESCUE, GAME-ANIMALS, GAME-FEED, CONT-READING, CONT-MATH, CONT-L10N]
test_prefix: MISS
updated: 2026-09-28
---

# Start missions — 10 animals

Content for the first 10 rescue missions (flow: GAME-RESCUE). German is the reference text;
English follows it. Animal selection is a **proposal** pending Q-002.

Per mission: **food box word** (exactly as printed on the box and used on the info board),
**hiding place** (location id for GAME-LAYOUT), **location riddle** per reading level, and
one optional **math task** (CONT-MATH).

Riddle rules (from CONT-READING / GAME-RESCUE):
- `kiga`: picture of the place + one word, read aloud on tap.
- `klasse1`: short sentences, ≤ 5 words each, never names the place.
- `klasse2`: 1–2 sentences, never names the place.
- `klasse3`: 3–5 sentences, describes the place through details, never names it.

Facts rules (GAME-ANIMALS "Info board" item 4): true and child-friendly, never a word of the
animal's hiding place; `kiga` one word + picture, `klasse1` 3 sentences of ≤ 5 words,
`klasse2` 3–4 sentences, `klasse3` 4–6 sentences. Keys `mission-<animal>-facts-<reading_level>`,
name `animal-<animal>`, heading above the facts `animal-<animal>-more`.

**Several hiding places per animal** (discovery, user decision 2026-09-26, GAME-RESCUE §1):
an animal has ≥ 3 candidate hiding places in its level; one is picked per playthrough and the
info board shows **the riddle of the picked place**. All 10 animals have 3 candidates:
level 1 (zebra, hippo, panda, GAME-LEVEL-1), level 2 (koala, elephant, giraffe, lion,
GAME-LEVEL-2) and level 3 (monkey, goldfish, snow fox, GAME-LEVEL-3) — places of levels 2–3
are proposals (Q-095). Additional rules for candidates:
- Each place has its own riddle for every reading level and language; keys
  `mission-<animal>-riddle-<hiding_place>-<reading_level>` with the full place id, e.g.
  `mission-zebra-riddle-loc_meadow-klasse1` (Behaviour 1).
- A riddle fits **only its own place** among the animal's candidates: its key detail (what
  the animal does + what is there) is shared with no other candidate of the same animal, and
  it contains no `kiga` place word of the animal's other candidates (MISS-007).
- Riddles describe the place, not the way there, so they stay true whichever places the
  other animals got.
- **Place words** (ANIM-007: facts never contain them; RESC-011: a riddle never contains the
  `kiga` word of its own place) are listed per place below. Two-word `kiga` labels (*bamboo
  forest*, *leaf pile*, as *ice cream*) are checked word by word (Q-083 answered).

## Overview

| # | Animal id | Food box (de / en) | Hiding place id | Place (for designers only — never shown in riddles ≥ klasse1) |
|---|---|---|---|---|
| 1 | `zebra` | Gras / grass | `loc_river`, `loc_meadow`, `loc_sand` | river with a bridge and ducks · tall-grass meadow with wildflowers and butterflies · dry yellow sand patch (dust bath) |
| 2 | `hippo` | Melonen / melons | `loc_pond`, `loc_mud`, `loc_shade` | still pond with water lilies and frogs · brown mud puddle · shade under big trees at the zoo wall |
| 3 | `panda` | Bambus / bamboo | `loc_cave`, `loc_bamboo`, `loc_leaves` | dark, cool stone cave · bamboo thicket taller than the wall · raked pile of red and yellow leaves |
| 4 | `koala` (pair) | Eukalyptus / eucalyptus | `loc_treehouse`, `loc_tallest_tree`, `loc_blossom_tree` (level 2) | tree house with rope ladder · the tallest tree of the zoo · tree with pink blossoms and bees |
| 5 | `elephant` | Heu / hay | `loc_fountain`, `loc_log_pile`, `loc_big_ball` (level 2) | stone fountain with water jet and coins · stacked logs, sawdust · giant red-and-white ball |
| 6 | `goldfish` | Fischfutter / fish food | `loc_waterfall`, `loc_water_wheel`, `loc_willow` (level 3, all in the stream; replaces `loc_fountain`) | waterfall with white foam · turning wooden water wheel · weeping willow over the water; needs the fish bowl (GAME-RESCUE "goldfish bowl") |
| 7 | `monkey` | Bananen / bananas | `loc_pirate_ship`, `loc_carousel`, `loc_trampoline` (level 3) | pirate ship climbing frame (mast, sail, black flag, treasure chest) · carousel with wooden horses · ground trampoline |
| 8 | `giraffe` | Blätter / leaves | `loc_lookout_tower`, `loc_train`, `loc_playground` (level 2) | wooden lookout tower · little zoo train with a bell · playground with slide and swings (no sandpit) |
| 9 | `lion` | Fleisch / meat | `loc_sun_rocks`, `loc_stage`, `loc_deckchairs` (level 2) | big flat rocks in full sun · round music stage with drums · striped deckchairs under a sunshade |
| 10 | `snow_fox` | Beeren / berries | `loc_ice_cream_kiosk`, `loc_sprinkler`, `loc_laundry` (level 3) | ice cream kiosk with freezer chest · lawn sprinkler with cold drops and a rainbow · washing line with white sheets |

The 10 food boxes together form the food storage; the other 9 boxes are the natural
distractors for each mission.

---

## 1. Zebra — `loc_river`, `loc_meadow`, `loc_sand`

Riddle — `loc_river` (keys `mission-zebra-riddle-loc_river-<reading_level>`; the PoC keys `mission-zebra-riddle-<reading_level>` were deleted 2026-09-26, M5a):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 river · **Fluss** | 🖼 river · **river** |
| klasse1 | Ich habe Durst. Ich suche fließendes Wasser. | I am thirsty. I look for running water. |
| klasse2 | Die Zebras haben großen Durst. Sie trinken dort, wo das Wasser fließt und Enten schwimmen. | The zebras are very thirsty. They drink where the water flows and ducks swim. |
| klasse3 | Die Zebras sind lange herumgerannt. Jetzt haben sie Durst. Sie suchen Wasser, das sich bewegt und rauscht. Dort, wo eine Brücke über das Wasser führt, trinken sie. | The zebras ran around for a long time. Now they are thirsty. They are looking for water that moves and rushes. They drink where a bridge crosses the water. |

(`klasse3` changed 2026-09-26: *über die Wiese* / *across the meadow* removed — the meadow is
now another zebra hiding place, `loc_meadow`.) Place words: *Fluss, Bach, Brücke, Enten,
Wasser* / *river, stream, bridge, ducks, water*.

Riddle — `loc_meadow` (keys `mission-zebra-riddle-loc_meadow-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 meadow with tall grass and flowers · **Wiese** | 🖼 meadow with tall grass and flowers · **meadow** |
| klasse1 | Ich habe Hunger. Das Gras ist hoch. Dort fliegen Schmetterlinge. | I am hungry. The grass is tall. Butterflies fly there. |
| klasse2 | Die Zebras haben Hunger. Sie fressen dort, wo das Gras hoch wächst, bunte Blumen blühen und Schmetterlinge fliegen. | The zebras are hungry. They eat where the grass grows tall, colourful flowers bloom and butterflies fly. |
| klasse3 | Die Zebras haben großen Hunger. Sie suchen einen Platz, an dem das Gras so hoch ist, dass es ihnen bis zum Bauch reicht. Zwischen den Halmen blühen bunte Blumen, und Schmetterlinge flattern umher. Hinter dem hohen Gras stehen große Bäume. | The zebras are very hungry. They are looking for a place where the grass is so tall that it reaches their bellies. Colourful flowers bloom between the blades, and butterflies flutter around. Big trees stand behind the tall grass. |

Place words: *Wiese, Blumen, Schmetterlinge* / *meadow, flowers, butterflies* (not *Gras* /
*grass* — that is the food word).

Riddle — `loc_sand` (keys `mission-zebra-riddle-loc_sand-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 yellow sand patch · **Sand** | 🖼 yellow sand patch · **sand** |
| klasse1 | Mein Fell juckt. Ich wälze mich gern. Der Boden ist gelb. | My fur itches. I like to roll. The ground is yellow. |
| klasse2 | Den Zebras juckt das Fell. Sie wälzen sich dort, wo der Boden gelb und trocken ist und kein Gras wächst. | The zebras' fur itches. They roll around where the ground is yellow and dry and no grass grows. |
| klasse3 | Zebras nehmen gern ein Staubbad. Dabei wälzen sie sich auf dem Rücken hin und her, bis ihr Fell ganz staubig ist. Das hilft gegen lästige Fliegen. Unsere Zebras haben einen Platz gefunden, an dem kein Grashalm wächst und der Boden gelb und warm ist. | Zebras like to take a dust bath. They roll on their backs until their fur is all dusty. It helps against annoying flies. Our zebras have found a place where not a single blade of grass grows and the ground is yellow and warm. |

Place words: *Sand, Staubbad* / *sand, dust*.

Animal name (`animal-zebra`, all reading levels): *Zebra* / *Zebra*. Heading above the facts
(`animal-zebra-more`): *Mehr über das Zebra* / *More about the zebra*.

**Facts** (*Steckbrief*, GAME-ANIMALS "Info board" item 4, keys `mission-zebra-facts-<reading_level>`;
shown after the riddle and the food word; never a place word — ANIM-007 checks the whole words *Fluss*,
*Brücke*, *Enten*, *Wasser* / *river*, *bridge*, *ducks*, *water*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 zebra stripes · **Streifen** | 🖼 zebra stripes · **stripes** |
| klasse1 | Ich habe Streifen. Ich lebe in Afrika. Ich renne sehr schnell. | I have stripes. I live in Africa. I run very fast. |
| klasse2 | Zebras leben in großen Herden in Afrika. Jedes Zebra hat andere Streifen. Kein Muster gibt es zweimal. Zebras können sehr schnell rennen. | Zebras live in big herds in Africa. Every zebra has different stripes. No two patterns are the same. Zebras can run very fast. |
| klasse3 | Zebras kommen aus Afrika und leben dort in großen Herden. Ihre Streifen sind so einmalig wie dein Fingerabdruck: Kein Zebra sieht genauso aus wie ein anderes. Den ganzen Tag fressen Zebras Gras. Kommt ein Löwe, rennen sie davon, so schnell wie ein Auto in der Stadt. Ein Zebrafohlen kann schon kurz nach der Geburt laufen. | Zebras come from Africa and live there in big herds. Their stripes are as unique as your fingerprint: no zebra looks exactly like another. Zebras eat grass all day long. When a lion comes, they run away, as fast as a car in town. A zebra foal can walk soon after it is born. |

Mission complete (`mission-zebra-home`, all reading levels, PoC M4): *Super! Die Zebras sind
wieder zu Hause.* / *Great! The zebras are home again.*

Math (`mathe1`): *3 Zebras fressen Gras. 2 kommen dazu. Wie viele Zebras sind es?* /
*3 zebras are eating grass. 2 more come. How many zebras are there?* → **5**

## 2. Hippo — `loc_pond`, `loc_mud`, `loc_shade`

Riddle — `loc_pond` (keys `mission-hippo-riddle-loc_pond-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 pond · **Teich** | 🖼 pond · **pond** |
| klasse1 | Ich bade gern. Das Wasser ist still. Dort blühen Seerosen. | I like to bathe. The water is still. Water lilies grow there. |
| klasse2 | Das Flusspferd liegt gern im Wasser. Es mag stilles Wasser mit Seerosen und Fröschen. | The hippo likes to lie in water. It likes still water with water lilies and frogs. |
| klasse3 | Das Flusspferd wird in der Sonne schnell heiß. Darum ist es in ein Wasser gestiegen, das nicht fließt. Auf dem Wasser schwimmen Seerosen, und am Ufer quaken Frösche. Nur die Augen und Ohren schauen heraus. | The hippo gets hot quickly in the sun. So it climbed into water that does not flow. Water lilies float on top, and frogs croak on the shore. Only its eyes and ears stick out. |

Place words: *Teich, Seerosen, Frösche* / *pond, water lilies, frogs*.

Riddle — `loc_mud` (keys `mission-hippo-riddle-loc_mud-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 brown mud puddle · **Matsch** | 🖼 brown mud puddle · **mud** |
| klasse1 | Ich mag es nass. Der Boden ist braun. Es spritzt und schmatzt. | I like it wet. The ground is brown. It splashes and squelches. |
| klasse2 | Das Flusspferd liegt in einer großen, braunen Pfütze. Wenn es sich bewegt, spritzt und schmatzt es laut. | The hippo lies in a big, brown puddle. When it moves, it splashes and squelches loudly. |
| klasse3 | Heute Nacht hat es geregnet. Jetzt gibt es im Zoo eine Stelle, an der der Boden ganz weich und braun ist. Das Flusspferd liebt so etwas: Es wälzt sich darin, bis sein Rücken braun ist wie Schokolade. Such dort, wo es schmatzt und spritzt. | It rained last night. Now there is a spot in the zoo where the ground is soft and brown. The hippo loves that: it rolls around until its back is as brown as chocolate. Look where it squelches and splashes. |

Place words: *Matsch, Pfütze, Schlamm* / *mud, puddle*.

Riddle — `loc_shade` (keys `mission-hippo-riddle-loc_shade-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 deep shade under big trees by a wall · **Schatten** | 🖼 deep shade under big trees by a wall · **shade** |
| klasse1 | Mir ist zu heiß. Ich liege unter Bäumen. Hinter mir ist eine Mauer. | I am too hot. I lie under trees. Behind me is a wall. |
| klasse2 | Dem Flusspferd ist die Sonne zu heiß. Es liegt dort, wo große Bäume die Sonne verdecken, direkt an der hohen Mauer des Zoos. | The sun is too hot for the hippo. It lies where big trees hide the sun, right next to the high zoo wall. |
| klasse3 | Flusspferde haben eine empfindliche Haut und mögen keine pralle Sonne. Darum hat sich unser Flusspferd einen dunklen, kühlen Platz gesucht. Dicke Baumkronen halten die Sonne ab, und hinter ihm steht die hohe Mauer, die den ganzen Zoo umgibt. Dort liegt es im trockenen Gras und döst. | Hippos have sensitive skin and do not like strong sun. So our hippo has looked for a dark, cool spot. Thick treetops keep the sun away, and behind it stands the high wall around the whole zoo. It lies there in the dry grass and dozes. |

Place words: *Schatten, Mauer* / *shade, wall*.

Animal name (`animal-hippo`, all reading levels): *Flusspferd* / *Hippo*. Heading above the
facts (`animal-hippo-more`): *Mehr über das Flusspferd* / *More about the hippo*.

**Facts** (*Steckbrief*, GAME-ANIMALS "Info board" item 4, keys `mission-hippo-facts-<reading_level>`;
shown after the riddle and the food word; never a place word of any hippo candidate — ANIM-007
checks the whole words *Teich*, *Seerosen*, *Frösche*, *Wasser*, *Matsch*, *Pfütze*,
*Schlamm*, *Schatten*, *Mauer*, *Bäume* / *pond*, *water*, *lilies*, *frogs*, *mud*, *puddle*,
*shade*, *wall*, *trees*; the facts also avoid swimming, diving and sweat/sun protection, which
would hint at a place):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 wide-open hippo mouth · **Maul** | 🖼 wide-open hippo mouth · **mouth** |
| klasse1 | Ich bin sehr schwer. Ich lebe in Afrika. Mein Maul ist riesig. | I am very heavy. I live in Africa. My mouth is huge. |
| klasse2 | Flusspferde leben in Afrika. Sie sind so schwer wie zwei Autos. Nachts fressen sie viel Gras. Ihr Maul können sie riesig weit aufreißen. | Hippos live in Africa. They are as heavy as two cars. At night they eat lots of grass. They can open their mouths very, very wide. |
| klasse3 | Flusspferde kommen aus Afrika und gehören zu den schwersten Tieren an Land. Ein großes Flusspferd wiegt so viel wie zwei Autos. Trotzdem kann es schneller rennen als ein Mensch. Nachts wandert es umher und frisst viel Gras. Im Zoo mag es besonders gern Melonen. | Hippos come from Africa and are among the heaviest animals on land. A big hippo weighs as much as two cars. Even so, it can run faster than a person. At night it wanders around and eats lots of grass. In the zoo it especially likes melons. |

Mission complete (`mission-hippo-home`, all reading levels): *Super! Das Flusspferd ist wieder
zu Hause.* / *Great! The hippo is home again.*

Math (`mathe2`): *Jedes Flusspferd frisst 4 Melonen. Es gibt 3 Flusspferde. Wie viele Melonen brauchst du?* /
*Each hippo eats 4 melons. There are 3 hippos. How many melons do you need?* → **12**
(herd size depends on Q-004 / Q-030)

## 3. Panda — `loc_cave`, `loc_bamboo`, `loc_leaves`

Riddle — `loc_cave` (keys `mission-panda-riddle-loc_cave-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 cave · **Höhle** | 🖼 cave · **cave** |
| klasse1 | Ich bin müde. Dort ist es dunkel. Dort ist es kühl. | I am tired. It is dark there. It is cool there. |
| klasse2 | Der Panda ist müde. Er schläft an einem dunklen, kühlen Ort mit Wänden aus Stein. | The panda is tired. It sleeps in a dark, cool place with walls of stone. |
| klasse3 | Der Panda hat den ganzen Morgen gefressen. Jetzt ist er sehr müde. Er hat einen Platz gefunden, an dem es dunkel und kühl ist. Die Wände sind aus Stein, und wenn man ruft, hallt es zurück. | The panda ate all morning. Now it is very tired. It found a place that is dark and cool. The walls are made of stone, and when you shout, it echoes. |

Place words: *Höhle, Stein, Echo* / *cave, stone, echo*.

Riddle — `loc_bamboo` (keys `mission-panda-riddle-loc_bamboo-<reading_level>`). The `kiga` word is
*Bambuswald*, not *Bambus*, because *Bambus* is the panda's food word (it must stay allowed
in the facts, ANIM-007):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 tall bamboo thicket · **Bambuswald** | 🖼 tall bamboo thicket · **bamboo forest** |
| klasse1 | Ich habe Hunger. Hier wachsen grüne Stangen. Sie sind sehr hoch. | I am hungry. Green stalks grow here. They are very tall. |
| klasse2 | Der Panda hat Hunger. Er sitzt dort, wo grüne Stangen dicht an dicht wachsen, höher als die Mauer des Zoos. | The panda is hungry. It sits where green stalks grow close together, taller than the zoo wall. |
| klasse3 | Pandas fressen fast den ganzen Tag. Unser Panda hat einen Platz gefunden, an dem sein Lieblingsessen einfach aus dem Boden wächst. Die grünen Stangen stehen so dicht, dass man kaum hindurchsehen kann, und sie sind höher als die Mauer. Wenn der Wind weht, klappern sie leise. | Pandas eat almost all day long. Our panda has found a place where its favourite food simply grows out of the ground. The green stalks stand so close together that you can hardly see through them, and they are taller than the wall. When the wind blows, they rattle softly. |

Place words: *Bambuswald, Dickicht* / *bamboo forest, thicket* (not *Bambus* / *bamboo* — food word).

Riddle — `loc_leaves` (keys `mission-panda-riddle-loc_leaves-<reading_level>`). The `kiga` word is
*Laubhaufen* / *leaf pile*, not *Blätter* / *leaves*, which is the giraffe's food word:

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 big pile of red and yellow leaves · **Laubhaufen** | 🖼 big pile of red and yellow leaves · **leaf pile** |
| klasse1 | Ich spiele gern. Mein Bett ist weich. Rotes und gelbes Laub! | I like to play. My bed is soft. Red and yellow leaves! |
| klasse2 | Der Panda spielt gern. Er hat sich in einen großen, weichen Haufen aus roten und gelben Blättern gekuschelt. | The panda likes to play. It has snuggled into a big, soft heap of red and yellow leaves. |
| klasse3 | Der Zoowärter hat unter den Bäumen fleißig Blätter zusammengeharkt. Jetzt liegt dort ein riesiger Haufen aus roten, gelben und braunen Blättern. Der Panda hat ihn entdeckt und sich mitten hineingeworfen. Wenn er sich dreht, raschelt es, und Blätter fliegen durch die Luft. | The zookeeper has been busy raking leaves under the trees. Now there is a huge heap of red, yellow and brown leaves. The panda found it and jumped right into the middle. When it turns over, the leaves rustle and fly through the air. |

Place words: *Laubhaufen, Laub, Haufen* / *leaf pile, heap*.

Animal name (`animal-panda`, all reading levels): *Panda* / *Panda*. Heading above the facts
(`animal-panda-more`): *Mehr über den Panda* / *More about the panda*.

**Facts** (*Steckbrief*, GAME-ANIMALS "Info board" item 4, keys `mission-panda-facts-<reading_level>`;
shown after the riddle and the food word; never a place word of any panda candidate — ANIM-007
checks the whole words *Höhle*, *Stein*, *Echo*, *dunkel*, *kühl*, *Bambuswald*, *Dickicht*,
*Stangen*, *Laubhaufen*, *Laub*, *Haufen*, *Blätter* / *cave*, *stone*, *echo*, *dark*, *cool*,
*bamboo forest* (word by word, except the food word *bamboo*), *thicket*, *stalks*, *leaf pile*
(word by word), *heap*, *leaves*; *Bambus* / *bamboo* is the food word and allowed; the facts
also avoid sleeping and playing, which would hint at `loc_cave` / `loc_leaves`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 black-and-white panda fur · **Fell** | 🖼 black-and-white panda fur · **fur** |
| klasse1 | Ich bin schwarz und weiß. Ich lebe in China. Ich fresse gern Bambus. | I am black and white. I live in China. I love to eat bamboo. |
| klasse2 | Pandas leben in China. Ihr Fell ist schwarz und weiß. Sie fressen fast nur Bambus, viele Stunden am Tag. Ein Pandababy ist bei der Geburt nicht größer als eine Banane. | Pandas live in China. Their fur is black and white. They eat almost only bamboo, many hours a day. A newborn panda baby is no bigger than a banana. |
| klasse3 | Große Pandas kommen aus China und leben dort in den Bergen. Sie fressen fast nur Bambus, bis zu vierzehn Stunden am Tag. Damit sie ihr Futter gut festhalten können, haben sie an jeder Vorderpfote einen extra Knochen, der wie ein Daumen hilft. Ein Pandababy ist bei der Geburt nicht größer als eine Banane. Zuerst ist es rosa, das schwarz-weiße Fell wächst erst später. | Giant pandas come from China, where they live in the mountains. They eat almost only bamboo, up to fourteen hours a day. To hold their food tightly, they have an extra bone on each front paw that works like a thumb. A newborn panda baby is no bigger than a banana. At first it is pink, and its black and white fur grows later. |

Mission complete (`mission-panda-home`, all reading levels): *Super! Der Panda ist wieder zu
Hause.* / *Great! The panda is home again.*

Math (`mathe1`): *Der Panda will 10 Stunden schlafen. 6 Stunden sind schon vorbei. Wie viele Stunden noch?* /
*The panda wants to sleep 10 hours. 6 hours have passed. How many hours are left?* → **4**

## 4. Koala (pair) — `loc_treehouse`, `loc_tallest_tree`, `loc_blossom_tree`

Level: `level_2` (GAME-LEVEL-2). Food box: **Eukalyptus** / *eucalyptus*. The koalas are a **pair** (GAME-FAMILY): riddles use *wir* / *we* and the plural. Riddle keys `mission-koala-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_treehouse` (wooden tree house with a rope ladder in an old oak; keys `mission-koala-riddle-loc_treehouse-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 wooden tree house with a rope ladder in an old oak · **Baumhaus** | 🖼 wooden tree house with a rope ladder in an old oak · **treehouse** |
| klasse1 | Wir klettern gern. Oben ist ein Haus. Eine Leiter hängt herab. | We like to climb. A house is up high. A rope ladder hangs down. |
| klasse2 | Die Koalas sitzen hoch oben in einem kleinen Haus aus Holz mit Dach und Fenster. Hinauf geht es nur über eine Strickleiter. | The koalas sit high up in a little wooden house with a roof and a window. The only way up is a rope ladder. |
| klasse3 | Koalas klettern gern, und am liebsten sind sie hoch oben. Unsere zwei Koalas haben ein kleines Haus aus Holz entdeckt, das zwischen dicken Ästen sitzt. Es hat ein Dach, ein Fenster und eine Strickleiter. Dort oben fühlen sie sich wie in einem Nest. | Koalas love to climb, and they like it best high up. Our two koalas have found a little wooden house that sits between thick branches. It has a roof, a window and a rope ladder. Up there they feel as cosy as in a nest. |

Place words: *Baumhaus*, *Strickleiter* / *treehouse*, *ladder*.

Riddle — `loc_tallest_tree` (the tallest tree of the zoo, koalas at the very top; keys `mission-koala-riddle-loc_tallest_tree-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 the tallest tree of the zoo, koalas at the very top · **Riesenbaum** | 🖼 the tallest tree of the zoo, koalas at the very top · **treetop** |
| klasse1 | Wir sitzen ganz oben. Nichts im Zoo ist höher. Wir sehen alles! | We sit at the top. Nothing here is higher. We can see everything! |
| klasse2 | Die Koalas sitzen ganz oben im höchsten Baum des Zoos. Er ist doppelt so hoch wie alle anderen Bäume. | The koalas sit at the very top of the tallest tree in the zoo. It is twice as tall as all the other trees. |
| klasse3 | Von ganz oben kann man am weitesten schauen, das wissen auch Koalas. Unsere zwei Koalas haben sich den höchsten Baum im ganzen Zoo ausgesucht. Sein Stamm ist so dick, dass drei Kinder ihn nicht umarmen können. Seine Krone ragt weit über alle anderen Bäume hinaus. | From the very top you can see the farthest, and koalas know that too. Our two koalas have picked the tallest tree in the whole zoo. Its trunk is so thick that three children cannot hug it. Its crown towers far above all the other trees. |

Place words: *Riesenbaum*, *Wipfel* / *treetop*, *tallest*.

Riddle — `loc_blossom_tree` (tree full of pink blossoms, petals drifting down, bees; keys `mission-koala-riddle-loc_blossom_tree-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 tree full of pink blossoms, petals drifting down, bees · **Blüten** | 🖼 tree full of pink blossoms, petals drifting down, bees · **blossoms** |
| klasse1 | Wir mögen es bunt. Alles ist rosa. Es duftet süß. | We like pretty colours. Everything is pink. It smells sweet. |
| klasse2 | Die Koalas sitzen in einem Baum, der ganz rosa ist. Kleine rosa Flocken schweben leise zu Boden, und Bienen summen. | The koalas sit in a tree that is all pink. Little pink flakes float softly to the ground, and bees hum. |
| klasse3 | Im Frühling zieht ein Baum im Zoo ein rosa Kleid an. Die Koalas finden ihn wunderschön und sind hinaufgeklettert. Wenn der Wind weht, rieseln kleine rosa Blättchen herab wie Schnee. Um die Äste summen fleißige Bienen. | In spring one tree in the zoo puts on a pink dress. The koalas think it is beautiful and have climbed up. When the wind blows, little pink petals drift down like snow. Busy bees hum around the branches. |

Place words: *Blüten*, *rosa* / *blossoms*, *pink*.

The `kiga` words differ by language on purpose: *Riesenbaum* (de) but *treetop* (en), because an English label containing *tree* would forbid the word *tree* in every koala riddle (MISS-007) — proposal Q-095.

Animal name (`animal-koala`): *Koala* / *Koala*. Heading above the facts (`animal-koala-more`): *Mehr über den Koala* / *More about the koala*.

**Facts** (keys `mission-koala-facts-<reading_level>`; never a place word — ANIM-007 checks *Baumhaus*, *Blüten*, *Riesenbaum*, *Strickleiter*, *Wipfel*, *rosa* / *blossoms*, *ladder*, *pink*, *tallest*, *treehouse*, *treetop*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Beutel** | 🖼 · **pouch** |
| klasse1 | Ich schlafe sehr viel. Ich lebe in Australien. Mein Baby wohnt im Beutel. | I sleep a lot. I live in Australia. Babies live in a pouch. |
| klasse2 | Koalas leben in Australien. Sie schlafen bis zu zwanzig Stunden am Tag. Ein Koalababy wohnt die ersten Monate im Beutel der Mutter. Koalas fressen fast nur Eukalyptus. | Koalas live in Australia. They sleep up to twenty hours a day. A koala baby spends its first months in its mother's pouch. Koalas eat almost nothing but eucalyptus. |
| klasse3 | Koalas kommen aus Australien und sehen aus wie kleine Teddybären, sind aber keine Bären. Sie fressen fast nur Eukalyptus. Diese Blätter geben wenig Kraft, darum schlafen Koalas bis zu zwanzig Stunden am Tag. Ein Koalababy heißt Joey und wohnt die ersten Monate im Beutel seiner Mutter. Später reitet es auf ihrem Rücken. | Koalas come from Australia and look like little teddy bears, but they are not bears. They eat almost nothing but eucalyptus. These leaves give them little energy, so koalas sleep up to twenty hours a day. A koala baby is called a joey and lives in its mother's pouch for its first months. Later it rides on her back. |

Mission complete (`mission-koala-home`): *Super! Die Koalas sind wieder zu Hause.* / *Great! The koalas are home again.*

Math (`mathe1`): *Ein Koala schläft 20 Stunden am Tag. Ein Tag hat 24 Stunden. Wie viele Stunden ist er wach?* /
*A koala sleeps 20 hours a day. A day has 24 hours. How many hours is it awake?* → **4** (reworded 2026-09-26: the old task named the tallest tree, now a hiding place (Behaviour 4))

## 5. Elephant — `loc_fountain`, `loc_log_pile`, `loc_big_ball`

Level: `level_2` (GAME-LEVEL-2). Food box: **Heu** / *hay*. Riddle keys `mission-elephant-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_fountain` (round stone fountain with a water jet and coins; keys `mission-elephant-riddle-loc_fountain-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 round stone fountain with a water jet and coins · **Brunnen** | 🖼 round stone fountain with a water jet and coins · **fountain** |
| klasse1 | Ich habe Durst. Das Wasser springt hoch. Unten glänzen Münzen. | I am thirsty. The water jumps up high. Coins shine at the bottom. |
| klasse2 | Der Elefant trinkt dort, wo Wasser aus der Mitte eines runden Steinbeckens in die Luft spritzt. Auf dem Boden des Beckens glänzen Münzen. | The elephant drinks where water sprays into the air from the middle of a round stone basin. Coins shine on the bottom of the basin. |
| klasse3 | Der Elefant hat großen Durst und eine gute Nase für Wasser. Er hat ein rundes Becken aus Stein gefunden. In der Mitte schießt Wasser hoch in die Luft und plätschert wieder herunter. Auf dem Grund liegen glänzende Münzen, denn Besucher wünschen sich hier etwas. Jetzt saugt er mit dem Rüssel Wasser auf und duscht sich. | The elephant is very thirsty and has a good nose for water. It has found a round stone basin. In the middle, water shoots high into the air and splashes back down. Shiny coins lie on the bottom, because visitors make a wish here. Now it sucks up water with its trunk and gives itself a shower. |

Place words: *Brunnen*, *Münzen* / *fountain*, *coins*.

Riddle — `loc_log_pile` (stacked logs of the fallen tree, sawdust; keys `mission-elephant-riddle-loc_log_pile-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 stacked logs of the fallen tree, sawdust · **Holzstapel** | 🖼 stacked logs of the fallen tree, sawdust · **log pile** |
| klasse1 | Ich bin sehr stark. Ich trage schweres Holz. Es riecht nach Sägespänen. | I am very strong. I carry heavy wood. It smells of sawdust. |
| klasse2 | Der Elefant steht bei dicken Baumstämmen, die ordentlich übereinander liegen. Rundherum liegen Sägespäne im Gras. | The elephant stands by thick tree trunks stacked neatly on top of each other. Sawdust lies all around in the grass. |
| klasse3 | Nach dem Sturm haben die Zoowärter den umgestürzten Baum in dicke Stücke gesägt und ordentlich aufgeschichtet. Das gefällt dem Elefanten! In seiner Heimat rollt er gern schwere Stämme herum. Jetzt steht er neben dem Holz, und um ihn herum riecht es nach frischen Sägespänen. | After the storm the zookeepers sawed the fallen tree into thick pieces and stacked them up neatly. The elephant loves that! At home it likes to roll heavy tree trunks around. Now it stands next to the wood, and all around it smells of fresh sawdust. |

Place words: *Holzstapel*, *Stapel* / *log pile*, *log*, *pile*.

Riddle — `loc_big_ball` (giant red-and-white play ball on the lawn; keys `mission-elephant-riddle-loc_big_ball-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 giant red-and-white play ball on the lawn · **Ball** | 🖼 giant red-and-white play ball on the lawn · **ball** |
| klasse1 | Ich spiele gern. Es ist rund und rot. Es ist größer als du! | I like to play. It is round and red. It is bigger than you! |
| klasse2 | Der Elefant spielt mit etwas Riesigem, Rundem in Rot und Weiß. Er schubst es mit dem Rüssel über das Gras. | The elephant plays with something huge and round, red and white. It pushes it across the grass with its trunk. |
| klasse3 | Elefanten sind klug und spielen gern, auch wenn sie schon groß sind. Die Zoowärter haben ihnen deshalb ein Spielzeug geschenkt, das größer ist als ein Kind. Es ist rund, rot und weiß und rollt, wenn man es anstößt. Unser Elefant schubst es mit dem Rüssel und trompetet vor Freude. | Elephants are clever and like to play, even when they are grown up. So the zookeepers gave them a toy that is bigger than a child. It is round, red and white, and it rolls when you push it. Our elephant nudges it with its trunk and trumpets with joy. |

Place words: *Ball* / *ball*.

Animal name (`animal-elephant`): *Elefant* / *Elephant*. Heading above the facts (`animal-elephant-more`): *Mehr über den Elefanten* / *More about the elephant*.

**Facts** (keys `mission-elephant-facts-<reading_level>`; never a place word — ANIM-007 checks *Ball*, *Brunnen*, *Holzstapel*, *Münzen*, *Stapel* / *ball*, *coins*, *fountain*, *log*, *log pile*, *pile*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Rüssel** | 🖼 · **trunk** |
| klasse1 | Ich habe einen Rüssel. Ich bin sehr schwer. Meine Ohren sind riesig. | I have a trunk. I am very heavy. My ears are huge. |
| klasse2 | Elefanten sind die größten Tiere an Land. Mit dem Rüssel können sie riechen, trinken und greifen. Mit ihren großen Ohren fächeln sie sich kühle Luft zu. | Elephants are the biggest animals on land. With their trunk they can smell, drink and grab things. They fan themselves cool air with their big ears. |
| klasse3 | Elefanten sind die größten Tiere, die auf dem Land leben. Ihr Rüssel ist Nase und Hand zugleich: Damit riechen sie, trinken sie und heben sogar kleine Äste auf. Ein Elefant frisst jeden Tag so viel Heu und Gras, wie in einen ganzen Wagen passt. Elefanten vergessen nie, wer nett zu ihnen war. Eine Elefantenfamilie hält fest zusammen. | Elephants are the biggest animals that live on land. Their trunk is a nose and a hand at the same time: with it they smell, drink and even pick up small branches. Every day an elephant eats as much hay and grass as fits into a whole cart. Elephants never forget who was kind to them. An elephant family sticks together. |

Mission complete (`mission-elephant-home`): *Super! Der Elefant ist wieder zu Hause.* / *Great! The elephant is home again.*

Math (`mathe3`): *Ein Elefant frisst am Tag 100 kg Heu. Wie viel frisst er in 3 Tagen?* /
*An elephant eats 100 kg of hay a day. How much does it eat in 3 days?* → **300 kg**

## 6. Goldfish — `loc_waterfall`, `loc_water_wheel`, `loc_willow`

Level: `level_3` (GAME-LEVEL-3). Food box: **Fischfutter** / *fish food*. All three places are in the stream `stream_l3`; the player feeds the fish from the bank and needs the filled fish bowl (GAME-RESCUE "goldfish bowl"). Riddle keys `mission-goldfish-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_waterfall` (waterfall from a rock ledge into the stream, white foam; keys `mission-goldfish-riddle-loc_waterfall-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 waterfall from a rock ledge into the stream, white foam · **Wasserfall** | 🖼 waterfall from a rock ledge into the stream, white foam · **waterfall** |
| klasse1 | Ich schwimme gern. Hier rauscht es laut. Weißer Schaum! | I like to swim. It is loud here. White foam! |
| klasse2 | Der Goldfisch schwimmt dort, wo das Wasser von hohen Steinen herunterfällt und weiß schäumt. | The goldfish swims where the water tumbles down from high stones and foams white. |
| klasse3 | Der kleine Goldfisch ist weit gegen den Strom geschwommen, fast bis zum Rand des Zoos. Dort fällt das Wasser über eine Felskante herunter und spritzt in alle Richtungen. Es rauscht so laut, dass man sein eigenes Wort kaum versteht. Im weißen Schaum blitzt etwas Oranges. | The little goldfish has swum far upstream, almost to the edge of the zoo. There the water falls down over a rocky ledge and splashes in every direction. It roars so loudly that you can hardly hear yourself. Something orange flashes in the white foam. |

Place words: *Wasserfall*, *Schaum* / *waterfall*, *foam*.

Riddle — `loc_water_wheel` (wooden water wheel turning in the stream at a tiny mill hut; keys `mission-goldfish-riddle-loc_water_wheel-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 wooden water wheel turning in the stream at a tiny mill hut · **Wasserrad** | 🖼 wooden water wheel turning in the stream at a tiny mill hut · **waterwheel** |
| klasse1 | Klapper, klapper! Ein Rad dreht sich. Es ist aus Holz. | Clatter, clatter! A wheel turns round. It is made of wood. |
| klasse2 | Der Goldfisch schwimmt neben einem großen Rad aus Holz, das sich im Wasser dreht und laut klappert. Daneben steht eine kleine Hütte. | The goldfish swims next to a big wooden wheel that turns in the water and clatters loudly. A little hut stands beside it. |
| klasse3 | Am Ufer steht eine winzige Mühle aus Holz. Das Wasser schiebt ein großes Rad an, das sich immer weiter dreht. Klapper, klapper, macht es, und Tropfen fallen glitzernd herunter. Der Goldfisch findet das lustig und schwimmt direkt daneben im Kreis. | A tiny wooden mill stands on the bank. The water pushes a big wheel that keeps on turning. Clatter, clatter, it goes, and glittering drops fall down. The goldfish thinks this is fun and swims in circles right beside it. |

Place words: *Wasserrad*, *Mühle* / *waterwheel*, *mill*.

Riddle — `loc_willow` (weeping willow whose branches hang into the stream; keys `mission-goldfish-riddle-loc_willow-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 weeping willow whose branches hang into the stream · **Weide** | 🖼 weeping willow whose branches hang into the stream · **willow** |
| klasse1 | Ich mag es ruhig. Lange Zweige hängen herab. Sie berühren das Wasser. | I like it quiet. Long branches hang down. They touch the water. |
| klasse2 | Der Goldfisch versteckt sich dort, wo ein Baum seine langen, dünnen Zweige bis ins Wasser hängen lässt. | The goldfish hides where a tree lets its long, thin branches hang down into the water. |
| klasse3 | Am Ufer steht ein Baum, der aussieht, als ließe er traurig den Kopf hängen. Seine langen, dünnen Zweige fallen wie ein grüner Vorhang bis ins Wasser. Darunter ist es schattig und still. Genau dort hat sich der Goldfisch versteckt. | On the bank stands a tree that looks as if it were sadly hanging its head. Its long, thin branches fall like a green curtain into the water. Underneath it is shady and calm. That is exactly where the goldfish is hiding. |

Place words: *Weide*, *Trauerweide* / *willow*.

Animal name (`animal-goldfish`): *Goldfisch* / *Goldfish*. Heading above the facts (`animal-goldfish-more`): *Mehr über den Goldfisch* / *More about the goldfish*.

**Facts** (keys `mission-goldfish-facts-<reading_level>`; never a place word — ANIM-007 checks *Mühle*, *Schaum*, *Trauerweide*, *Wasserfall*, *Wasserrad*, *Weide* / *foam*, *mill*, *waterfall*, *waterwheel*, *willow*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Flossen** | 🖼 · **fins** |
| klasse1 | Ich bin orange. Ich habe Flossen. Ich atme im Wasser. | I am orange. I have fins. I breathe in water. |
| klasse2 | Goldfische kommen ursprünglich aus China. Sie atmen mit Kiemen unter Wasser. Mit ihren Flossen steuern sie wie mit kleinen Rudern. | Goldfish originally come from China. They breathe under water with gills. They steer with their fins as if with little oars. |
| klasse3 | Goldfische stammen aus China, wo man sie schon vor sehr langer Zeit gezüchtet hat. Sie atmen nicht mit einer Lunge, sondern mit Kiemen. Mit ihren Flossen lenken und bremsen sie wie mit kleinen Rudern. Ein Goldfisch kann viele Jahre alt werden, manchmal älter als du. Er erkennt sogar die Person, die ihn füttert. | Goldfish come from China, where people bred them a very long time ago. They do not breathe with lungs but with gills. They steer and brake with their fins as if with little oars. A goldfish can live for many years, sometimes longer than you have been alive. It even recognises the person who feeds it. |

Mission complete (`mission-goldfish-home`): *Super! Der Goldfisch ist wieder zu Hause.* / *Great! The goldfish is home again.*

**Fish bowl texts** (GAME-RESCUE "goldfish bowl"; the board hint is shown on the goldfish info board after the food word):

| Key | Deutsch | English |
|---|---|---|
| `mission-goldfish-needs-bowl` | Ich brauche ein Glas mit Wasser! | I need a bowl of water! |
| `mission-goldfish-bowl-empty` | Im Glas ist ja kein Wasser! | There is no water in the bowl! |
| `mission-goldfish-bowl-filled` | Das Glas ist voll Wasser. | The bowl is full of water. |
| `mission-goldfish-in-bowl` | Platsch! Der Goldfisch ist im Glas. | Splash! The goldfish is in the bowl. |
| `mission-goldfish-bowl-hint-kiga` | Glas | bowl |
| `mission-goldfish-bowl-hint-klasse1` | Ich brauche ein Glas. Das Glas braucht Wasser. | I need a bowl. The bowl needs water. |
| `mission-goldfish-bowl-hint-klasse2` | Bring ein großes Glas voll Wasser mit. Nur darin kann der Goldfisch nach Hause reisen. | Bring a big bowl full of water. Only in it can the goldfish travel home. |
| `mission-goldfish-bowl-hint-klasse3` | Ein Goldfisch kann nicht über den Weg laufen. Du brauchst ein großes Glas, und die Zoowärter haben eins in ihrem Haus. Fülle es mit Wasser, dann kann der Goldfisch hineinspringen. | A goldfish cannot walk along the path. You need a big glass bowl, and the zookeepers keep one in their house. Fill it with water, and the goldfish can jump in. |

Math (`mathe1`): *Du hast 7 Futterflocken. Der Goldfisch frisst 3. Wie viele sind noch übrig?* /
*You have 7 food flakes. The goldfish eats 3. How many are left?* → **4** (reworded 2026-09-26: the old task (coins in the water) pointed at the fountain)

## 7. Monkey — `loc_pirate_ship`, `loc_carousel`, `loc_trampoline`

Level: `level_3` (GAME-LEVEL-3). Food box: **Bananen** / *bananas*. Riddle keys `mission-monkey-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_pirate_ship` (pirate ship climbing frame: mast, sail, black flag, treasure chest; keys `mission-monkey-riddle-loc_pirate_ship-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 pirate ship climbing frame: mast, sail, black flag, treasure chest · **Schiff** | 🖼 pirate ship climbing frame: mast, sail, black flag, treasure chest · **ship** |
| klasse1 | Ahoi! Ich bin jetzt Pirat. Ich sitze am Mast. | Ahoy! I am a pirate now. I sit on the mast. |
| klasse2 | Der Affe spielt jetzt Pirat. Er sitzt ganz oben am Mast und sucht nach einem Schatz. | The monkey is playing pirate now. It sits at the top of the mast and looks for treasure. |
| klasse3 | Der freche Affe hat ein neues Zuhause gefunden – glaubt er jedenfalls. Es hat ein großes Segel, eine schwarze Flagge und eine Schatzkiste. Er ruft „Ahoi!“ und klettert ganz nach oben. | The cheeky monkey has found a new home – or so it thinks. It has a big sail, a black flag and a treasure chest. It shouts "Ahoy!" and climbs all the way up. |

Place words: *Schiff*, *Pirat*, *Mast* / *ship*, *pirate*, *mast*.

Riddle — `loc_carousel` (small carousel with wooden horses and a striped roof; keys `mission-monkey-riddle-loc_carousel-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 small carousel with wooden horses and a striped roof · **Karussell** | 🖼 small carousel with wooden horses and a striped roof · **carousel** |
| klasse1 | Ich reite gern. Mein Pferd ist aus Holz. Alles dreht sich! | I like to ride. My horse is wooden. Everything turns! |
| klasse2 | Der Affe sitzt auf einem Pferd aus Holz. Es fährt im Kreis herum, und dazu spielt fröhliche Musik. | The monkey sits on a wooden horse. It goes round and round while happy music plays. |
| klasse3 | Der Affe liebt es, wenn sich alles dreht. Er hat einen Platz gefunden, an dem bunte Holzpferde unter einem gestreiften Dach im Kreis fahren. Dazu klingt eine fröhliche Melodie. Der Affe sitzt verkehrt herum auf einem Pferd und quietscht vor Vergnügen. | The monkey loves it when everything spins. It has found a place where colourful wooden horses go round in a circle under a striped roof. A cheerful tune plays along. The monkey sits backwards on a horse and squeals with delight. |

Place words: *Karussell*, *Pferd* / *carousel*, *horse*.

Riddle — `loc_trampoline` (ground-level trampoline in the lawn; keys `mission-monkey-riddle-loc_trampoline-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 ground-level trampoline in the lawn · **Trampolin** | 🖼 ground-level trampoline in the lawn · **trampoline** |
| klasse1 | Hopp, hopp, hopp! Ich springe hoch. Der Boden federt. | Boing, boing, boing! I jump up high. The ground is bouncy. |
| klasse2 | Der Affe hüpft auf einer runden, blauen Matte im Gras. Sie federt so gut, dass er fast bis in den Himmel springt. | The monkey bounces on a round blue mat in the grass. It is so springy that the monkey jumps almost up to the sky. |
| klasse3 | Affen springen gern von Ast zu Ast. Unser Affe hat etwas noch Besseres entdeckt: eine runde, blaue Matte, die im Rasen liegt. Jedes Mal, wenn er darauf landet, schleudert sie ihn wieder hoch in die Luft. Er macht Purzelbäume und lacht dabei. | Monkeys like to jump from branch to branch. Our monkey has found something even better: a round blue mat lying in the lawn. Every time it lands on it, the mat throws it back up into the air. It does somersaults and laughs. |

Place words: *Trampolin*, *Matte* / *trampoline*, *mat*.

`loc_pirate_ship` (`klasse3` changed 2026-09-26): the flag is a plain black pirate flag (white paw print, no skull — nothing scary); the pirate ship is a climbing frame on the level-3 adventure playground (Q-017 proposal).

Animal name (`animal-monkey`): *Affe* / *Monkey*. Heading above the facts (`animal-monkey-more`): *Mehr über den Affen* / *More about the monkey*.

**Facts** (keys `mission-monkey-facts-<reading_level>`; never a place word — ANIM-007 checks *Karussell*, *Mast*, *Matte*, *Pferd*, *Pirat*, *Schiff*, *Trampolin* / *carousel*, *horse*, *mast*, *mat*, *pirate*, *ship*, *trampoline*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Schwanz** | 🖼 · **tail** |
| klasse1 | Ich klettere sehr gut. Ich mag Bananen. Ich bin frech. | I climb very well. I like bananas. I am cheeky. |
| klasse2 | Affen sind sehr geschickte Kletterer. Mit ihren Händen können sie greifen wie du. Sie sind neugierig und spielen gern Streiche. | Monkeys are very good climbers. They can grab things with their hands, just like you. They are curious and love to play tricks. |
| klasse3 | Affen sind wahre Kletterkünstler und schwingen sich von Ast zu Ast. Ihre Hände sehen fast aus wie deine, und sie können damit sogar Früchte schälen. Viele Affen halten sich mit dem Schwanz fest wie mit einer dritten Hand. Sie sind sehr neugierig und lernen schnell. Am liebsten leben sie mit ihrer Familie zusammen. | Monkeys are true climbing artists and swing from branch to branch. Their hands look almost like yours, and they can even peel fruit with them. Many monkeys hold on with their tail like a third hand. They are very curious and learn quickly. They like best to live together with their family. |

Mission complete (`mission-monkey-home`): *Super! Der Affe ist wieder zu Hause.* / *Great! The monkey is home again.*

Math (`mathe2`): *Der Affe hat 25 Bananen. Er findet noch 17. Wie viele Bananen hat er jetzt?* /
*The monkey has 25 bananas. It finds 17 more. How many bananas does it have now?* → **42** (reworded 2026-09-26: the old task (treasure chest lock) pointed at the pirate ship)

## 8. Giraffe — `loc_lookout_tower`, `loc_train`, `loc_playground`

Level: `level_2` (GAME-LEVEL-2). Food box: **Blätter** / *leaves*. Riddle keys `mission-giraffe-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_lookout_tower` (wooden lookout tower with stairs and a high platform; keys `mission-giraffe-riddle-loc_lookout_tower-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 wooden lookout tower with stairs and a high platform · **Turm** | 🖼 wooden lookout tower with stairs and a high platform · **tower** |
| klasse1 | Ich bin sehr groß. Oben stehen Leute. Ich schaue sie an. | I am very tall. People stand up high. I look at them. |
| klasse2 | Die Giraffe steht neben einem hohen Gerüst aus Holz mit einer Treppe. Oben auf der Plattform ist sie genau auf Augenhöhe mit den Besuchern. | The giraffe stands next to a tall wooden frame with stairs. Up on the platform it is right at eye level with the visitors. |
| klasse3 | Besucher steigen gern viele Stufen hinauf, um von oben über den Zoo zu schauen. Heute bekommen sie eine Überraschung: Direkt neben dem Geländer taucht ein langer Hals auf! Die Giraffe ist so groß, dass sie den Leuten oben ins Gesicht schauen kann. Sie hofft, dass ihr jemand ein paar Blätter gibt. | Visitors like to climb many steps to look out over the zoo from above. Today they get a surprise: a long neck pops up right next to the railing! The giraffe is so tall that it can look the people up there in the face. It hopes that somebody will give it a few leaves. |

Place words: *Turm*, *Treppe* / *tower*, *stairs*.

Riddle — `loc_train` (little zoo train (engine + 2 wagons) at its station; keys `mission-giraffe-riddle-loc_train-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 little zoo train (engine + 2 wagons) at its station · **Zug** | 🖼 little zoo train (engine + 2 wagons) at its station · **train** |
| klasse1 | Tuut, tuut! Hier ist ein Bahnhof. Die Glocke bimmelt. | Toot, toot! Here is a station. The bell rings. |
| klasse2 | Die Giraffe steht bei einer kleinen bunten Lok mit zwei Wagen. Sie ist viel größer als der Schornstein. | The giraffe stands by a little colourful engine with two wagons. It is much taller than the chimney. |
| klasse3 | Im Zoo gibt es eine kleine Bahn, mit der Besucher gern fahren. Heute steht sie still am Bahnsteig, und die Giraffe ist neugierig geworden. Sie schnuppert am Schornstein der Lok und schaut in die leeren Wagen. Als sie an die Glocke stößt, bimmelt es laut. | The zoo has a little railway that visitors love to ride. Today it is standing still at the platform, and the giraffe has become curious. It sniffs the chimney of the engine and looks into the empty wagons. When it bumps the bell, it rings loudly. |

Place words: *Zug*, *Bahnhof* / *train*, *station*.

Riddle — `loc_playground` (playground with slide and swings; keys `mission-giraffe-riddle-loc_playground-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 playground with slide and swings · **Spielplatz** | 🖼 playground with slide and swings · **playground** |
| klasse1 | Mein Hals ist lang. Hier rutschen Kinder. Hier schaukeln Kinder. | My neck is long. Children slide here. Children swing here. |
| klasse2 | Die Giraffe steht dort, wo Kinder rutschen und schaukeln. Sie ist viel größer als die Rutsche. | The giraffe stands where children slide and swing. It is much taller than the slide. |
| klasse3 | Die Giraffe ist sehr neugierig. Sie hat Kinder lachen hören und ist dem Lachen gefolgt. Jetzt steht sie zwischen Schaukeln und Rutsche. Mit ihrer langen Zunge zupft sie Blätter von den Bäumen. | The giraffe is very curious. It heard children laughing and followed the sound. Now it stands between the swings and the slide. With its long tongue it pulls leaves off the trees. |

Place words: *Spielplatz*, *Rutsche*, *Schaukel* / *playground*, *slide*, *swing*.

`loc_playground`: `kiga` word *Spielplatz* / *playground* instead of *Rutsche* / *slide* (Q-039), and no sandpit any more (sand is the zebra's `loc_sand`, Q-083).

Animal name (`animal-giraffe`): *Giraffe* / *Giraffe*. Heading above the facts (`animal-giraffe-more`): *Mehr über die Giraffe* / *More about the giraffe*.

**Facts** (keys `mission-giraffe-facts-<reading_level>`; never a place word — ANIM-007 checks *Bahnhof*, *Rutsche*, *Schaukel*, *Spielplatz*, *Treppe*, *Turm*, *Zug* / *playground*, *slide*, *stairs*, *station*, *swing*, *tower*, *train*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Hals** | 🖼 · **neck** |
| klasse1 | Mein Hals ist sehr lang. Ich fresse Blätter. Meine Zunge ist blau. | My neck is very long. I eat leaves. My tongue is blue. |
| klasse2 | Giraffen sind die größten Tiere der Welt. Mit ihrem langen Hals kommen sie an die höchsten Blätter. Ihre Zunge ist dunkelblau und sehr lang. | Giraffes are the tallest animals in the world. With their long neck they reach the highest leaves. Their tongue is dark blue and very long. |
| klasse3 | Giraffen leben in Afrika und sind die größten Tiere der Welt. Obwohl ihr Hals so lang ist, hat er nur sieben Knochen, genau wie deiner. Ihre Zunge ist fast so lang wie dein Arm und dunkelblau. Jede Giraffe hat ihr eigenes Fleckenmuster. Giraffen schlafen oft im Stehen und nur ganz kurz. | Giraffes live in Africa and are the tallest animals in the world. Although their neck is so long, it has only seven bones, just like yours. Their tongue is almost as long as your arm and dark blue. Every giraffe has its own pattern of patches. Giraffes often sleep standing up, and only for a short time. |

Mission complete (`mission-giraffe-home`): *Super! Die Giraffe ist wieder zu Hause.* / *Great! The giraffe is home again.*

Math (`mathe4`): *Die Giraffe ist 5 m groß. Das Mädchen ist 120 cm groß. Wie viele Zentimeter ist die Giraffe größer?* /
*The giraffe is 5 m tall. The girl is 120 cm tall. How many centimetres taller is the giraffe?* → **380** (reworded 2026-09-26: the old task compared the giraffe with the slide (a hiding place))

## 9. Lion — `loc_sun_rocks`, `loc_stage`, `loc_deckchairs`

Level: `level_2` (GAME-LEVEL-2). Food box: **Fleisch** / *meat*. Riddle keys `mission-lion-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_sun_rocks` (big flat rocks in full sun, no shade; keys `mission-lion-riddle-loc_sun_rocks-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 big flat rocks in full sun, no shade · **Felsen** | 🖼 big flat rocks in full sun, no shade · **rocks** |
| klasse1 | Ich bin faul. Die Sonne ist warm. Ich liege auf Stein. | I am lazy. The sun is warm. I lie on stone. |
| klasse2 | Der Löwe liegt faul auf großen, warmen Steinen in der Sonne. Er gähnt und schläft fast ein. | The lion lies lazily on big, warm stones in the sun. It yawns and almost falls asleep. |
| klasse3 | Der Löwe ist der König der Tiere, aber heute ist er vor allem faul. Er hat sich den wärmsten Platz im Zoo gesucht. Dort liegen große, flache Steine, auf die den ganzen Tag die Sonne scheint. Er streckt sich aus und gähnt laut. | The lion is the king of animals, but today it is mostly lazy. It found the warmest spot in the zoo. There are big, flat stones there that the sun shines on all day. It stretches out and yawns loudly. |

Place words: *Felsen*, *Steine* / *rocks*, *stones*.

Riddle — `loc_stage` (round wooden music stage with a pointed roof, drums; keys `mission-lion-riddle-loc_stage-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 round wooden music stage with a pointed roof, drums · **Bühne** | 🖼 round wooden music stage with a pointed roof, drums · **stage** |
| klasse1 | Ich brülle gern laut. Hier spielt Musik. Hier stehen Trommeln. | I like to roar loudly. Music plays here. There are drums here. |
| klasse2 | Der Löwe liegt vor einem runden Podest aus Holz mit spitzem Dach, auf dem Trommeln stehen. Er brüllt, als ob er singen will. | The lion lies in front of a round wooden platform with a pointed roof and drums on it. It roars as if it wants to sing. |
| klasse3 | Der Löwe ist der König der Tiere, und heute möchte er auch ein König der Musik sein. Er hat einen runden Platz aus Holz mit einem spitzen Dach gefunden, auf dem sonst Musiker spielen. Dort stehen Trommeln und ein Xylofon. Der Löwe brüllt, so laut er kann, und das klingt fast wie ein Lied. | The lion is the king of animals, and today it also wants to be the king of music. It has found a round wooden place with a pointed roof where musicians usually play. There are drums and a xylophone. The lion roars as loudly as it can, and it sounds almost like a song. |

Place words: *Bühne*, *Trommeln* / *stage*, *drums*.

Riddle — `loc_deckchairs` (striped deckchairs under a big sunshade; keys `mission-lion-riddle-loc_deckchairs-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 striped deckchairs under a big sunshade · **Liegestuhl** | 🖼 striped deckchairs under a big sunshade · **deckchair** |
| klasse1 | Ich bin so müde. Hier ist ein Sonnenschirm. Die Stühle sind gestreift. | I am so sleepy. There is a sunshade here. The chairs are striped. |
| klasse2 | Der Löwe macht Urlaub. Er räkelt sich neben bunten, gestreiften Stühlen unter einem großen Sonnenschirm. | The lion is on holiday. It stretches out next to colourful striped chairs under a big sunshade. |
| klasse3 | Heute fühlt sich der Löwe wie im Urlaub am Meer. Er hat einen Platz mit bunt gestreiften Stühlen zum Ausruhen gefunden. Ein großer Sonnenschirm schützt ihn vor der Sonne. Dort liegt er, streckt alle vier Pfoten von sich und schnarcht. | Today the lion feels as if it were on holiday by the sea. It has found a place with colourful striped chairs for resting. A big sunshade keeps the sun off it. There it lies, stretches out all four paws and snores. |

Place words: *Liegestuhl*, *Sonnenschirm* / *deckchair*, *sunshade*.

Animal name (`animal-lion`): *Löwe* / *Lion*. Heading above the facts (`animal-lion-more`): *Mehr über den Löwen* / *More about the lion*.

**Facts** (keys `mission-lion-facts-<reading_level>`; never a place word — ANIM-007 checks *Bühne*, *Felsen*, *Liegestuhl*, *Sonnenschirm*, *Steine*, *Trommeln* / *deckchair*, *drums*, *rocks*, *stage*, *stones*, *sunshade*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Mähne** | 🖼 · **mane** |
| klasse1 | Ich habe eine Mähne. Ich brülle sehr laut. Ich schlafe viel. | I have a mane. I roar very loudly. I sleep a lot. |
| klasse2 | Löwen leben in Afrika in einer Gruppe, dem Rudel. Nur die Männchen haben eine große Mähne. Das Brüllen eines Löwen hört man sehr weit. | Lions live in Africa in a group called a pride. Only the males have a big mane. You can hear a lion's roar from very far away. |
| klasse3 | Löwen leben in Afrika und sind die einzigen Katzen, die in Gruppen wohnen. So eine Gruppe heißt Rudel. Nur die Männchen tragen eine dicke Mähne. Ihr Brüllen ist so laut, dass man es noch weit entfernt hört. Die meiste Zeit des Tages verschlafen Löwen gemütlich. | Lions live in Africa and are the only big cats that live in groups. Such a group is called a pride. Only the males have a thick mane. Their roar is so loud that you can hear it far away. Lions spend most of the day sleeping comfortably. |

Mission complete (`mission-lion-home`): *Super! Der Löwe ist wieder zu Hause.* / *Great! The lion is home again.*

Math (`mathe5`): *Der Löwe schläft ¾ von 24 Stunden. Wie viele Stunden sind das?* /
*The lion sleeps ¾ of 24 hours. How many hours is that?* → **18**

## 10. Snow fox — `loc_ice_cream_kiosk`, `loc_sprinkler`, `loc_laundry`

Level: `level_3` (GAME-LEVEL-3). Food box: **Beeren** / *berries*. Riddle keys `mission-snow_fox-riddle-<hiding_place>-<reading_level>`.

Riddle — `loc_ice_cream_kiosk` (ice cream kiosk with a freezer chest and cones; keys `mission-snow_fox-riddle-loc_ice_cream_kiosk-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 ice cream kiosk with a freezer chest and cones · **Eis** | 🖼 ice cream kiosk with a freezer chest and cones · **ice cream** |
| klasse1 | Mir ist zu warm. Ich mag es kalt. Hier gibt es Waffeln. | I am too warm. I like it cold. There are waffles here. |
| klasse2 | Dem Schneefuchs ist es viel zu warm. Er sitzt dort, wo Kinder kalte, süße Kugeln in der Waffel kaufen. | The snow fox is much too warm. It sits where children buy cold, sweet scoops in a cone. |
| klasse3 | Der Schneefuchs kommt aus einem Land voller Schnee. Im Zoo ist es ihm heute viel zu warm. Er hat einen Ort gefunden, an dem kalte Luft aus einer Truhe weht. Dort kaufen Kinder Waffeln mit bunten Kugeln. | The snow fox comes from a land full of snow. Today the zoo is much too warm for it. It found a place where cold air blows out of a chest. Children buy cones with colourful scoops there. |

Place words: *Eis*, *Kiosk*, *Waffeln* / *ice cream*, *ice*, *cream*, *kiosk*.

Riddle — `loc_sprinkler` (lawn with a turning sprinkler, cold drops, rainbow; keys `mission-snow_fox-riddle-loc_sprinkler-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 lawn with a turning sprinkler, cold drops, rainbow · **Rasensprenger** | 🖼 lawn with a turning sprinkler, cold drops, rainbow · **sprinkler** |
| klasse1 | Mir ist zu heiß. Kalte Tropfen regnen herab. Ein Regenbogen! | I am too hot. Cold drops rain down. A rainbow! |
| klasse2 | Der Schneefuchs sitzt auf nassem Gras. Dort dreht sich etwas und verspritzt kaltes Wasser im Kreis, und in den Tropfen leuchtet ein Regenbogen. | The snow fox sits on wet grass. Something there turns and sprays cold water in a circle, and a rainbow shines in the drops. |
| klasse3 | Der Schneefuchs kommt aus einem Land, in dem es fast immer schneit. Heute ist es ihm viel zu warm. Er hat einen Rasen gefunden, auf dem sich ein kleines Gerät im Kreis dreht und kühle Tropfen verteilt. Das Gras glänzt nass, und in der Luft schimmert ein kleiner Regenbogen. | The snow fox comes from a land where it snows nearly all the time. Today it is far too warm for it. It has found a lawn where a little machine turns round and round and scatters cool drops. The grass is shiny and wet, and a small rainbow shimmers in the air. |

Place words: *Rasensprenger*, *Regenbogen* / *sprinkler*, *rainbow*.

Riddle — `loc_laundry` (washing line with big white sheets; keys `mission-snow_fox-riddle-loc_laundry-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 washing line with big white sheets · **Wäsche** | 🖼 washing line with big white sheets · **laundry** |
| klasse1 | Mein Fell ist weiß. Hier ist alles weiß. Der Wind weht. | My fur is white. Everything here is white. The wind blows. |
| klasse2 | Der Schneefuchs versteckt sich zwischen großen weißen Tüchern, die an einer Leine im Wind flattern. Weiß auf Weiß sieht man ihn kaum. | The snow fox hides between big white sheets that flap on a line in the wind. White on white, you can hardly see it. |
| klasse3 | Im Winter ist das Fell des Schneefuchses so weiß wie Schnee. So kann er sich gut verstecken. Heute hat er einen Platz gefunden, an dem die Zoowärter große weiße Laken und Handtücher zum Trocknen aufgehängt haben. Zwischen den flatternden Tüchern sitzt er ganz still und hofft, dass ihn niemand entdeckt. | In winter the snow fox's fur is as white as snow. That helps it to hide well. Today it has found a place where the zookeepers have hung big white sheets and towels out to dry. It sits very still between the flapping sheets and hopes that nobody will spot it. |

Place words: *Wäsche*, *Wäscheleine*, *Laken* / *laundry*, *sheets*.

Animal name (`animal-snow_fox`): *Schneefuchs* / *Snow fox*. Heading above the facts (`animal-snow_fox-more`): *Mehr über den Schneefuchs* / *More about the snow fox*.

**Facts** (keys `mission-snow_fox-facts-<reading_level>`; never a place word — ANIM-007 checks *Eis*, *Kiosk*, *Laken*, *Rasensprenger*, *Regenbogen*, *Waffeln*, *Wäsche*, *Wäscheleine* / *cream*, *ice*, *ice cream*, *kiosk*, *laundry*, *rainbow*, *sheets*, *sprinkler*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 · **Schnee** | 🖼 · **snow** |
| klasse1 | Mein Fell ist dick. Ich mag Schnee. Ich esse Beeren. | My fur is thick. I like snow. I eat berries. |
| klasse2 | Schneefüchse leben im hohen Norden, wo es sehr kalt ist. Im Winter ist ihr Fell weiß, im Sommer braun. Ihr dicker Schwanz wärmt sie wie ein Schal. | Snow foxes live in the far north, where it is very cold. In winter their fur is white, in summer it is brown. Their bushy tail keeps them warm like a scarf. |
| klasse3 | Der Schneefuchs heißt auch Polarfuchs und lebt im kalten Norden. Sein Fell ist so dicht, dass ihm selbst bei großer Kälte warm bleibt. Im Winter ist es weiß, im Sommer braun, damit er sich immer gut verstecken kann. Wenn er schläft, legt er seinen buschigen Schwanz wie einen Schal um die Nase. Er frisst gern Beeren und hört sogar leise Geräusche unter dem Schnee. | The snow fox is also called the arctic fox and lives in the cold north. Its fur is so thick that it stays warm even in great cold. In winter it is white, in summer brown, so that it can always hide well. When it sleeps, it wraps its bushy tail around its nose like a scarf. It likes to eat berries and can even hear quiet sounds under the snow. |

Mission complete (`mission-snow_fox-home`): *Super! Der Schneefuchs ist wieder zu Hause.* / *Great! The snow fox is home again.*

Math (`mathe4`): *Im Zoo gibt es 4 Körbe mit je 250 Beeren. Wie viele Beeren sind es zusammen?* /
*The zoo has 4 baskets with 250 berries each. How many berries are there in total?* → **1000** (reworded 2026-09-26: the old task (kiosk freezers) pointed at the kiosk)

---

## Night level 1 — hedgehog, bat, owl (GAME-NIGHT, GAME-LEVEL-NIGHT-1)

Night missions follow the same rules as the day missions (GAME-NIGHT rule 9; riddle and facts rules at the top of this
spec). Riddles use **night clues** — moonlight, sounds, smells (GAME-NIGHT rule 5). Places: GAME-LEVEL-NIGHT-1
"Hiding places". Fluent file: `assets/i18n/{de,en}/night.ftl` (same key scheme as `missions.ftl`).
Night riddles avoid every `kiga` place word of the day levels too (zoo-wide, as MISS-011), although the night
zoo is a riddle scope of its own for scenery kinds (*Q-136 answered*). The owl's `kiga` word *Teich* / *pond*
equals the hippo's (`loc_pond`) — allowed because the two riddles are never active at the same time (Q-136).

| # | Animal id | Food box (de / en) | Hiding place id | Place (for designers only) |
|---|---|---|---|---|
| N1 | `hedgehog` | Käfer / beetles | `loc_brush_pile`, `loc_flowerpots`, `loc_mushrooms` | heap of dry twigs and sticks in the corner where two hedges meet; rustling · potting bench with stacked clay flowerpots, white night-scented flowers (sweet smell at night) · ring of brown/cream mushrooms on moss at the foot of a big old tree; earthy smell |
| N2 | `bat` | Obst / fruit | `loc_windmill`, `loc_fireflies`, `loc_hollow_tree` | little wooden garden windmill, four sails turning, soft whirring · low meadow with many fireflies (the only firefly place, Q-115), small crooked tree · very thick old tree with a big round knothole at 2.5 m |
| N3 | `owl` | Käfer / beetles | `loc_moon_pond`, `loc_hilltop`, `loc_fir` | small still pond mirroring the moon and stars, reeds, owl on a wooden post at the shore · small grassy hill with a big round stone on top, no trees, brightest moonlight · the one tall dark pointed fir tree with cones |

**Night food storage** (`food_storage_n1`, Q-135 answered): four boxes — *Käfer* / *beetles* (hedgehog **and** owl:
two animals share one food, so the child takes a beetle box twice), *Obst* / *fruit* (bat), and two distractors for
later night levels, *Würmer* / *worms* and *Nektar* / *nectar*. Keys `food-beetles`, `food-fruit`, `food-worms`,
`food-nectar`. MISS-004 extends to all 14 food words (distinct).

### N1. Igel / Hedgehog — `loc_brush_pile`, `loc_flowerpots`, `loc_mushrooms`

Riddle — `loc_brush_pile` (heap of dry twigs and sticks in the corner where two hedges meet; rustling; keys `mission-hedgehog-riddle-loc_brush_pile-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 heap of dry twigs in the hedge corner · **Reisig** | 🖼 heap of dry twigs in the hedge corner · **twigs** |
| klasse1 | Ich bin klein und stachelig. Ich liege unter trockenen Ästen. Es raschelt leise. | I am small and spiky. I sleep under dry sticks. It rustles softly. |
| klasse2 | Der Igel hat sich in der Ecke an der Hecke versteckt, unter einem Haufen aus trockenen Ästen. Hör genau hin: Dort raschelt es leise. | The hedgehog is hiding in the corner by the hedge, under a heap of dry sticks. Listen closely: something rustles there softly. |
| klasse3 | Igel schlafen am Tag und werden erst in der Nacht munter. Unser Igel hat sich ein gemütliches Versteck gebaut: ganz hinten in der Ecke, wo zwei Hecken zusammenstoßen. Dort liegt ein großer Haufen aus trockenen Ästen und Stöcken. Wenn der Igel sich bewegt, knackt und raschelt es leise. | Hedgehogs sleep during the day and only wake up at night. Our hedgehog has built a cosy hiding spot: far back in the corner where two hedges meet. A big heap of dry sticks and branches lies there. When the hedgehog moves, it cracks and rustles softly. |

Place words: *Reisig*, *Reisighaufen* / *twigs*.

Riddle — `loc_flowerpots` (potting bench with stacked clay flowerpots, white night-scented flowers (sweet smell at night); keys `mission-hedgehog-riddle-loc_flowerpots-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 old clay flowerpots on and under a potting bench, white flowers · **Blumentöpfe** | 🖼 old clay flowerpots on and under a potting bench, white flowers · **flowerpots** |
| klasse1 | Ich schnuppere gern. Hier riecht es süß. Weiße Blumen blühen nachts. | I like to sniff. It smells sweet here. White flowers bloom at night. |
| klasse2 | Der Igel schnuppert an alten Töpfen aus Ton. Darin wachsen weiße Blumen, die nur in der Nacht so süß duften. | The hedgehog is sniffing at old clay pots. White flowers grow in them that only smell so sweet at night. |
| klasse3 | Am Tag riecht man hier fast nichts. Doch wenn es dunkel wird, öffnen sich kleine weiße Blumen und duften süß. Sie wachsen in alten Tontöpfen, die auf und unter einem Holztisch stehen. Zwischen den Töpfen schnuppert unser Igel mit seiner kleinen Nase. | During the day you can hardly smell anything here. But when it gets dark, little white flowers open and smell sweet. They grow in old clay pots that stand on and under a wooden table. Our hedgehog is sniffing between the pots with its little nose. |

Place words: *Blumentöpfe*, *Töpfe*, *Tontöpfe* / *flowerpots*, *pots*.

Riddle — `loc_mushrooms` (ring of brown/cream mushrooms on moss at the foot of a big old tree; earthy smell; keys `mission-hedgehog-riddle-loc_mushrooms-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 ring of brown mushrooms on green moss under a big old tree · **Pilze** | 🖼 ring of brown mushrooms on green moss under a big old tree · **mushrooms** |
| klasse1 | Der Boden ist weich. Es riecht nach Erde. Hier wächst grünes Moos. | The ground is soft. It smells of damp earth. Green moss grows here. |
| klasse2 | Der Igel sitzt auf weichem, grünem Moos unter einem alten, dicken Baum. Rund um ihn stehen kleine braune Hüte im Kreis, und es riecht nach feuchter Erde. | The hedgehog sits on soft green moss under a thick old tree. Little brown caps stand in a circle around it, and it smells of damp earth. |
| klasse3 | Unter einem großen alten Baum ist der Boden weich wie ein Kissen. Dort wächst grünes Moos, und es riecht nach feuchter Erde und altem Holz. Kleine braune und cremefarbene Hüte stehen im Kreis wie ein Ring. Mitten in diesem Ring hat sich unser Igel zusammengerollt. | Under a big old tree the ground is as soft as a pillow. Green moss grows there, and it smells of damp earth and old wood. Little brown and pale caps stand in a circle like a ring. Our hedgehog has curled up right in the middle of this ring. |

Place words: *Pilze*, *Moos* / *mushrooms*, *moss*.

Animal name (`animal-hedgehog`): *Igel* / *Hedgehog*. Heading above the facts (`animal-hedgehog-more`): *Mehr über den Igel* / *More about the hedgehog*.

**Facts** (keys `mission-hedgehog-facts-<reading_level>`; never a place word — ANIM-007 checks *Blumentöpfe*, *Moos*, *Pilze*, *Reisig*, *Reisighaufen*, *Tontöpfe*, *Töpfe* / *flowerpots*, *moss*, *mushrooms*, *pots*, *twigs*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 hedgehog spines · **Stacheln** | 🖼 hedgehog spines · **spines** |
| klasse1 | Ich habe viele Stacheln. Ich rolle mich ein. Nachts bin ich wach. | I have lots of spines. I curl up tight. I am awake at night. |
| klasse2 | Igel haben bis zu 8000 Stacheln auf dem Rücken. Bei Gefahr rollen sie sich ganz fest ein, dann sieht man nur noch Stacheln. Am Tag schlafen sie, in der Nacht suchen sie Futter. Sie fressen gern Käfer und Würmer. | Hedgehogs have up to 8,000 spines on their backs. When they are scared, they roll up tight so that only their spines show. They sleep during the day and look for food at night. They like to eat beetles and worms. |
| klasse3 | Ein Igel hat bis zu 8000 Stacheln. Das sind besondere Haare, die innen hohl und trotzdem sehr fest sind. Mit seiner feinen Nase findet er nachts Käfer, Würmer und Schnecken. Im Winter hält der Igel Winterschlaf und wacht erst im Frühling wieder auf. Wenn er Futter sucht, schnauft und schmatzt er so laut, dass man ihn hören kann. | A hedgehog has up to 8,000 spines. They are special hairs that are hollow inside but still very strong. With its fine nose it finds beetles, worms and snails at night. In winter the hedgehog hibernates and only wakes up again in spring. When it looks for food, it snuffles and smacks its lips so loudly that you can hear it. |

Mission complete (`mission-hedgehog-home`): *Super! Der Igel ist wieder zu Hause.* / *Great! The hedgehog is home again.*

Math (`mathe1`): *Der Igel findet 4 Käfer. Dann findet er noch 3. Wie viele Käfer sind es?* / *The hedgehog finds 4 beetles. Then it finds 3 more. How many beetles are there?* → **7**

### N2. Fledermaus / Bat — `loc_windmill`, `loc_fireflies`, `loc_hollow_tree`

Riddle — `loc_windmill` (little wooden garden windmill, four sails turning, soft whirring; keys `mission-bat-riddle-loc_windmill-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 little wooden windmill with four turning sails at night · **Windmühle** | 🖼 little wooden windmill with four turning sails at night · **windmill** |
| klasse1 | Ich hänge kopfüber. Große Flügel drehen sich. Es surrt leise. | I hang upside down. Big sails turn round. It whirs softly. |
| klasse2 | Die Fledermaus hängt kopfüber unter einem kleinen Holzhaus, auf dem sich vier große Flügel im Wind drehen. Man hört ein leises Surren. | The bat hangs upside down under a small wooden house with four big sails turning in the wind. You can hear a soft whirring. |
| klasse3 | Fledermäuse ruhen sich gern kopfüber aus. Unsere Fledermaus hat sich einen Platz gesucht, an dem sich etwas im Wind dreht. Vier große hölzerne Flügel gehen rundherum, immer im Kreis. Dabei surrt und knarrt es leise, und die Fledermaus schaukelt sanft mit. | Bats like to rest upside down. Our bat has found a place where something turns in the wind. Four big wooden sails go round and round, always in a circle. It whirs and creaks softly, and the bat sways gently along. |

Place words: *Windmühle*, *Mühle* / *windmill*, *mill*.

Riddle — `loc_fireflies` (low meadow with many fireflies (the only firefly place, Q-115), small crooked tree; keys `mission-bat-riddle-loc_fireflies-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 short meadow full of glowing fireflies, small crooked tree · **Glühwürmchen** | 🖼 short meadow full of glowing fireflies, small crooked tree · **fireflies** |
| klasse1 | Kleine Lichter tanzen hier. Ich hänge an einem Ast. Es blinkt überall. | Little lights dance here. I hang from a branch. It twinkles everywhere. |
| klasse2 | Die Fledermaus hängt an einem krummen kleinen Baum. Um sie herum tanzen viele kleine gelbe Lichter durch die Nacht. | The bat hangs from a small crooked tree. Lots of little yellow lights dance around it through the night. |
| klasse3 | Heute Nacht leuchtet es an einem Ort im Garten ganz besonders. Viele winzige Tierchen fliegen dort herum, und jedes hat ein kleines gelbes Licht am Bauch. Sie blinken und tanzen durch die Luft. Mitten darin hängt unsere Fledermaus kopfüber an einem krummen Baum und schaut dem Lichtertanz zu. | Tonight one place in the garden glows in a very special way. Lots of tiny creatures fly around there, and each one has a little yellow light on its tummy. They blink and dance through the air. Right in the middle our bat hangs upside down from a crooked tree and watches the dance of lights. |

Place words: *Glühwürmchen*, *Lichter* / *fireflies*, *lights*.

Riddle — `loc_hollow_tree` (very thick old tree with a big round knothole at 2.5 m; keys `mission-bat-riddle-loc_hollow_tree-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 very thick old tree with a big round knothole · **Astloch** | 🖼 very thick old tree with a big round knothole · **knothole** |
| klasse1 | Der Baum ist sehr dick. Er hat ein rundes Loch. Dort hänge ich. | The tree is very thick. It has a round hole. I hang there. |
| klasse2 | Die Fledermaus hängt an einem uralten, sehr dicken Baum. In seinem Stamm ist ein großes rundes Loch, wie ein kleines Fenster. | The bat hangs on a very old, very thick tree. In its trunk there is a big round hole, like a little window. |
| klasse3 | Dieser Baum ist der dickste und älteste im ganzen Garten. Drei Kinder könnten ihn zusammen kaum umarmen. Weit oben hat sein Stamm ein großes rundes Loch, dunkel und gemütlich. Genau davor hängt unsere Fledermaus kopfüber und hält sich mit ihren kleinen Krallen fest. | This tree is the thickest and oldest in the whole garden. Three children together could hardly hug it. High up, its trunk has a big round hole, dark and cosy. Our bat hangs upside down right in front of it, holding on with its little claws. |

Place words: *Astloch*, *Loch* / *knothole*, *hole*.

Animal name (`animal-bat`): *Fledermaus* / *Bat*. Heading above the facts (`animal-bat-more`): *Mehr über die Fledermaus* / *More about the bat*.

**Facts** (keys `mission-bat-facts-<reading_level>`; never a place word — ANIM-007 checks *Astloch*, *Glühwürmchen*, *Lichter*, *Loch*, *Mühle*, *Windmühle* / *fireflies*, *hole*, *knothole*, *lights*, *mill*, *windmill*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 bat with open wings · **Flügel** | 🖼 bat with open wings · **wings** |
| klasse1 | Ich kann fliegen. Ich hänge kopfüber. Ich esse gern Obst. | I can fly. I hang upside down. I like to eat fruit. |
| klasse2 | Fledermäuse sind die einzigen Säugetiere, die richtig fliegen können. Ihre Flügel sind aus dünner Haut zwischen langen Fingern. Unsere Fledermaus ist ein Flughund und frisst am liebsten süßes Obst. | Bats are the only mammals that can really fly. Their wings are made of thin skin between long fingers. Our bat is a fruit bat and loves to eat sweet fruit. |
| klasse3 | Fledermäuse schlafen am Tag kopfüber und halten sich dabei mit den Krallen ihrer Füße fest. Nachts wachen sie auf und fliegen los. Flughunde wie unsere Fledermaus haben große Augen und eine gute Nase, damit finden sie reifes Obst. Zum Schlafen wickeln sie sich in ihre Flügel ein wie in eine Decke. | Bats sleep upside down during the day and hold on with the claws of their feet. At night they wake up and fly off. Fruit bats like ours have big eyes and a good nose, which help them find ripe fruit. To sleep, they wrap themselves in their wings like in a blanket. |

Mission complete (`mission-bat-home`): *Super! Die Fledermaus ist wieder zu Hause.* / *Great! The bat is home again.*

Math (`mathe2`): *Im Nachtzoo hängen 3 Fledermäuse. Jede frisst 5 Stück Obst. Wie viele Stück Obst sind es zusammen?* / *3 bats hang in the night zoo. Each one eats 5 pieces of fruit. How many pieces of fruit is that altogether?* → **15**

### N3. Eule / Owl — `loc_moon_pond`, `loc_hilltop`, `loc_fir`

Riddle — `loc_moon_pond` (small still pond mirroring the moon and stars, reeds, owl on a wooden post at the shore; keys `mission-owl-riddle-loc_moon_pond-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 still pond at night with the moon and stars mirrored in it, reeds, a wooden post · **Teich** | 🖼 still pond at night with the moon and stars mirrored in it, reeds, a wooden post · **pond** |
| klasse1 | Der Mond schwimmt im Wasser. Ich sitze auf einem Pfahl. Das Schilf rauscht. | The moon floats on water. I sit on a post. The reeds rustle. |
| klasse2 | Die Eule sitzt auf einem Holzpfahl am Wasser. Unter ihr glitzern die Sterne, und der Mond scheint im stillen Wasser zu schwimmen. | The owl sits on a wooden post by the water. Below it the stars glitter, and the moon seems to swim in the still water. |
| klasse3 | Wenn die Luft ganz still ist, wird das Wasser glatt wie ein Spiegel. Dann sieht man darin den Mond und viele Sterne, als ob der Himmel auf dem Boden liegt. Am Ufer wiegt sich das Schilf, und ein kleiner Steg führt ein Stück hinaus. Auf einem Holzpfahl daneben sitzt unsere Eule und bewundert den Mond. | When the air is completely still, the water becomes as smooth as a mirror. Then you can see the moon and lots of stars in it, as if the sky were lying on the ground. Reeds sway on the bank, and a small jetty leads out a little way. Our owl sits on a wooden post next to it and admires the moon. |

Place words: *Teich*, *Wasser*, *Steg* / *pond*, *water*, *jetty*.

Riddle — `loc_hilltop` (small grassy hill with a big round stone on top, no trees, brightest moonlight; keys `mission-owl-riddle-loc_hilltop-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 small round grassy hill with a big round stone on top in bright moonlight · **Hügel** | 🖼 small round grassy hill with a big round stone on top in bright moonlight · **hill** |
| klasse1 | Ich sitze ganz oben. Hier ist kein Baum. Der Mond scheint hell. | I sit at the top. No tree grows here. The moon shines brightly. |
| klasse2 | Die Eule sitzt oben auf einem großen runden Stein. Dort stehen keine Bäume, darum scheint der Mond hier am hellsten. | The owl sits on top of a big round stone. No trees stand there, so the moon shines brightest here. |
| klasse3 | Eulen haben große Augen und sehen im Dunkeln sehr gut. Unsere Eule mag es aber heute besonders hell. Sie ist dorthin geflogen, wo der Boden ein wenig ansteigt und kein Baum den Mond verdeckt. Auf einem großen runden Stein ganz oben sitzt sie im silbernen Mondlicht. | Owls have big eyes and see very well in the dark. But tonight our owl likes it especially bright. It has flown to where the ground rises a little and no tree hides the moon. It sits right at the top on a big round stone in the silvery moonlight. |

Place words: *Hügel*, *Stein* / *hill*, *stone*.

Riddle — `loc_fir` (the one tall dark pointed fir tree with cones; keys `mission-owl-riddle-loc_fir-<reading_level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 tall dark pointed fir tree with cones, owl at the very top · **Tanne** | 🖼 tall dark pointed fir tree with cones, owl at the very top · **fir** |
| klasse1 | Ich sitze ganz hoch. Der Baum ist spitz. Er hat Zapfen. | I sit very high. The tree is pointed. It has cones. |
| klasse2 | Die Eule sitzt ganz oben in einem hohen, spitzen Baum mit dunklen Nadeln und Zapfen. Von dort ruft sie: Huhu! | The owl sits at the very top of a tall, pointed tree with dark needles and cones. From up there it calls: hoo-hoo! |
| klasse3 | Im Garten steht ein Baum, der auch im Winter grün bleibt. Er hat dunkle, spitze Nadeln, und an seinen Ästen hängen braune Zapfen. Seine Spitze zeigt wie ein Pfeil in den Himmel. Ganz oben auf dieser Spitze sitzt unsere Eule und ruft leise: Huhu! | In the garden there is a tree that stays green even in winter. It has dark, sharp needles, and brown cones hang from its branches. Its top points into the sky like an arrow. Our owl sits right up on this top and softly calls: hoo-hoo! |

Place words: *Tanne*, *Zapfen*, *Nadeln* / *fir*, *cones*, *needles*.

Animal name (`animal-owl`): *Eule* / *Owl*. Heading above the facts (`animal-owl-more`): *Mehr über die Eule* / *More about the owl*.

**Facts** (keys `mission-owl-facts-<reading_level>`; never a place word — ANIM-007 checks *Hügel*, *Nadeln*, *Steg*, *Stein*, *Tanne*, *Teich*, *Wasser*, *Zapfen* / *cones*, *fir*, *hill*, *jetty*, *needles*, *pond*, *stone*, *water*):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 owl face with big round eyes · **Augen** | 🖼 owl face with big round eyes · **eyes** |
| klasse1 | Ich habe große Augen. Ich fliege ganz leise. Ich rufe: Huhu! | I have big eyes. I fly very quietly. I call: hoo-hoo! |
| klasse2 | Eulen sehen und hören in der Nacht sehr gut. Sie können ihren Kopf weit nach hinten drehen. Ihre weichen Federn machen beim Fliegen fast kein Geräusch. | Owls see and hear very well at night. They can turn their heads far round to the back. Their soft feathers make almost no sound when they fly. |
| klasse3 | Eulen sind Nachtvögel. Mit ihren großen Augen sehen sie im Dunkeln viel besser als wir. Ihren Kopf können sie fast ganz nach hinten drehen, ohne den Körper zu bewegen. Weil ihre Federn so weich sind, fliegen sie fast lautlos. Unsere Eule frisst am liebsten Käfer. | Owls are night birds. With their big eyes they see much better in the dark than we do. They can turn their heads almost all the way to the back without moving their bodies. Because their feathers are so soft, they fly almost silently. Our owl likes to eat beetles best. |

Mission complete (`mission-owl-home`): *Super! Die Eule ist wieder zu Hause.* / *Great! The owl is home again.*

Math (`mathe3`): *Eine Eule ruft in einer Stunde 25-mal Huhu. Wie oft ruft sie in 4 Stunden?* / *An owl calls hoo-hoo 25 times in one hour. How many times does it call in 4 hours?* → **100**

### Night texts (GAME-NIGHT) and burglar texts (GAME-EVENTS)

Per reading level, keys `<key>-<reading_level>`; `kiga` is one word next to an icon/picture, read aloud.
Night-house sign `sign-night-house`: *Nachttierhaus* / *Night house*.

`night-dusk` — Nightfall, after the last day animal is home (GAME-NIGHT rule 1):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Nacht | night |
| klasse1 | Alle Tiere sind zu Hause. Es wird Nacht. | All animals are home. Night is falling. |
| klasse2 | Alle Tiere sind wieder zu Hause. Jetzt wird es dunkel, und die Laternen gehen an. | All the animals are home again. Now it is getting dark, and the lanterns light up. |
| klasse3 | Geschafft! Alle Tiere sind wieder zu Hause. Die Sonne geht unter, der Himmel wird dunkelblau, und überall gehen die Laternen an. Du kannst jetzt ins Bett gehen oder durch das Mondtor in den Nachtzoo. | You did it! All the animals are home again. The sun goes down, the sky turns deep blue, and lanterns light up everywhere. Now you can go to bed or walk through the moon door into the night zoo. |

`night-bed` — Reading panel at the bed (zookeeper house, NIGHT-003):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Bett | bed |
| klasse1 | Hier kannst du schlafen. Gute Nacht! | You can sleep here. Good night! |
| klasse2 | Bist du müde? Leg dich ins Bett. Morgen früh geht es im Zoo weiter. | Are you tired? Lie down in the bed. The zoo will be waiting in the morning. |
| klasse3 | Das Bett ist weich und warm, und durch das Fenster scheint der Mond. Wenn du dich hinlegst, schläfst du bis zum nächsten Morgen. Dein Spiel wird dabei gespeichert. | The bed is soft and warm, and the moon shines through the window. If you lie down, you will sleep until the next morning. Your game is saved while you sleep. |

`night-moon-door` — Reading panel at the moon door (level 1, GAME-NIGHT rule 3):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Mondtor | moon door |
| klasse1 | Das Mondtor ist offen! Komm mit! | The moon door is open! Come along! |
| klasse2 | Das Mondtor leuchtet. Dahinter wohnen Tiere, die nachts wach sind. | The moon door is glowing. Behind it live animals that are awake at night. |
| klasse3 | Hinter dem Mondtor liegt der Nachtzoo. Dort wohnen Tiere, die am Tag schlafen und in der Nacht wach sind. Doch auch sie sind heute ausgebüxt! | The night zoo lies behind the moon door. Animals live there that sleep during the day and are awake at night. But tonight they have run away too! |

`night-welcome` — First step into night_1 (cut-in):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Nachtzoo | night zoo |
| klasse1 | Willkommen im Nachtzoo! Wo sind die Tiere? | Welcome to the night zoo! Where are the animals? |
| klasse2 | Willkommen im Nachtzoo! Der Igel, die Fledermaus und die Eule sind weg. Lies die Schilder am Nachttierhaus. | Welcome to the night zoo! The hedgehog, the bat and the owl are gone. Read the boards at the night house. |
| klasse3 | Willkommen im Nachtzoo! Hier ist es still, nur die Grillen zirpen. Der Igel, die Fledermaus und die Eule sind aus dem Nachttierhaus ausgebüxt. Lies die Schilder vor dem Haus und nimm deine Laterne mit: Im Licht leuchten die Augen der Tiere. | Welcome to the night zoo! It is quiet here, only the crickets are chirping. The hedgehog, the bat and the owl have run away from the night house. Read the boards in front of the house and take your lantern: in its light the animals' eyes shine. |

`night-complete` — All night_1 animals home (GAME-NIGHT rule 7):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Bett | bed |
| klasse1 | Alle Nachttiere sind zu Hause. Zeit fürs Bett! | All night animals are home. Time for bed! |
| klasse2 | Toll! Alle Nachttiere sind wieder zu Hause. Geh jetzt ins Bett, morgen wartet ein neuer Teil des Zoos. | Well done! All the night animals are home again. Now go to bed; tomorrow a new part of the zoo is waiting. |
| klasse3 | Toll gemacht! Der Igel, die Fledermaus und die Eule sind wieder in ihrem Haus. Es ist schon spät, und die Sterne funkeln. Geh zurück durch das Mondtor und leg dich ins Bett. Morgen früh öffnet sich ein neuer Teil des Zoos. | Well done! The hedgehog, the bat and the owl are back in their house. It is late, and the stars are twinkling. Go back through the moon door and lie down in bed. Tomorrow morning a new part of the zoo will open. |

`night-morning` — Morning after sleeping (NIGHT-003):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Morgen | morning |
| klasse1 | Guten Morgen! Die Sonne scheint. | Good morning! The sun is shining. |
| klasse2 | Guten Morgen! Du hast gut geschlafen. Ein neuer Tag im Zoo beginnt. | Good morning! You slept well. A new day at the zoo begins. |
| klasse3 | Guten Morgen! Die Sonne scheint, die Vögel singen, und die Tiere in den Gehegen sind schon wach. Ein neuer Tag im Zoo beginnt. Schau nach, was es heute zu tun gibt! | Good morning! The sun is shining, the birds are singing, and the animals in their enclosures are already awake. A new day at the zoo begins. Let's see what there is to do today! |

`event-burglar-start` — Cut-in when the burglars are seen (GAME-EVENTS rules 2, 5):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Diebe | burglars |
| klasse1 | Pssst! Zwei Diebe sind im Zoo! | Shh! Two burglars are here! |
| klasse2 | Zwei Langfinger haben sich in den Zoo geschlichen und eine Futterkiste mitgenommen. Folge ihren Spuren! | Two sneaky burglars have crept into the zoo and taken a food box. Follow their footprints! |
| klasse3 | Hast du das gehört? Zwei tollpatschige Langfinger sind über die Mauer geklettert und haben eine Futterkiste mitgenommen. Folge ihren Fußspuren und leuchte sie mit deiner Laterne an, bevor sie wieder verschwinden! | Did you hear that? Two clumsy burglars have climbed over the wall and taken a food box. Follow their footprints and shine your lantern on them before they disappear again! |

`event-burglar-note-level_1` — Dropped note, level 1 (hideout between the west wall and the pond):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Mauer | wall |
| klasse1 | Wir warten an der Mauer. Hinter dem Teich. | We wait at the wall. Behind the pond. |
| klasse2 | Treffpunkt: an der großen Mauer, gleich hinter dem Teich mit den Seerosen. | Meeting point: at the big wall, just behind the pond with the water lilies. |
| klasse3 | Hallo Kumpel! Wir treffen uns heute Nacht an der großen Zoomauer. Geh am Teich mit den Fröschen vorbei, dann findest du uns hinter dem Schilf. Bring den Sack mit! | Hi pal! We'll meet tonight at the big zoo wall. Walk past the pond with the frogs, and you'll find us behind the reeds. Bring the sack! |

`event-burglar-note-level_2` — Dropped note, level 2 (hideout between the elephant enclosure and the east wall):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Elefant | elephant |
| klasse1 | Wir warten bei den Elefanten. An der Mauer. | We wait by the elephants. At the wall. |
| klasse2 | Treffpunkt: hinter dem Gehege der Elefanten, ganz nah an der Mauer. | Meeting point: behind the elephants' enclosure, right next to the wall. |
| klasse3 | Hallo Kumpel! Wir verstecken uns hinter dem großen Gehege mit den grauen Riesen. Dort, wo die Zoomauer ist, sieht uns keiner. Sei leise, sonst trompeten sie! | Hi pal! We are hiding behind the big enclosure with the grey giants. Nobody can see us there by the zoo wall. Be quiet, or they will trumpet! |

`event-burglar-note-level_3` — Dropped note, level 3 (hideout between the monkey enclosure and the north wall):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Affen | monkeys |
| klasse1 | Wir warten bei den Affen. An der Mauer. | We wait by the monkeys. At the wall. |
| klasse2 | Treffpunkt: hinter dem Affengehege, ganz oben an der Mauer. | Meeting point: behind the monkey enclosure, right up by the wall. |
| klasse3 | Hallo Kumpel! Wir warten hinter dem Gehege, in dem es tagsüber so laut kreischt und klettert. Ganz hinten an der Zoomauer ist unser Versteck. Pass auf, dass dich niemand sieht! | Hi pal! We are waiting behind the enclosure where it screeches and climbs so loudly during the day. Our hideout is right at the back by the zoo wall. Make sure nobody sees you! |

`event-burglar-caught` — Burglars caught in time (EVT-003):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Polizei | police |
| klasse1 | Erwischt! Die Polizei kommt. | Caught! The police are coming. |
| klasse2 | Erwischt! Die Diebe erschrecken sich, und die Polizei nimmt sie mit. Die Futterkiste ist wieder da. | Caught! The burglars get a fright, and the police take them away. The food box is back. |
| klasse3 | Erwischt! Im Licht deiner Laterne bleiben die zwei Langfinger wie angewurzelt stehen. Mit Blaulicht kommt die Polizei und nimmt sie mit. Die Futterkiste ist wieder da, und alle Tiere schlafen ruhig weiter. | Caught! In the light of your lantern the two burglars freeze on the spot. The police arrive with flashing blue lights and take them away. The food box is back, and all the animals sleep on peacefully. |

`event-burglar-late` — Time window over (EVT-004; GAME-EVENTS rule 7):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | Polizei | police |
| klasse1 | Die Polizei fängt sie trotzdem! | The police caught them anyway! |
| klasse2 | Die Diebe sind über die Mauer geflohen. Aber die Polizei hat sie trotzdem erwischt! Die Futterkiste steht am Eingang. | The burglars escaped over the wall. But the police caught them anyway! The food box is at the entrance. |
| klasse3 | Die zwei Langfinger sind schnell über die Mauer geklettert. Doch draußen hat die Polizei schon auf sie gewartet und sie trotzdem erwischt! Die Futterkiste haben sie fallen lassen, sie steht jetzt am Zooeingang. | The two burglars quickly climbed over the wall. But the police were already waiting outside and caught them anyway! They dropped the food box; it is now at the zoo entrance. |

---

## Behaviour

1. Each mission's texts are stored as Fluent keys:
   `mission-<animal>-riddle-<hiding_place>-<reading_level>` (one riddle per candidate hiding place,
   full place id, e.g. `mission-hippo-riddle-loc_mud-klasse2`), `food-<food_id>` (box
   label), `mission-<animal>-math`, `mission-<animal>-home` (mission complete).
   *Migration:* the PoC keys `mission-zebra-riddle-<reading_level>` (no place id) were deleted
   (2026-09-26, M5a) once zoo-core's `riddle_key` used the hiding place; all animals use the
   new scheme.
2. Every hiding place id must exist in the layout data (GAME-LAYOUT, tested by ANIM-004);
   the zoo-level-designer places all locations (representation: Q-044; level 1 candidates:
   `[[hiding_place]]` in `level-1.toml`, Q-080).
3. Math tasks here are fixed examples; the generator in CONT-MATH may produce variants of the
   same template.
4. The math task of a mission is the same for all its hiding places and never names or hints
   at a place (the zebra task was reworded for this on 2026-09-26).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MISS-001 | Given the 10 missions, then every one has riddles for `kiga`, `klasse1`, `klasse2`, `klasse3` in `de` and `en`. | unit |
| MISS-002 | *Retired — merged into READ-002 (covers all `klasse1` content incl. riddles).* | — |
| MISS-003 | *Retired — merged into RESC-011 (place word = the `kiga` word of the hiding place).* | — |
| MISS-004 | Given the 10 food words, then all are distinct (one box per food). | unit |
| MISS-005 | Given every math task, then the stored answer equals the computed result. | unit |
| MISS-006 | *Retired — merged into ANIM-004.* | — |
| MISS-007 | Given an animal with several candidate hiding places, then no `klasse1`–`klasse3` riddle of one candidate contains (whole word, case-insensitive, each word of a two-word label) the `kiga` place word of another candidate of the same animal, in the same language. | unit |
| MISS-008 | Given every `[[hiding_place]]` of every level and every reading level and language, then the key `mission-<animal>-riddle-<hiding_place>-<reading_level>` exists and is not empty (instance of RESC-003). | unit |
| MISS-010 | Given the goldfish mission, then the fish-bowl texts `mission-goldfish-needs-bowl`, `mission-goldfish-bowl-empty`, `mission-goldfish-bowl-filled`, `mission-goldfish-in-bowl` and `mission-goldfish-bowl-hint-<reading_level>` (4 reading levels) exist in `de` and `en`, and the `klasse1` hint has ≤ 5 words per sentence. | unit |
| MISS-011 | Given the riddles of levels 2 and 3, then no `klasse1`–`klasse3` riddle contains (whole word) the `kiga` word of a hiding place of another animal in the same or an earlier level (zoo-wide uniqueness, Q-083). | unit |
| MISS-012 | Given the night missions `hedgehog`, `bat`, `owl` (section "Night level 1"), then every `[[hiding_place]]` of `night-1.toml` has riddles for all four reading levels in `de` and `en` (`night.ftl`), facts, name, heading and home text exist, and `klasse1` sentences have ≤ 5 words (READ-002). | unit |
| MISS-013 | Given the night riddles, then none contains (whole word, each word of a two-word label) the `kiga` word of its own place, of another candidate of the same animal, of a place of another night animal, or of a day-level place (Q-083, Q-136); night facts contain no place word of their animal (ANIM-007). | unit |
| MISS-014 | Given the night texts `night-dusk`, `night-bed`, `night-moon-door`, `night-welcome`, `night-complete`, `night-morning` and the burglar texts `event-burglar-start`, `event-burglar-note-level_1/2/3`, `event-burglar-caught`, `event-burglar-late`, then each exists for all four reading levels in `de` and `en`. | unit |
| MISS-009 | Given the riddles of one animal's candidates, then a reviewer confirms that each riddle fits only its own place in the level (manual review with the level map, as Q-037). | manual |

## Open questions

- Q-002 Confirm the 10 animals. Q-036 answered: goldfish bowl, not a bucket (GAME-RESCUE).
- Night level 1 (Q-135, Q-136, Q-139 answered 2026-09-27): night foods (hedgehog and owl share *Käfer*), own riddle scope of night levels (owl `kiga` *Teich* = hippo's), burglar note places; teacher review of the night riddles with Q-037.
- Review by a primary school teacher for age-appropriate wording (Q-037).
- Q-039 Riddles containing their place word (koala, giraffe, elephant). Q-044 Hiding places in the layout.
- Q-064 Enclosure sign texts (`sign-<animal>`, picture on `kiga`?).
- Q-095 New hiding places and `kiga` words of levels 2–3 (sections 4–10); math tasks of koala, goldfish, monkey, giraffe and snow fox reworded so they no longer point at a place (Behaviour 4).
- Q-083 (answered) Two-word `kiga` place words (*bamboo forest*, *leaf pile*) and clashes with food words (*Bambus*, *Blätter* / *leaves*). Q-081 (answered) growing bamboo in the panda enclosure.
