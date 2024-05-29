---
  @scene Bruno's Side Quest
  @author Pedro Braga
---

let $BRUNO_SIDE_QUEST_ROOM = asset "nhc.xhna.back.01"

[bruno_side_quest.01]

* The building looks incredibly ominous.
* Get inside?

> Sure
* Against your better judgement, you open the door.
warp $BRUNO_SIDE_QUEST_ROOM "entrance"

> Hell no
* You decide not to enter.

[bruno_side_quest.02]

(Claire) - You don't look that tough for a "final boss."
(Bruno)  - Well, tough this!
(Claire) - What?
wait 2.0
(Claire) - What?
(Bruno)  - Give it a minute...
