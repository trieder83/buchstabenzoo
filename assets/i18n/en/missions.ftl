# Missions and food box labels — English. Source: CONT-MISSIONS
# (specs/20-content/missions/start-missions.md). Mirrors de/missions.ftl key by key.

## Food box labels (word printed on the box, also used on the info boards)

food-grass = grass
food-melons = melons
food-bamboo = bamboo
food-eucalyptus = eucalyptus
food-hay = hay
food-fish_food = fish food
food-bananas = bananas
food-leaves = leaves
food-meat = meat
food-berries = berries

## 1. Zebra — loc_river, loc_meadow, loc_sand

# Info board heading (level-independent)
animal-zebra = Zebra
# heading above the facts
animal-zebra-more = More about the zebra

# Facts on the info board (GAME-ANIMALS "Info board" 4) — never a place word.
# kiga: shown next to a picture of zebra stripes
mission-zebra-facts-kiga = stripes
mission-zebra-facts-klasse1 = I have stripes. I live in Africa. I run very fast.
mission-zebra-facts-klasse2 = Zebras live in big herds in Africa. Every zebra has different stripes. No two patterns are the same. Zebras can run very fast.
mission-zebra-facts-klasse3 = Zebras come from Africa and live there in big herds. Their stripes are as unique as your fingerprint: no zebra looks exactly like another. Zebras eat grass all day long. When a lion comes, they run away, as fast as a car in town. A zebra foal can walk soon after it is born.

# Location riddle — LEGACY PoC keys without the hiding place (= loc_river texts). Used by the
# current zoo-core riddle_key(); delete when it switches to
# mission-<animal>-riddle-<hiding_place>-<reading_level> (CONT-MISSIONS Behaviour 1).

# kiga: shown next to a picture of the river
mission-zebra-riddle-kiga = river
mission-zebra-riddle-klasse1 = I am thirsty. I look for running water.
mission-zebra-riddle-klasse2 = The zebras are very thirsty. They drink where the water flows and ducks swim.
mission-zebra-riddle-klasse3 = The zebras ran around for a long time. Now they are thirsty. They are looking for water that moves and rushes. They drink where a bridge crosses the water.

# Location riddles per candidate hiding place (level 1, GAME-LEVEL-1 "Hiding places").
# kiga: shown next to the picture of the place (picture id = hiding place id).

# loc_river — river with bridge and ducks
mission-zebra-riddle-loc_river-kiga = river
mission-zebra-riddle-loc_river-klasse1 = I am thirsty. I look for running water.
mission-zebra-riddle-loc_river-klasse2 = The zebras are very thirsty. They drink where the water flows and ducks swim.
mission-zebra-riddle-loc_river-klasse3 = The zebras ran around for a long time. Now they are thirsty. They are looking for water that moves and rushes. They drink where a bridge crosses the water.

# loc_meadow — tall grass, wildflowers, butterflies
mission-zebra-riddle-loc_meadow-kiga = meadow
mission-zebra-riddle-loc_meadow-klasse1 = I am hungry. The grass is tall. Butterflies fly there.
mission-zebra-riddle-loc_meadow-klasse2 = The zebras are hungry. They eat where the grass grows tall, colourful flowers bloom and butterflies fly.
mission-zebra-riddle-loc_meadow-klasse3 = The zebras are very hungry. They are looking for a place where the grass is so tall that it reaches their bellies. Colourful flowers bloom between the blades, and butterflies flutter around. Big trees stand behind the tall grass.

# loc_sand — dry yellow sand patch (dust bath)
mission-zebra-riddle-loc_sand-kiga = sand
mission-zebra-riddle-loc_sand-klasse1 = My fur itches. I like to roll. The ground is yellow.
mission-zebra-riddle-loc_sand-klasse2 = The zebras' fur itches. They roll around where the ground is yellow and dry and no grass grows.
mission-zebra-riddle-loc_sand-klasse3 = Zebras like to take a dust bath. They roll on their backs until their fur is all dusty. It helps against annoying flies. Our zebras have found a place where not a single blade of grass grows and the ground is yellow and warm.

