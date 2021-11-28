class Game {
  player: {
    hitPoints: number;
    mana: number;
    manaSpent: number;
    shield: number;
    recover: number;
    poison: number;
  };
  boss: {
    hitPoints: number;
    damage: number;
  };
}

enum GameResult {
  WIN,
  LOSE,
  CONTINUE,
}

interface Spell {
  cost: number;
  canCast: (Game) => boolean;
  effect: (Game) => Game;
}

const spellbook: Spell[] = [
  // Magic Missile costs 53 mana. It instantly does 4 damage.
  {
    cost: 53,
    canCast: () => true,
    effect: ({ player, boss }: Game): Game => ({
      player: { ...player },
      boss: { ...boss, hitPoints: boss.hitPoints - 4 },
    }),
  },
  // Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
  {
    cost: 73,
    canCast: () => true,
    effect: ({ player, boss }: Game): Game => ({
      player: { ...player, hitPoints: player.hitPoints + 2 },
      boss: { ...boss, hitPoints: boss.hitPoints - 2 },
    }),
  },
  // Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
  {
    cost: 113,
    canCast: ({ player }) => player.shield == 0,
    effect: ({ player, boss }: Game): Game => ({
      player: { ...player, shield: 6 },
      boss: { ...boss },
    }),
  },
  // Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
  {
    cost: 173,
    canCast: ({ player }) => player.poison == 0,
    effect: ({ player, boss }: Game): Game => ({
      player: { ...player, poison: 6 },
      boss: { ...boss },
    }),
  },
  // Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
  {
    cost: 229,
    canCast: ({ player }) => player.recover == 0,
    effect: ({ player, boss }: Game): Game => ({
      player: { ...player, recover: 6 },
      boss: { ...boss },
    }),
  },
];

function applyEffects({ player, boss }: Game): Game {
  const newPlayer = { ...player };
  const newBoss = { ...boss };

  if (player.poison > 0) {
    newPlayer.poison--;
    newBoss.hitPoints -= 3;
  }

  if (player.shield > 0) {
    newPlayer.shield--;
  }

  if (player.recover > 0) {
    newPlayer.recover--;
    newPlayer.mana += 101;
  }

  return { player: newPlayer, boss: newBoss };
}

const activeGames: Game[] = [
  {
    player: {
      hitPoints: 50,
      mana: 500,
      manaSpent: 0,
      shield: 0,
      recover: 0,
      poison: 0,
    },
    boss: {
      hitPoints: 51,
      damage: 9,
    },
  },
];

const wonGames: Game[] = [];
const lostGames: Game[] = [];

function checkGame({ player, boss }: Game): GameResult {
  if (player.hitPoints <= 0) return GameResult.LOSE;
  if (boss.hitPoints <= 0) return GameResult.WIN;
  return GameResult.CONTINUE;
}

while (activeGames.length) {
  let game = activeGames.pop();

  const cheapestWin = Math.min(...wonGames.map((g) => g.player.manaSpent));

  if (game.player.manaSpent > cheapestWin) continue;

  game.player.hitPoints--;

  // GameLose check
  if (checkGame(game) == GameResult.LOSE) {
    lostGames.push(game);
    continue;
  }

  // Apply effects
  game = applyEffects(game);

  // Poison check
  if (checkGame(game) == GameResult.WIN) {
    wonGames.push(game);
    continue;
  }

  // List possible attacks
  const spells = spellbook.filter((spell) => {
    return spell.canCast(game) && spell.cost < game.player.mana;
  });

  // No mana check
  if (spells.length == 0) {
    lostGames.push(game);
    continue;
  }

  // Pick attacks
  let allGames = spells.map((spell: Spell) => {
    let newGame = {
      player: {
        ...game.player,
        mana: game.player.mana - spell.cost,
        manaSpent: game.player.manaSpent + spell.cost,
      },
      boss: { ...game.boss },
    };

    return spell.effect(newGame);
  });

  // Boss effects
  let effectedGames = allGames
    .map((game) => {
      return applyEffects(game);
    })
    .filter((game) => {
      // Poison check
      if (checkGame(game) == GameResult.WIN) {
        wonGames.push(game);
        return false;
      }
      return true;
    });

  // Boss move

  let finalGames = effectedGames
    .map(({ player, boss }) => {
      let damage =
        player.shield > 0 ? Math.max(boss.damage - 7, 1) : boss.damage;
      return {
        player: { ...player, hitPoints: (player.hitPoints -= damage) },
        boss: { ...boss },
      };
    })
    .filter((game) => {
      // Damage check
      if (checkGame(game) == GameResult.LOSE) {
        lostGames.push(game);
        return false;
      }
      return true;
    });

  activeGames.push(...finalGames);
}

console.log(activeGames.length, wonGames.length, lostGames.length);

const cheapestWin = Math.min(...wonGames.map((g) => g.player.manaSpent));

console.log(cheapestWin);
