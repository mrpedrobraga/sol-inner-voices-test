---
@impl Item
---

using invo::[ player ]

$id            = "hamburger"
$display_name  = "Cool Hamburger"

[description]
* The finest hamburger in town.
* Heals 10 HP.

[events.on_use]
* This is a delicious hamburger!
attr (player) heal HP 10
