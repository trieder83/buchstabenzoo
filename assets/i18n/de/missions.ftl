# Missions and food box labels — German (reference locale). Source: CONT-MISSIONS
# (specs/20-content/missions/start-missions.md). Keys: mission-<animal>-riddle-<hiding_place>-<reading_level>
# (one riddle per candidate hiding place, GAME-RESCUE §1), food-<food_id>. Keep en/missions.ftl in sync (L10N-001/002).

## Food box labels (word printed on the box, also used on the info boards)

food-grass = Gras
food-melons = Melonen
food-bamboo = Bambus
food-eucalyptus = Eukalyptus
food-hay = Heu
food-fish_food = Fischfutter
food-bananas = Bananen
food-leaves = Blätter
food-meat = Fleisch
food-berries = Beeren

## 1. Zebra — loc_river, loc_meadow, loc_sand

# Info board heading (level-independent)
animal-zebra = Zebra
# heading above the facts
animal-zebra-more = Mehr über das Zebra

# Facts on the info board (Steckbrief, GAME-ANIMALS "Info board" 4), shown after riddle and
# food word — never a place word.
# kiga: shown next to a picture of zebra stripes
mission-zebra-facts-kiga = Streifen
mission-zebra-facts-klasse1 = Ich habe Streifen. Ich lebe in Afrika. Ich renne sehr schnell.
mission-zebra-facts-klasse2 = Zebras leben in großen Herden in Afrika. Jedes Zebra hat andere Streifen. Kein Muster gibt es zweimal. Zebras können sehr schnell rennen.
mission-zebra-facts-klasse3 = Zebras kommen aus Afrika und leben dort in großen Herden. Ihre Streifen sind so einmalig wie dein Fingerabdruck: Kein Zebra sieht genauso aus wie ein anderes. Den ganzen Tag fressen Zebras Gras. Kommt ein Löwe, rennen sie davon, so schnell wie ein Auto in der Stadt. Ein Zebrafohlen kann schon kurz nach der Geburt laufen.

# Location riddle — LEGACY PoC keys without the hiding place (= loc_river texts). Used by the
# current zoo-core riddle_key(); delete when it switches to
# mission-<animal>-riddle-<hiding_place>-<reading_level> (CONT-MISSIONS Behaviour 1).

# kiga: shown next to a picture of the river
mission-zebra-riddle-kiga = Fluss
mission-zebra-riddle-klasse1 = Ich habe Durst. Ich suche fließendes Wasser.
mission-zebra-riddle-klasse2 = Die Zebras haben großen Durst. Sie trinken dort, wo das Wasser fließt und Enten schwimmen.
mission-zebra-riddle-klasse3 = Die Zebras sind lange herumgerannt. Jetzt haben sie Durst. Sie suchen Wasser, das sich bewegt und rauscht. Dort, wo eine Brücke über das Wasser führt, trinken sie.

# Location riddles per candidate hiding place (level 1, GAME-LEVEL-1 "Hiding places").
# kiga: shown next to the picture of the place (picture id = hiding place id).

# loc_river — river with bridge and ducks
mission-zebra-riddle-loc_river-kiga = Fluss
mission-zebra-riddle-loc_river-klasse1 = Ich habe Durst. Ich suche fließendes Wasser.
mission-zebra-riddle-loc_river-klasse2 = Die Zebras haben großen Durst. Sie trinken dort, wo das Wasser fließt und Enten schwimmen.
mission-zebra-riddle-loc_river-klasse3 = Die Zebras sind lange herumgerannt. Jetzt haben sie Durst. Sie suchen Wasser, das sich bewegt und rauscht. Dort, wo eine Brücke über das Wasser führt, trinken sie.

# loc_meadow — tall grass, wildflowers, butterflies
mission-zebra-riddle-loc_meadow-kiga = Wiese
mission-zebra-riddle-loc_meadow-klasse1 = Ich habe Hunger. Das Gras ist hoch. Dort fliegen Schmetterlinge.
mission-zebra-riddle-loc_meadow-klasse2 = Die Zebras haben Hunger. Sie fressen dort, wo das Gras hoch wächst, bunte Blumen blühen und Schmetterlinge fliegen.
mission-zebra-riddle-loc_meadow-klasse3 = Die Zebras haben großen Hunger. Sie suchen einen Platz, an dem das Gras so hoch ist, dass es ihnen bis zum Bauch reicht. Zwischen den Halmen blühen bunte Blumen, und Schmetterlinge flattern umher. Hinter dem hohen Gras stehen große Bäume.

# loc_sand — dry yellow sand patch (dust bath)
mission-zebra-riddle-loc_sand-kiga = Sand
mission-zebra-riddle-loc_sand-klasse1 = Mein Fell juckt. Ich wälze mich gern. Der Boden ist gelb.
mission-zebra-riddle-loc_sand-klasse2 = Den Zebras juckt das Fell. Sie wälzen sich dort, wo der Boden gelb und trocken ist und kein Gras wächst.
mission-zebra-riddle-loc_sand-klasse3 = Zebras nehmen gern ein Staubbad. Dabei wälzen sie sich auf dem Rücken hin und her, bis ihr Fell ganz staubig ist. Das hilft gegen lästige Fliegen. Unsere Zebras haben einen Platz gefunden, an dem kein Grashalm wächst und der Boden gelb und warm ist.

