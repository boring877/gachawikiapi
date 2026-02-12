// Merges Zone Nova basic character data with detailed skill/awakening/memory data
// Run: node build/merge-characters.cjs

const fs = require('fs');
const path = require('path');

const GACHA_WIKI_PATH = 'C:/Users/Borin/OneDrive/Documents/gacha-wiki/src/data/zone-nova';
const OUTPUT_PATH = 'C:/Users/Borin/gachawikiapi/data/zone-nova-characters.json';

function normalizeStat(val) {
  if (typeof val === 'number') return val.toLocaleString();
  return val;
}

async function main() {
  const basicChars = require(path.join(GACHA_WIKI_PATH, 'characters.js')).ZONE_NOVA_CHARACTERS;
  
  const mergedChars = await Promise.all(basicChars.map(async (basic) => {
    const detailFile = path.join(GACHA_WIKI_PATH, 'characters', `${basic.slug}.js`);
    
    if (!fs.existsSync(detailFile)) {
      console.warn(`Missing detail file: ${basic.slug}.js`);
      return { ...basic, skills: null, awakenings: null, teamSkill: null, memoryCard: null };
    }
    
    const fileUrl = 'file:///' + detailFile.replace(/\\/g, '/');
    const detailModule = await import(fileUrl);
    const detailData = detailModule.default || Object.entries(detailModule).find(([key]) => key.endsWith('Data'))?.[1];
    
    if (!detailData) {
      console.warn(`No data export found in: ${basic.slug}.js`);
      return { ...basic, skills: null, awakenings: null, teamSkill: null, memoryCard: null };
    }

    // Normalize memoryCard stats to strings
    const memoryCard = detailData.memoryCard ? {
      ...detailData.memoryCard,
      stats: {
        hp: normalizeStat(detailData.memoryCard.stats?.hp),
        attack: normalizeStat(detailData.memoryCard.stats?.attack),
        defense: normalizeStat(detailData.memoryCard.stats?.defense),
      }
    } : null;
    
    return {
      id: basic.id,
      name: basic.name,
      slug: basic.slug,
      rarity: basic.rarity,
      element: basic.element,
      class: basic.class,
      role: basic.role,
      faction: basic.faction,
      tags: basic.tags,
      image: basic.image,
      detailUrl: basic.detailUrl,
      stats: {
        hp: basic.stats.hp,
        attack: basic.stats.attack,
        defense: basic.stats.defense,
        energyRecovery: basic.stats.energyRecovery,
        critRate: basic.stats.critRate,
        critDmg: basic.stats.critDmg,
      },
      skills: detailData.skills || null,
      teamSkill: detailData.teamSkill || null,
      awakenings: detailData.awakenings || null,
      memoryCard,
    };
  }));
  
  fs.writeFileSync(OUTPUT_PATH, JSON.stringify(mergedChars, null, 2));
  console.log(`Merged ${mergedChars.length} characters to ${OUTPUT_PATH}`);
}

main().catch(console.error);
