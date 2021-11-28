// --- Day 21: RPG Simulator 20XX ---

// Little Henry Case got a new video game for Christmas.
// It's an RPG, and he's stuck on a boss. He needs to know what equipment to buy at the shop. He hands you the controller.

interface CharacterStats {
  hitPoints: number;
  damage: number;
  armor: number;
}

interface Items {
  cost: number;
  damage: number;
  armor: number;
}

const boss: CharacterStats = {
  hitPoints: 109,
  damage: 8,
  armor: 2,
};

const character: CharacterStats = {
  hitPoints: 100,
  damage: 0,
  armor: 0,
};

// In this game, the player (you) and the enemy (the boss) take turns attacking.
// The player always goes first. Each attack reduces the opponent's hit points by at least 1. The first character at or below 0 hit points loses.

// Damage dealt by an attacker each turn is equal to the attacker's damage score minus the defender's armor score. A
// An attacker always does at least 1 damage.
// So, if the attacker has a damage score of 8, and the defender has an armor score of 3, the defender loses 5 hit points.
// If the defender had an armor score of 300, the defender would still lose 1 hit point.

// Your damage score and armor score both start at zero.
// They can be increased by buying items in exchange for gold.
// You start with no items and have as much gold as you need.
// Your total damage or armor is equal to the sum of those stats from all of your items. You have 100 hit points.

// Here is what the item shop is selling:

const weapons: Items[] = [
  {
    cost: 8,
    damage: 4,
    armor: 0,
  },
  {
    cost: 10,
    damage: 5,
    armor: 0,
  },
  {
    cost: 25,
    damage: 6,
    armor: 0,
  },
  {
    cost: 40,
    damage: 7,
    armor: 0,
  },
  {
    cost: 74,
    damage: 8,
    armor: 0,
  },
];
// Weapons:    Cost  Damage  Armor
// Dagger        8     4       0
// Shortsword   10     5       0
// Warhammer    25     6       0
// Longsword    40     7       0
// Greataxe     74     8       0

const armors: Items[] = [
  {
    cost: 13,
    damage: 0,
    armor: 1,
  },
  {
    cost: 31,
    damage: 0,
    armor: 2,
  },
  {
    cost: 53,
    damage: 0,
    armor: 3,
  },
  {
    cost: 75,
    damage: 0,
    armor: 4,
  },
  {
    cost: 102,
    damage: 0,
    armor: 5,
  },
];

// Armor:      Cost  Damage  Armor
// Leather      13     0       1
// Chainmail    31     0       2
// Splintmail   53     0       3
// Bandedmail   75     0       4
// Platemail   102     0       5

const rings: Items[] = [
  {
    cost: 25,
    damage: 1,
    armor: 0,
  },
  {
    cost: 50,
    damage: 2,
    armor: 0,
  },

  {
    cost: 100,
    damage: 3,
    armor: 0,
  },

  {
    cost: 20,
    damage: 0,
    armor: 1,
  },
  {
    cost: 40,
    damage: 0,
    armor: 2,
  },
  {
    cost: 80,
    damage: 0,
    armor: 3,
  },
];
// Rings:      Cost  Damage  Armor
// Damage +1    25     1       0
// Damage +2    50     2       0
// Damage +3   100     3       0
// Defense +1   20     0       1
// Defense +2   40     0       2
// Defense +3   80     0       3

const possibleEquipmentStats: Items[] = [];

// You must buy exactly one weapon; no dual-wielding.

weapons.forEach((weapon) => {
  possibleEquipmentStats.push(weapon);
});

// Armor is optional, but you can't use more than one.

possibleEquipmentStats.forEach((stat) => {
  armors.forEach((armor) => {
    possibleEquipmentStats.push({
      cost: stat.cost + armor.cost,
      damage: stat.damage,
      armor: stat.armor + armor.armor,
    });
  });
});

// You can buy 0-2 rings (at most one for each hand).

possibleEquipmentStats.forEach((stat) => {
  for (let i = 0; i < rings.length; i++) {
    // Add single ring
    possibleEquipmentStats.push({
      cost: stat.cost + rings[i].cost,
      damage: stat.damage + rings[i].damage,
      armor: stat.armor + rings[i].armor,
    });

    for (let j = 0; j < rings.length; j++) {
      // Add single ring with a companion
      if (i != j) {
        possibleEquipmentStats.push({
          cost: stat.cost + rings[i].cost + rings[j].cost,
          damage: stat.damage + rings[i].damage + rings[j].damage,
          armor: stat.armor + rings[i].armor + rings[j].armor,
        });
      }
    }
  }
});

// The shop only has one of each item, so you can't buy, for example, two rings of Damage +3.

// You have 100 hit points. The boss's actual stats are in your puzzle input. What is the least amount of gold you can spend and still win the fight?

const winningEquipment = [];
const losingEquipment = [];

possibleEquipmentStats.forEach((equipment) => {
  const characterStats = { ...equipment, hitPoints: 100 };
  const bossStats = { ...boss };

  while (bossStats.hitPoints > 0 && characterStats.hitPoints > 0) {
    // Player attacks
    const playerDamage = Math.max(characterStats.damage - bossStats.armor, 1);
    bossStats.hitPoints -= playerDamage;

    if (bossStats.hitPoints <= 0) break;

    // Boss attacks
    const bossDamage = Math.max(bossStats.damage - characterStats.armor, 1);
    characterStats.hitPoints -= bossDamage;
  }

  if (characterStats.hitPoints > 0) winningEquipment.push(equipment);
  else {
    losingEquipment.push(equipment);
  }
});

console.log(winningEquipment.length);

const winningEquipmentCost = winningEquipment.map((e) => e.cost);
console.log(Math.min(...winningEquipmentCost));

const losingEquipmentCost = losingEquipment.map((e) => e.cost);
console.log(Math.max(...losingEquipmentCost));
