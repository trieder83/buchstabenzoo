---
id: CONT-MISSIONS
title: Start missions — 10 animals
aspect: content
module: missions
status: draft
depends_on: [GAME-RESCUE, GAME-ANIMALS, GAME-FEED, CONT-READING, CONT-MATH, CONT-L10N]
test_prefix: MISS
updated: 2026-09-26
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
`klasse2` 3–4 sentences, `klasse3` 4–6 sentences. Keys `mission-<animal>-facts-<level>`,
name `animal-<animal>`, heading above the facts `animal-<animal>-more`.

**Several hiding places per animal** (discovery, user decision 2026-09-26, GAME-RESCUE §1):
an animal has ≥ 3 candidate hiding places in its level; one is picked per playthrough and the
info board shows **the riddle of the picked place**. Level 1 (zebra, hippo, panda) has all
candidates below; the other 7 animals still have one place each until their level is laid
out (then ≥ 3 each). Additional rules for candidates:
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
  forest*, *leaf pile*, as *ice cream*) are checked word by word (proposal, Q-083).

## Overview

| # | Animal id | Food box (de / en) | Hiding place id | Place (for designers only — never shown in riddles ≥ klasse1) |
|---|---|---|---|---|
| 1 | `zebra` | Gras / grass | `loc_river`, `loc_meadow`, `loc_sand` | river with a bridge and ducks · tall-grass meadow with wildflowers and butterflies · dry yellow sand patch (dust bath) |
| 2 | `hippo` | Melonen / melons | `loc_pond`, `loc_mud`, `loc_shade` | still pond with water lilies and frogs · brown mud puddle · shade under big trees at the zoo wall |
| 3 | `panda` | Bambus / bamboo | `loc_cave`, `loc_bamboo`, `loc_leaves` | dark, cool stone cave · bamboo thicket taller than the wall · raked pile of red and yellow leaves |
| 4 | `koala` | Eukalyptus / eucalyptus | `loc_tallest_tree` | tallest tree in the zoo |
| 5 | `elephant` | Heu / hay | `loc_mud_pool` | mud pool |
| 6 | `goldfish` | Fischfutter / fish food | river/stream places of its level (replaces `loc_fountain`, user decision 2026-09-26; riddles below are the old fountain texts and will be rewritten with its level) | river / stream; needs the fish bowl (GAME-RESCUE "goldfish bowl") |
| 7 | `monkey` | Bananen / bananas | `loc_pirate_ship` | pirate ship (mast, sail, flag, treasure chest) |
| 8 | `giraffe` | Blätter / leaves | `loc_playground` | playground with slide, swings, sandpit, trees |
| 9 | `lion` | Fleisch / meat | `loc_sun_rocks` | big flat rocks in the sun |
| 10 | `snow_fox` | Beeren / berries | `loc_ice_cream_kiosk` | ice cream kiosk with freezer |

The 10 food boxes together form the food storage; the other 9 boxes are the natural
distractors for each mission.

---

## 1. Zebra — `loc_river`, `loc_meadow`, `loc_sand`

Riddle — `loc_river` (keys `mission-zebra-riddle-loc_river-<level>`; the PoC keys `mission-zebra-riddle-<level>` hold the same texts until zoo-core uses the new scheme):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 river · **Fluss** | 🖼 river · **river** |
| klasse1 | Ich habe Durst. Ich suche fließendes Wasser. | I am thirsty. I look for running water. |
| klasse2 | Die Zebras haben großen Durst. Sie trinken dort, wo das Wasser fließt und Enten schwimmen. | The zebras are very thirsty. They drink where the water flows and ducks swim. |
| klasse3 | Die Zebras sind lange herumgerannt. Jetzt haben sie Durst. Sie suchen Wasser, das sich bewegt und rauscht. Dort, wo eine Brücke über das Wasser führt, trinken sie. | The zebras ran around for a long time. Now they are thirsty. They are looking for water that moves and rushes. They drink where a bridge crosses the water. |

(`klasse3` changed 2026-09-26: *über die Wiese* / *across the meadow* removed — the meadow is
now another zebra hiding place, `loc_meadow`.) Place words: *Fluss, Bach, Brücke, Enten,
Wasser* / *river, stream, bridge, ducks, water*.

