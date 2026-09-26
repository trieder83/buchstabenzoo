---
id: GAME-CART
title: Golf carts
aspect: gameplay
module: golf-carts
status: draft
depends_on: [GAME-PLAYER, GAME-LAYOUT, GAME-RESCUE, GAME-SAVE]
test_prefix: CART
updated: 2026-09-26
---

# Golf carts

## Goal

The zoo has **3 golf carts** (*Golfwagen*) the child can drive to get from one place to
another more quickly (user request 2026-09-26). **Animals do not follow a cart** — rescuing
still means walking the animal home.

## Behaviour

1. **Three carts** stand at parking spots (proposal Q-119: one at the entrance plaza of
   level 1, one at the entry of level 2, one at the entry of level 3 — a cart of a locked
   level is only usable once that level is open). Each cart has a small parking sign.
2. **Getting in / out:** standing next to a cart (GAME-PLAYER §5 rules, the driver's side),
   the interact button shows a cart icon; interact → the girl sits in the driver's seat.
   Interact again (or a big "get out" button) → she steps out on the side, the cart stays
   exactly where it was left (it does not drive back by itself — proposal Q-119).
3. **Driving:** the same controls as walking (left thumb / WASD): push forward = drive
   forward, steer by the joystick direction relative to the camera; smooth acceleration and
   braking, no reversing needed (proposal: slow reverse when pulling back). Speed on paths
   **4.5 m/s** (≈ 2.3× walking), on grass 2.0 m/s (proposal Q-120). The camera zooms out a
   little (+3 m) while driving and follows smoothly.
4. **Where carts can go:** only walkable cells of unlocked levels — paths and grass; never
   through barriers, fences, gates into enclosures, the garden gate, buildings, water
   (bridges are fine if at least 2 m wide), sparse woods (trunks too close) or props. The
   cart collides like a box (≈ 1.3 × 2.3 m) and slides along obstacles; it never gets
   stuck (it can always turn on the spot).
5. **Safety, child-friendly:** the cart slows down and stops before it would touch a
   visitor, an animal or a duck on land; nothing can be run over or pushed. No crashes, no
   damage, no timer. A friendly horn button (🔔 "tüt-tüt") is optional fun.
6. **Animals do not follow the cart:** when the girl gets into a cart while animals are
   `following`, they stop and **wait** at that spot (GAME-RESCUE §6 waiting); when she comes
   back on foot within 5 m they follow again. A short feedback bubble tells the child
   ("Wir laufen lieber!" / "We'd rather walk!"). Showing food from the cart does nothing.
7. **Carried things:** food, the basket and the fish bowl (with the fish) come along in the
   cart. The fish stays safe in the bowl.
8. **Interactions while driving:** reading panels do not open automatically while driving
   (so the ride isn't interrupted); get out to read. The map (GAME-MAP) can be opened.
9. **Night:** carts have small headlights at night (GAME-NIGHT).
10. **Saving:** every cart's position, yaw and whether the girl sits in it are saved
    (GAME-SAVE).
11. **Art:** a comic zoo golf cart (green-white with a striped roof, zoo logo space left
    blank, 2 seats, small cargo area where carried items are visible). Concept sheet first
    (ART-PIPELINE); the girl has a `drive` sitting clip (ART-RIG).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| CART-001 | Given a new game, then 3 carts stand at their parking spots and only the carts of unlocked levels can be entered. | unit |
| CART-002 | Given the player next to a cart, when she interacts, then she sits in it; interacting again puts her on the ground next to it and the cart stays where it is. | unit |
| CART-003 | Given the cart on a path, when driving forward for 1 s at full speed, then it moves at 4.5 m/s ± 5 % (grass 2.0 m/s). | unit |
| CART-004 | Given the cart driven against a fence, a barrier, water, a building and an enclosure gate, then it never overlaps them and never enters a locked level. | unit |
| CART-005 | Given an animal on the cart's path, then the cart stops before touching it. | unit |
| CART-006 | Given the zebra is following, when the player gets into a cart and drives 20 m away, then the zebra waits at the spot where she got in; when she returns on foot within 5 m, it follows again. | unit |
| CART-007 | Given the player carries the fish bowl with the fish, when she drives and gets out, then she still carries the bowl and the fish. | unit |
| CART-008 | Given the player drives past an info board, then no reading panel opens. | unit |
| CART-009 | Given a save while sitting in a cart, when restored, then she sits in the same cart at the same place. | unit |
| CART-010 | Given touch controls, then driving works with the left thumb exactly like walking and the get-out button is reachable with the right thumb. | e2e |

## Open questions

- Q-119 Where the 3 carts park, and whether a cart drives back to its parking spot by itself.
- Q-120 Speeds (paths 4.5 m/s, grass 2.0 m/s) and whether carts may drive on grass at all.
