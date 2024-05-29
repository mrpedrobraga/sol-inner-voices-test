---
@type ResourceDef
@name Item
---

param $id: text
param $display_name: text
param $description: Routine

[events]
param $on_use: Routine(character)?
param $on_thrash: Routine(character)?
param $on_eat: Routine(character)?