Riddle — `loc_meadow` (keys `mission-zebra-riddle-loc_meadow-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 meadow with tall grass and flowers · **Wiese** | 🖼 meadow with tall grass and flowers · **meadow** |
| klasse1 | Ich habe Hunger. Das Gras ist hoch. Dort fliegen Schmetterlinge. | I am hungry. The grass is tall. Butterflies fly there. |
| klasse2 | Die Zebras haben Hunger. Sie fressen dort, wo das Gras hoch wächst, bunte Blumen blühen und Schmetterlinge fliegen. | The zebras are hungry. They eat where the grass grows tall, colourful flowers bloom and butterflies fly. |
| klasse3 | Die Zebras haben großen Hunger. Sie suchen einen Platz, an dem das Gras so hoch ist, dass es ihnen bis zum Bauch reicht. Zwischen den Halmen blühen bunte Blumen, und Schmetterlinge flattern umher. Hinter dem hohen Gras stehen große Bäume. | The zebras are very hungry. They are looking for a place where the grass is so tall that it reaches their bellies. Colourful flowers bloom between the blades, and butterflies flutter around. Big trees stand behind the tall grass. |

Place words: *Wiese, Blumen, Schmetterlinge* / *meadow, flowers, butterflies* (not *Gras* /
*grass* — that is the food word).

Riddle — `loc_sand` (keys `mission-zebra-riddle-loc_sand-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 yellow sand patch · **Sand** | 🖼 yellow sand patch · **sand** |
| klasse1 | Mein Fell juckt. Ich wälze mich gern. Der Boden ist gelb. | My fur itches. I like to roll. The ground is yellow. |
| klasse2 | Den Zebras juckt das Fell. Sie wälzen sich dort, wo der Boden gelb und trocken ist und kein Gras wächst. | The zebras' fur itches. They roll around where the ground is yellow and dry and no grass grows. |
| klasse3 | Zebras nehmen gern ein Staubbad. Dabei wälzen sie sich auf dem Rücken hin und her, bis ihr Fell ganz staubig ist. Das hilft gegen lästige Fliegen. Unsere Zebras haben einen Platz gefunden, an dem kein Grashalm wächst und der Boden gelb und warm ist. | Zebras like to take a dust bath. They roll on their backs until their fur is all dusty. It helps against annoying flies. Our zebras have found a place where not a single blade of grass grows and the ground is yellow and warm. |

Place words: *Sand, Staubbad* / *sand, dust*.

Animal name (`animal-zebra`, all reading levels): *Zebra* / *Zebra*. Heading above the facts
(`animal-zebra-more`): *Mehr über das Zebra* / *More about the zebra*.

**Facts** (*Steckbrief*, GAME-ANIMALS "Info board" item 4, keys `mission-zebra-facts-<level>`;
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

Riddle — `loc_pond` (keys `mission-hippo-riddle-loc_pond-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 pond · **Teich** | 🖼 pond · **pond** |
| klasse1 | Ich bade gern. Das Wasser ist still. Dort blühen Seerosen. | I like to bathe. The water is still. Water lilies grow there. |
| klasse2 | Das Flusspferd liegt gern im Wasser. Es mag stilles Wasser mit Seerosen und Fröschen. | The hippo likes to lie in water. It likes still water with water lilies and frogs. |
| klasse3 | Das Flusspferd wird in der Sonne schnell heiß. Darum ist es in ein Wasser gestiegen, das nicht fließt. Auf dem Wasser schwimmen Seerosen, und am Ufer quaken Frösche. Nur die Augen und Ohren schauen heraus. | The hippo gets hot quickly in the sun. So it climbed into water that does not flow. Water lilies float on top, and frogs croak on the shore. Only its eyes and ears stick out. |

Place words: *Teich, Seerosen, Frösche* / *pond, water lilies, frogs*.

Riddle — `loc_mud` (keys `mission-hippo-riddle-loc_mud-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 brown mud puddle · **Matsch** | 🖼 brown mud puddle · **mud** |
| klasse1 | Ich mag es nass. Der Boden ist braun. Es spritzt und schmatzt. | I like it wet. The ground is brown. It splashes and squelches. |
| klasse2 | Das Flusspferd liegt in einer großen, braunen Pfütze. Wenn es sich bewegt, spritzt und schmatzt es laut. | The hippo lies in a big, brown puddle. When it moves, it splashes and squelches loudly. |
| klasse3 | Heute Nacht hat es geregnet. Jetzt gibt es im Zoo eine Stelle, an der der Boden ganz weich und braun ist. Das Flusspferd liebt so etwas: Es wälzt sich darin, bis sein Rücken braun ist wie Schokolade. Such dort, wo es schmatzt und spritzt. | It rained last night. Now there is a spot in the zoo where the ground is soft and brown. The hippo loves that: it rolls around until its back is as brown as chocolate. Look where it squelches and splashes. |

Place words: *Matsch, Pfütze, Schlamm* / *mud, puddle*.

Riddle — `loc_shade` (keys `mission-hippo-riddle-loc_shade-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 deep shade under big trees by a wall · **Schatten** | 🖼 deep shade under big trees by a wall · **shade** |
| klasse1 | Mir ist zu heiß. Ich liege unter Bäumen. Hinter mir ist eine Mauer. | I am too hot. I lie under trees. Behind me is a wall. |
| klasse2 | Dem Flusspferd ist die Sonne zu heiß. Es liegt dort, wo große Bäume die Sonne verdecken, direkt an der hohen Mauer des Zoos. | The sun is too hot for the hippo. It lies where big trees hide the sun, right next to the high zoo wall. |
| klasse3 | Flusspferde haben eine empfindliche Haut und mögen keine pralle Sonne. Darum hat sich unser Flusspferd einen dunklen, kühlen Platz gesucht. Dicke Baumkronen halten die Sonne ab, und hinter ihm steht die hohe Mauer, die den ganzen Zoo umgibt. Dort liegt es im trockenen Gras und döst. | Hippos have sensitive skin and do not like strong sun. So our hippo has looked for a dark, cool spot. Thick treetops keep the sun away, and behind it stands the high wall around the whole zoo. It lies there in the dry grass and dozes. |

Place words: *Schatten, Mauer* / *shade, wall*.

Math (`mathe2`): *Jedes Flusspferd frisst 4 Melonen. Es gibt 3 Flusspferde. Wie viele Melonen brauchst du?* /
*Each hippo eats 4 melons. There are 3 hippos. How many melons do you need?* → **12**
(herd size depends on Q-004 / Q-030)

## 3. Panda — `loc_cave`, `loc_bamboo`, `loc_leaves`

Riddle — `loc_cave` (keys `mission-panda-riddle-loc_cave-<level>`):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 cave · **Höhle** | 🖼 cave · **cave** |
| klasse1 | Ich bin müde. Dort ist es dunkel. Dort ist es kühl. | I am tired. It is dark there. It is cool there. |
| klasse2 | Der Panda ist müde. Er schläft an einem dunklen, kühlen Ort mit Wänden aus Stein. | The panda is tired. It sleeps in a dark, cool place with walls of stone. |
| klasse3 | Der Panda hat den ganzen Morgen gefressen. Jetzt ist er sehr müde. Er hat einen Platz gefunden, an dem es dunkel und kühl ist. Die Wände sind aus Stein, und wenn man ruft, hallt es zurück. | The panda ate all morning. Now it is very tired. It found a place that is dark and cool. The walls are made of stone, and when you shout, it echoes. |

Place words: *Höhle, Stein, Echo* / *cave, stone, echo*.

Riddle — `loc_bamboo` (keys `mission-panda-riddle-loc_bamboo-<level>`). The `kiga` word is
*Bambuswald*, not *Bambus*, because *Bambus* is the panda's food word (it must stay allowed
in the facts, ANIM-007):

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 tall bamboo thicket · **Bambuswald** | 🖼 tall bamboo thicket · **bamboo forest** |
| klasse1 | Ich habe Hunger. Hier wachsen grüne Stangen. Sie sind sehr hoch. | I am hungry. Green stalks grow here. They are very tall. |
| klasse2 | Der Panda hat Hunger. Er sitzt dort, wo grüne Stangen dicht an dicht wachsen, höher als die Mauer des Zoos. | The panda is hungry. It sits where green stalks grow close together, taller than the zoo wall. |
| klasse3 | Pandas fressen fast den ganzen Tag. Unser Panda hat einen Platz gefunden, an dem sein Lieblingsessen einfach aus dem Boden wächst. Die grünen Stangen stehen so dicht, dass man kaum hindurchsehen kann, und sie sind höher als die Mauer. Wenn der Wind weht, klappern sie leise. | Pandas eat almost all day long. Our panda has found a place where its favourite food simply grows out of the ground. The green stalks stand so close together that you can hardly see through them, and they are taller than the wall. When the wind blows, they rattle softly. |

Place words: *Bambuswald, Dickicht* / *bamboo forest, thicket* (not *Bambus* / *bamboo* — food word).

Riddle — `loc_leaves` (keys `mission-panda-riddle-loc_leaves-<level>`). The `kiga` word is
*Laubhaufen* / *leaf pile*, not *Blätter* / *leaves*, which is the giraffe's food word:

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 big pile of red and yellow leaves · **Laubhaufen** | 🖼 big pile of red and yellow leaves · **leaf pile** |
| klasse1 | Ich spiele gern. Mein Bett ist weich. Rotes und gelbes Laub! | I like to play. My bed is soft. Red and yellow leaves! |
| klasse2 | Der Panda spielt gern. Er hat sich in einen großen, weichen Haufen aus roten und gelben Blättern gekuschelt. | The panda likes to play. It has snuggled into a big, soft heap of red and yellow leaves. |
| klasse3 | Der Zoowärter hat unter den Bäumen fleißig Blätter zusammengeharkt. Jetzt liegt dort ein riesiger Haufen aus roten, gelben und braunen Blättern. Der Panda hat ihn entdeckt und sich mitten hineingeworfen. Wenn er sich dreht, raschelt es, und Blätter fliegen durch die Luft. | The zookeeper has been busy raking leaves under the trees. Now there is a huge heap of red, yellow and brown leaves. The panda found it and jumped right into the middle. When it turns over, the leaves rustle and fly through the air. |

Place words: *Laubhaufen, Laub, Haufen* / *leaf pile, heap*.

Math (`mathe1`): *Der Panda will 10 Stunden schlafen. 6 Stunden sind schon vorbei. Wie viele Stunden noch?* /
*The panda wants to sleep 10 hours. 6 hours have passed. How many hours are left?* → **4**

## 4. Koala — `loc_tallest_tree`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 tall tree · **Baum** | 🖼 tall tree · **tree** |
| klasse1 | Ich klettere gern. Ich sitze ganz oben. Kein Baum ist höher. | I like to climb. I sit at the top. No tree is higher. |
| klasse2 | Der Koala sitzt ganz oben auf dem höchsten Baum im Zoo. Dort schläft er fast den ganzen Tag. | The koala sits at the very top of the tallest tree in the zoo. It sleeps there almost all day. |
| klasse3 | Koalas schlafen bis zu zwanzig Stunden am Tag. Unser Koala hat sich den höchsten Baum im ganzen Zoo ausgesucht. Er sitzt ganz oben in einer Astgabel. Von dort kann er sogar das Eingangstor sehen. | Koalas sleep up to twenty hours a day. Our koala picked the tallest tree in the whole zoo. It sits at the very top in a fork of branches. From there it can even see the entrance gate. |

Math (`mathe3`): *Der Baum ist 24 Meter hoch. Der Koala sitzt 6 Meter unter der Spitze. In welcher Höhe sitzt er?* /
*The tree is 24 metres tall. The koala sits 6 metres below the top. How high up is it?* → **18**

## 5. Elephant — `loc_mud_pool`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 mud · **Matsch** | 🖼 mud · **mud** |
| klasse1 | Ich bin groß und grau. Ich mag es nass. Braun und matschig! | I am big and grey. I like it wet. Brown and muddy! |
| klasse2 | Der Elefant wälzt sich gern in brauner, nasser Erde. Das schützt seine Haut vor der Sonne. | The elephant likes to roll in brown, wet earth. It protects its skin from the sun. |
| klasse3 | Elefanten haben keine Sonnencreme. Darum schützen sie ihre Haut auf eine andere Art. Sie wälzen sich in nasser, brauner Erde, bis sie ganz dreckig sind. Such dort, wo es am meisten spritzt und schmatzt. | Elephants have no sun cream. So they protect their skin in another way. They roll in wet, brown earth until they are completely dirty. Look where it splashes and squelches the most. |

Math (`mathe3`): *Ein Elefant frisst am Tag 100 kg Heu. Wie viel frisst er in 3 Tagen?* /
*An elephant eats 100 kg of hay a day. How much does it eat in 3 days?* → **300 kg**

## 6. Goldfish — `loc_fountain`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 fountain · **Brunnen** | 🖼 fountain · **fountain** |
| klasse1 | Ich schwimme im Kreis. Das Wasser springt hoch. Unten liegen Münzen. | I swim in circles. The water jumps up high. Coins lie at the bottom. |
| klasse2 | Der Goldfisch schwimmt dort, wo das Wasser in die Luft springt. Besucher werfen dort Münzen hinein. | The goldfish swims where the water jumps into the air. Visitors throw coins in there. |
| klasse3 | Der kleine Goldfisch ist in ein Becken aus Stein gesprungen. In der Mitte spritzt Wasser hoch in die Luft und plätschert wieder herunter. Auf dem Boden glänzen Münzen, denn viele Besucher wünschen sich hier etwas. | The little goldfish jumped into a stone basin. In the middle, water sprays high into the air and splashes back down. Coins shine on the bottom, because many visitors make a wish here. |

Math (`mathe1`): *Im Wasser liegen 7 Münzen. 3 sind golden. Wie viele sind nicht golden?* /
*There are 7 coins in the water. 3 are golden. How many are not golden?* → **4**

Special: a fish cannot follow over land — the player needs a bucket (Q-036).

## 7. Monkey — `loc_pirate_ship`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 ship · **Schiff** | 🖼 ship · **ship** |
| klasse1 | Ahoi! Ich bin jetzt Pirat. Ich sitze am Mast. | Ahoy! I am a pirate now. I sit on the mast. |
| klasse2 | Der Affe spielt jetzt Pirat. Er sitzt ganz oben am Mast und sucht nach einem Schatz. | The monkey is playing pirate now. It sits at the top of the mast and looks for treasure. |
| klasse3 | Der freche Affe hat ein neues Zuhause gefunden – glaubt er jedenfalls. Es hat ein großes Segel, eine Flagge mit einem Totenkopf und eine Schatzkiste. Er ruft „Ahoi!“ und klettert ganz nach oben. | The cheeky monkey has found a new home – or so it thinks. It has a big sail, a flag with a skull and a treasure chest. It shouts "Ahoy!" and climbs all the way up. |

Math (`mathe2`): *Das Schloss der Schatzkiste öffnet sich mit der Zahl 25 + 17.* /
*The treasure chest lock opens with the number 25 + 17.* → **42**

## 8. Giraffe — `loc_playground`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 playground · **Rutsche** | 🖼 playground · **slide** |
| klasse1 | Mein Hals ist lang. Hier rutschen Kinder. Hier schaukeln Kinder. | My neck is long. Children slide here. Children swing here. |
| klasse2 | Die Giraffe steht dort, wo Kinder rutschen und schaukeln. Sie ist viel größer als die Rutsche. | The giraffe stands where children slide and swing. It is much taller than the slide. |
| klasse3 | Die Giraffe ist sehr neugierig. Sie hat Kinder lachen hören und ist dem Lachen gefolgt. Jetzt steht sie zwischen Schaukeln, Rutsche und Sandkasten. Mit ihrer langen Zunge zupft sie Blätter von den Bäumen. | The giraffe is very curious. It heard children laughing and followed the sound. Now it stands between the swings, the slide and the sandpit. With its long tongue it pulls leaves off the trees. |

Math (`mathe4`): *Die Giraffe ist 5 m groß. Die Rutsche ist 180 cm hoch. Wie viele Zentimeter ist die Giraffe größer?* /
*The giraffe is 5 m tall. The slide is 180 cm high. How many centimetres taller is the giraffe?* → **320**

## 9. Lion — `loc_sun_rocks`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 rocks in sun · **Felsen** | 🖼 rocks in sun · **rocks** |
| klasse1 | Ich bin faul. Die Sonne ist warm. Ich liege auf Stein. | I am lazy. The sun is warm. I lie on stone. |
| klasse2 | Der Löwe liegt faul auf großen, warmen Steinen in der Sonne. Er gähnt und schläft fast ein. | The lion lies lazily on big, warm stones in the sun. It yawns and almost falls asleep. |
| klasse3 | Der Löwe ist der König der Tiere, aber heute ist er vor allem faul. Er hat sich den wärmsten Platz im Zoo gesucht. Dort liegen große, flache Steine, auf die den ganzen Tag die Sonne scheint. Er streckt sich aus und gähnt laut. | The lion is the king of animals, but today it is mostly lazy. It found the warmest spot in the zoo. There are big, flat stones there that the sun shines on all day. It stretches out and yawns loudly. |

Math (`mathe5`): *Der Löwe schläft ¾ von 24 Stunden. Wie viele Stunden sind das?* /
*The lion sleeps ¾ of 24 hours. How many hours is that?* → **18**

## 10. Snow fox — `loc_ice_cream_kiosk`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 ice cream kiosk · **Eis** | 🖼 ice cream kiosk · **ice cream** |
| klasse1 | Mir ist zu warm. Ich mag es kalt. Hier gibt es Waffeln. | I am too warm. I like it cold. There are waffles here. |
| klasse2 | Dem Schneefuchs ist es viel zu warm. Er sitzt dort, wo Kinder kalte, süße Kugeln in der Waffel kaufen. | The snow fox is much too warm. It sits where children buy cold, sweet scoops in a cone. |
| klasse3 | Der Schneefuchs kommt aus einem Land voller Schnee. Im Zoo ist es ihm heute viel zu warm. Er hat einen Ort gefunden, an dem kalte Luft aus einer Truhe weht. Dort kaufen Kinder Waffeln mit bunten Kugeln. | The snow fox comes from a land full of snow. Today the zoo is much too warm for it. It found a place where cold air blows out of a chest. Children buy cones with colourful scoops there. |

Math (`mathe4`): *Im Kiosk stehen 4 Truhen mit je 250 Eis. Wie viele Eis sind es zusammen?* /
*The kiosk has 4 freezers with 250 ice creams each. How many ice creams in total?* → **1000**

---

## Behaviour

1. Each mission's texts are stored as Fluent keys:
   `mission-<animal>-riddle-<hiding_place>-<level>` (one riddle per candidate hiding place,
   full place id, e.g. `mission-hippo-riddle-loc_mud-klasse2`), `food-<food_id>` (box
   label), `mission-<animal>-math`, `mission-<animal>-home` (mission complete).
   *Migration:* the PoC keys `mission-zebra-riddle-<level>` (no place id) stay in the `.ftl`
   files with the `loc_river` texts until zoo-core's `riddle_key` uses the hiding place; then
   they are deleted (keys of the other 7 animals are written in the new scheme directly).
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
| MISS-008 | Given every `[[hiding_place]]` of every level and every reading level and language, then the key `mission-<animal>-riddle-<hiding_place>-<level>` exists and is not empty (instance of RESC-003). | unit |
| MISS-009 | Given the riddles of one animal's candidates, then a reviewer confirms that each riddle fits only its own place in the level (manual review with the level map, as Q-037). | manual |

## Open questions

- Q-002 Confirm the 10 animals. Q-036 Goldfish transport (bucket?).
- Review by a primary school teacher for age-appropriate wording (Q-037).
- Q-039 Riddles containing their place word (koala, giraffe, elephant). Q-044 Hiding places in the layout.
- Q-064 Enclosure sign texts (`sign-<animal>`, picture on `kiga`?).
- Q-083 Two-word `kiga` place words (*bamboo forest*, *leaf pile*) and clashes with food words (*Bambus*, *Blätter* / *leaves*). Q-081 growing bamboo in the panda enclosure.