# shown when the zebras are back in their enclosure (mission complete, GAME-RESCUE §8)
mission-zebra-home = Great! The zebras are home again.

## 2. Hippo — loc_pond, loc_mud, loc_shade (location riddles only; other hippo texts: Q-069)

# loc_pond — still pond, water lilies, frogs
mission-hippo-riddle-loc_pond-kiga = pond
mission-hippo-riddle-loc_pond-klasse1 = I like to bathe. The water is still. Water lilies grow there.
mission-hippo-riddle-loc_pond-klasse2 = The hippo likes to lie in water. It likes still water with water lilies and frogs.
mission-hippo-riddle-loc_pond-klasse3 = The hippo gets hot quickly in the sun. So it climbed into water that does not flow. Water lilies float on top, and frogs croak on the shore. Only its eyes and ears stick out.

# loc_mud — brown mud puddle
mission-hippo-riddle-loc_mud-kiga = mud
mission-hippo-riddle-loc_mud-klasse1 = I like it wet. The ground is brown. It splashes and squelches.
mission-hippo-riddle-loc_mud-klasse2 = The hippo lies in a big, brown puddle. When it moves, it splashes and squelches loudly.
mission-hippo-riddle-loc_mud-klasse3 = It rained last night. Now there is a spot in the zoo where the ground is soft and brown. The hippo loves that: it rolls around until its back is as brown as chocolate. Look where it squelches and splashes.

# loc_shade — shade under big trees at the zoo wall
mission-hippo-riddle-loc_shade-kiga = shade
mission-hippo-riddle-loc_shade-klasse1 = I am too hot. I lie under trees. Behind me is a wall.
mission-hippo-riddle-loc_shade-klasse2 = The sun is too hot for the hippo. It lies where big trees hide the sun, right next to the high zoo wall.
mission-hippo-riddle-loc_shade-klasse3 = Hippos have sensitive skin and do not like strong sun. So our hippo has looked for a dark, cool spot. Thick treetops keep the sun away, and behind it stands the high wall around the whole zoo. It lies there in the dry grass and dozes.

## 3. Panda — loc_cave, loc_bamboo, loc_leaves (location riddles only; other panda texts: Q-069)

# loc_cave — dark, cool stone cave
mission-panda-riddle-loc_cave-kiga = cave
mission-panda-riddle-loc_cave-klasse1 = I am tired. It is dark there. It is cool there.
mission-panda-riddle-loc_cave-klasse2 = The panda is tired. It sleeps in a dark, cool place with walls of stone.
mission-panda-riddle-loc_cave-klasse3 = The panda ate all morning. Now it is very tired. It found a place that is dark and cool. The walls are made of stone, and when you shout, it echoes.

# loc_bamboo — bamboo thicket taller than the zoo wall (kiga word is not the food word bamboo)
mission-panda-riddle-loc_bamboo-kiga = bamboo forest
mission-panda-riddle-loc_bamboo-klasse1 = I am hungry. Green stalks grow here. They are very tall.
mission-panda-riddle-loc_bamboo-klasse2 = The panda is hungry. It sits where green stalks grow close together, taller than the zoo wall.
mission-panda-riddle-loc_bamboo-klasse3 = Pandas eat almost all day long. Our panda has found a place where its favourite food simply grows out of the ground. The green stalks stand so close together that you can hardly see through them, and they are taller than the wall. When the wind blows, they rattle softly.

# loc_leaves — raked pile of red and yellow leaves (kiga word is not the food word leaves)
mission-panda-riddle-loc_leaves-kiga = leaf pile
mission-panda-riddle-loc_leaves-klasse1 = I like to play. My bed is soft. Red and yellow leaves!
mission-panda-riddle-loc_leaves-klasse2 = The panda likes to play. It has snuggled into a big, soft heap of red and yellow leaves.
mission-panda-riddle-loc_leaves-klasse3 = The zookeeper has been busy raking leaves under the trees. Now there is a huge heap of red, yellow and brown leaves. The panda found it and jumped right into the middle. When it turns over, the leaves rustle and fly through the air.
