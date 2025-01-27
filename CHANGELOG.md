# Changelog

## Unreleased

## [1.0.0](https://github.com/facundoolano/rpg-cli/releases/tag/1.0.0) - 2021-09-07
### Fixed
* Don't reward items, gold or xp for cheap victories 6dc970a

## [1.0.0-beta](https://github.com/facundoolano/rpg-cli/releases/tag/1.0.0-beta) - 2021-09-05
### Added
* Quest to beat your own shadow #86
* Easter egg quest  #87
* Sorcerer enemy class  #88
* Stat increasing stones #95
* Effect rings as items and equipment, initial set of stat-based rings #98
* RegenHP and RegenMP rings #109
* Status effect rings #110
* Battle related rings #113
* Treasure related rings #114
* Ring related quests #115

### Changed
* Remember unlocked quests and todo list order #89
* Cheaper ether 62bb9ed
* Renamed status effects "poisoned" to "poison" and "burning" to "burn" #92
* Doubled ether restored mp 0e01209
* Tweaked enemy levels to be based primarily on distance from home rather than player level 0798d53
* Changed internal representation of equipment #99
* When a magic-using character runs out of mp, its physical attacks incorporate the weapon contribution 39c5e01
* Changed internal representation of items #105
* Fail gracefully on data breaking changes #107
* Show items bought and money spent in the buy command output #108
* Show mp cost in magic attacks 9f92efc
* The stat command can be used to describe items and equipment #117
* Equipment level found in chest based on distance instead of player level d22d3b9
* Game balance related tweaks #118

### Fixed
* Reach level 50 and 100 unlock and reward 4128f75
* Properly report raise class levels quest progress e7d73f9
* Reach level quests rewarded when multiple levels raised in a single event 60f5fb2
* Give base mp when switching to a magic class from a non base level 96c2de6
* Missed levels with class quest completion 1ec760
* Tweak gold found in chests 0317979 83691fa
* Don't add xp beyond the actual inflicted damage (prevents high xp when beating weaker enemies) 812a5f1
* Continue moving through dirs after successful bribe/run away 570a0de

## [0.6.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.6.0) - 2021-08-04
### Added
* Customizable classes file #76
* Thief class and command to select player class #77
* Mage class, magic attacks and ether item #78
* Quests to raise 5 levels on each available player class #81
* Reach level 50 and level 100 quests #81
* Items rewarded on battle won #82

### Removed
* Backwards compatibility code for binary game data from v0.4.0 #75

### Changed
* `rpg reset --hard` removes datafile instead of entire .rpg dir 5adfb87
* Character speed contributes to run away success probability 4d6e1a3
* Initial stats are randomized 50af983
* Use GitHub actions instead of travis for CI and release building #80
* Change xp gained based on enemy class category #83
* Accept multiple items in buy and use commands #84

### Fixed
* Find chest quest not rewarded when finding a tombstone c0d62aa

## [0.5.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.5.0) - 2021-06-26
### Added
* a `rpg reset --hard` flag to remove data files and forget information from previous plays #46
* Quest system #47
* Tutorial quests #49
* `rpg ls` command to look for chests at the current location #51
* Example sh file #54
* Poisoned and burning status effects #48

### Changed
* Tombstones are found with `rpg ls` instead of automatically #52

### Fixed
* When hero dies twice in the same location, tombstone chest contents
are merged instead of overridden #73

## [0.4.1](https://github.com/facundoolano/rpg-cli/releases/tag/0.4.1) - 2021-06-14
### Changed
* Game data is now serialized to JSON to allow extending it without breaking backwards compatibility.

## [0.4.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.4.0) - 2021-06-05
### Added
* This Changelog
* `rpg cd -f` sets the hero location without initiating battles, intended for custom shell integrations
* `rpg battle` initiates a battle (with a probability) at the hero's current location.
* --quiet,-q option to reduce output while changing directories and printing the hero status.
* --plain to facilitate scripting around the hero stats.
* Documentation for shell integrations.

### Changed
* General command overhaul, now all actions are done via a subcommand: `rpg cd`, `rpg stat`, etc., with status printing being the default.
* `rpg cd` without args moves the hero to home and `rpg cd -` moves it to `$OLDPWD` (when present) to match the `cd` behavior 4ba4c59
* --shop,-s renamed to buy,b and --inventory,-i renamed to use,u f737a81
* Removed most empty lines from output.

## [0.3.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.3.0) - 2021-05-28
### Added
* Binary upload from travis on GitHub releases #36
* Experimental support for windows #35
* Different OS tests in travis 3a7eb6b

### Changed
* Print version number in help 8efdead
* Rebalancing of character stats to prevent overgrowth #33
* Several updates to the README instructions

### Fixed
* Prevent overflow bug at high levels #33
* Keep items sorted when printing the character status #15
* Missing Cargo.lock checked into the repository #26

## [0.2.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.2.0) - 2021-05-23

## [0.1.0](https://github.com/facundoolano/rpg-cli/releases/tag/0.1.0) - 2021-05-06