# shown when the zebras are back in their enclosure (mission complete, GAME-RESCUE §8)
mission-zebra-home = Super! Die Zebras sind wieder zu Hause.

## 2. Hippo — loc_pond, loc_mud, loc_shade (location riddles only; other hippo texts: Q-069)

# loc_pond — still pond, water lilies, frogs
mission-hippo-riddle-loc_pond-kiga = Teich
mission-hippo-riddle-loc_pond-klasse1 = Ich bade gern. Das Wasser ist still. Dort blühen Seerosen.
mission-hippo-riddle-loc_pond-klasse2 = Das Flusspferd liegt gern im Wasser. Es mag stilles Wasser mit Seerosen und Fröschen.
mission-hippo-riddle-loc_pond-klasse3 = Das Flusspferd wird in der Sonne schnell heiß. Darum ist es in ein Wasser gestiegen, das nicht fließt. Auf dem Wasser schwimmen Seerosen, und am Ufer quaken Frösche. Nur die Augen und Ohren schauen heraus.

# loc_mud — brown mud puddle
mission-hippo-riddle-loc_mud-kiga = Matsch
mission-hippo-riddle-loc_mud-klasse1 = Ich mag es nass. Der Boden ist braun. Es spritzt und schmatzt.
mission-hippo-riddle-loc_mud-klasse2 = Das Flusspferd liegt in einer großen, braunen Pfütze. Wenn es sich bewegt, spritzt und schmatzt es laut.
mission-hippo-riddle-loc_mud-klasse3 = Heute Nacht hat es geregnet. Jetzt gibt es im Zoo eine Stelle, an der der Boden ganz weich und braun ist. Das Flusspferd liebt so etwas: Es wälzt sich darin, bis sein Rücken braun ist wie Schokolade. Such dort, wo es schmatzt und spritzt.

# loc_shade — shade under big trees at the zoo wall
mission-hippo-riddle-loc_shade-kiga = Schatten
mission-hippo-riddle-loc_shade-klasse1 = Mir ist zu heiß. Ich liege unter Bäumen. Hinter mir ist eine Mauer.
mission-hippo-riddle-loc_shade-klasse2 = Dem Flusspferd ist die Sonne zu heiß. Es liegt dort, wo große Bäume die Sonne verdecken, direkt an der hohen Mauer des Zoos.
mission-hippo-riddle-loc_shade-klasse3 = Flusspferde haben eine empfindliche Haut und mögen keine pralle Sonne. Darum hat sich unser Flusspferd einen dunklen, kühlen Platz gesucht. Dicke Baumkronen halten die Sonne ab, und hinter ihm steht die hohe Mauer, die den ganzen Zoo umgibt. Dort liegt es im trockenen Gras und döst.

## 3. Panda — loc_cave, loc_bamboo, loc_leaves (location riddles only; other panda texts: Q-069)

# loc_cave — dark, cool stone cave
mission-panda-riddle-loc_cave-kiga = Höhle
mission-panda-riddle-loc_cave-klasse1 = Ich bin müde. Dort ist es dunkel. Dort ist es kühl.
mission-panda-riddle-loc_cave-klasse2 = Der Panda ist müde. Er schläft an einem dunklen, kühlen Ort mit Wänden aus Stein.
mission-panda-riddle-loc_cave-klasse3 = Der Panda hat den ganzen Morgen gefressen. Jetzt ist er sehr müde. Er hat einen Platz gefunden, an dem es dunkel und kühl ist. Die Wände sind aus Stein, und wenn man ruft, hallt es zurück.

# loc_bamboo — bamboo thicket taller than the zoo wall (kiga word is not the food word Bambus)
mission-panda-riddle-loc_bamboo-kiga = Bambuswald
mission-panda-riddle-loc_bamboo-klasse1 = Ich habe Hunger. Hier wachsen grüne Stangen. Sie sind sehr hoch.
mission-panda-riddle-loc_bamboo-klasse2 = Der Panda hat Hunger. Er sitzt dort, wo grüne Stangen dicht an dicht wachsen, höher als die Mauer des Zoos.
mission-panda-riddle-loc_bamboo-klasse3 = Pandas fressen fast den ganzen Tag. Unser Panda hat einen Platz gefunden, an dem sein Lieblingsessen einfach aus dem Boden wächst. Die grünen Stangen stehen so dicht, dass man kaum hindurchsehen kann, und sie sind höher als die Mauer. Wenn der Wind weht, klappern sie leise.

# loc_leaves — raked pile of red and yellow leaves (kiga word is not the food word Blätter)
mission-panda-riddle-loc_leaves-kiga = Laubhaufen
mission-panda-riddle-loc_leaves-klasse1 = Ich spiele gern. Mein Bett ist weich. Rotes und gelbes Laub!
mission-panda-riddle-loc_leaves-klasse2 = Der Panda spielt gern. Er hat sich in einen großen, weichen Haufen aus roten und gelben Blättern gekuschelt.
mission-panda-riddle-loc_leaves-klasse3 = Der Zoowärter hat unter den Bäumen fleißig Blätter zusammengeharkt. Jetzt liegt dort ein riesiger Haufen aus roten, gelben und braunen Blättern. Der Panda hat ihn entdeckt und sich mitten hineingeworfen. Wenn er sich dreht, raschelt es, und Blätter fliegen durch die Luft.
