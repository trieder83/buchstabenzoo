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

In this first version each animal has **one** hiding place; more places per animal later
(GAME-RESCUE §1 random choice).

## Overview

| # | Animal id | Food box (de / en) | Hiding place id | Place (for designers only — never shown in riddles ≥ klasse1) |
|---|---|---|---|---|
| 1 | `zebra` | Gras / grass | `loc_river` | river with a bridge and ducks |
| 2 | `hippo` | Melonen / melons | `loc_pond` | still pond with water lilies and frogs |
| 3 | `panda` | Bambus / bamboo | `loc_cave` | dark, cool stone cave |
| 4 | `koala` | Eukalyptus / eucalyptus | `loc_tallest_tree` | tallest tree in the zoo |
| 5 | `elephant` | Heu / hay | `loc_mud_pool` | mud pool |
| 6 | `goldfish` | Fischfutter / fish food | `loc_fountain` | fountain with coins |
| 7 | `monkey` | Bananen / bananas | `loc_pirate_ship` | pirate ship (mast, sail, flag, treasure chest) |
| 8 | `giraffe` | Blätter / leaves | `loc_playground` | playground with slide, swings, sandpit, trees |
| 9 | `lion` | Fleisch / meat | `loc_sun_rocks` | big flat rocks in the sun |
| 10 | `snow_fox` | Beeren / berries | `loc_ice_cream_kiosk` | ice cream kiosk with freezer |

The 10 food boxes together form the food storage; the other 9 boxes are the natural
distractors for each mission.

---

## 1. Zebra — `loc_river`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 river · **Fluss** | 🖼 river · **river** |
| klasse1 | Ich habe Durst. Ich suche fließendes Wasser. | I am thirsty. I look for running water. |
| klasse2 | Die Zebras haben großen Durst. Sie trinken dort, wo das Wasser fließt und Enten schwimmen. | The zebras are very thirsty. They drink where the water flows and ducks swim. |
| klasse3 | Die Zebras sind lange über die Wiese gerannt. Jetzt haben sie Durst. Sie suchen Wasser, das sich bewegt und rauscht. Dort, wo eine Brücke über das Wasser führt, trinken sie. | The zebras ran across the meadow for a long time. Now they are thirsty. They are looking for water that moves and rushes. They drink where a bridge crosses the water. |

Mission complete (`mission-zebra-home`, all reading levels, PoC M4): *Super! Die Zebras sind
wieder zu Hause.* / *Great! The zebras are home again.*

Math (`mathe1`): *Am Wasser trinken 3 Zebras. 2 kommen dazu. Wie viele Zebras sind es?* /
*3 zebras are drinking. 2 more come. How many zebras are there?* → **5**

## 2. Hippo — `loc_pond`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 pond · **Teich** | 🖼 pond · **pond** |
| klasse1 | Ich bade gern. Das Wasser ist still. Dort blühen Seerosen. | I like to bathe. The water is still. Water lilies grow there. |
| klasse2 | Das Flusspferd liegt gern im Wasser. Es mag stilles Wasser mit Seerosen und Fröschen. | The hippo likes to lie in water. It likes still water with water lilies and frogs. |
| klasse3 | Das Flusspferd wird in der Sonne schnell heiß. Darum ist es in ein Wasser gestiegen, das nicht fließt. Auf dem Wasser schwimmen Seerosen, und am Ufer quaken Frösche. Nur die Augen und Ohren schauen heraus. | The hippo gets hot quickly in the sun. So it climbed into water that does not flow. Water lilies float on top, and frogs croak on the shore. Only its eyes and ears stick out. |

Math (`mathe2`): *Jedes Flusspferd frisst 4 Melonen. Es gibt 3 Flusspferde. Wie viele Melonen brauchst du?* /
*Each hippo eats 4 melons. There are 3 hippos. How many melons do you need?* → **12**
(herd size depends on Q-004 / Q-030)

## 3. Panda — `loc_cave`

| Reading level | Deutsch | English |
|---|---|---|
| kiga | 🖼 cave · **Höhle** | 🖼 cave · **cave** |
| klasse1 | Ich bin müde. Dort ist es dunkel. Dort ist es kühl. | I am tired. It is dark there. It is cool there. |
| klasse2 | Der Panda ist müde. Er schläft an einem dunklen, kühlen Ort mit Wänden aus Stein. | The panda is tired. It sleeps in a dark, cool place with walls of stone. |
| klasse3 | Der Panda hat den ganzen Morgen gefressen. Jetzt ist er sehr müde. Er hat einen Platz gefunden, an dem es dunkel und kühl ist. Die Wände sind aus Stein, und wenn man ruft, hallt es zurück. | The panda ate all morning. Now it is very tired. It found a place that is dark and cool. The walls are made of stone, and when you shout, it echoes. |

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

1. Each mission's texts are stored as Fluent keys: `mission-<animal>-riddle-<level>`,
   `food-<food_id>` (box label), `mission-<animal>-math`, `mission-<animal>-home`
   (mission complete).
2. Every hiding place id must exist in the layout data (GAME-LAYOUT, tested by ANIM-004);
   the zoo-level-designer places all 10 locations (representation: Q-044).
3. Math tasks here are fixed examples; the generator in CONT-MATH may produce variants of the
   same template.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MISS-001 | Given the 10 missions, then every one has riddles for `kiga`, `klasse1`, `klasse2`, `klasse3` in `de` and `en`. | unit |
| MISS-002 | *Retired — merged into READ-002 (covers all `klasse1` content incl. riddles).* | — |
| MISS-003 | *Retired — merged into RESC-011 (place word = the `kiga` word of the hiding place).* | — |
| MISS-004 | Given the 10 food words, then all are distinct (one box per food). | unit |
| MISS-005 | Given every math task, then the stored answer equals the computed result. | unit |
| MISS-006 | *Retired — merged into ANIM-004.* | — |

## Open questions

- Q-002 Confirm the 10 animals. Q-036 Goldfish transport (bucket?).
- Review by a primary school teacher for age-appropriate wording (Q-037).
- Q-039 Riddles containing their place word (koala, giraffe, elephant). Q-044 Hiding places in the layout.
- Q-064 Enclosure sign texts (`sign-<animal>`, picture on `kiga`?).
