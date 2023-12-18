import initTurbo, * as turbo from "/pkg/turbo_genesis_host_wasm_bindgen.js";

// Check if the user is on a mobile device
function isMobile() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
}

// Example usage
if (isMobile()) {
    console.log("User is on a mobile device");
} else {
    console.log("User is on a desktop device");
}

/**************************************************/
/* CONFIGURATION                                  */
/**************************************************/

// Game metadata
const APP_NAME = "My Turbo Game";
const APP_VERSION = "0.0.0";
const APP_AUTHOR = "DDX & 9te";
const APP_DESCRIPTION = "Not your usual awesome 2D pixel TCG game, made with Turbo";

// NOTE: You can find your builds in your rust crate in one of two places:
// - target/wasm32-unknown-unknown/release/[package_name].wasm
// - target/wasm32-unknown-unknown/debug/[package_name].wasm
// Copy it into this web directory
const WASM_SRC = "/turbo.wasm";

// The game's resolution
const RESOLUTION = [256, 144];

if (isMobile()) {
    RESOLUTION = [144, 256];
}

// Add sprites to this array
const SPRITES = [
    "/sprites/card_attack.png",
    "/sprites/confirm.png",
    "/sprites/deck.png",
    "/sprites/grass.png",
    "/sprites/hand.png",
    "/sprites/humanoid.png",
    "/sprites/humanoid_archer.png",
    "/sprites/humanoid_assasin.png",
    "/sprites/humanoid_fanatic.png",
    "/sprites/humanoid_fox.png",
    "/sprites/humanoid_grunt.png",
    "/sprites/humanoid_occultist.png",
    "/sprites/humanoid_pikeman.png",
    "/sprites/humanoid_tinker.png",
    "/sprites/humanoid_wanderer.png",
    "/sprites/humanoid_wolfrider.png",
    "/sprites/mana.png",
    "/sprites/mana_empty.png",
    "/sprites/nus.png",
    "/sprites/paper.png",
    "/sprites/paper_selected.png",
    "/sprites/potion.png",
    "/sprites/punch.png",
    "/sprites/road.png",
    "/sprites/royal.png",
    "/sprites/royal_angel.png",
    "/sprites/royal_champion.png",
    "/sprites/royal_cleric.png",
    "/sprites/royal_crusade.png",
    "/sprites/royal_knight.png",
    "/sprites/royal_paladin.png",
    "/sprites/royal_priest.png",
    "/sprites/royal_shield.png",
    "/sprites/royal_soldier.png",
    "/sprites/royal_swordman.png",
    "/sprites/selecting.png",
    "/sprites/selecting_red.png",
    "/sprites/selecting_yellow.png",
    "/sprites/squarepaper.png",
    "/sprites/tree.png",
    "/sprites/undead.png",
    "/sprites/undead_archer.png",
    "/sprites/undead_bones.png",
    "/sprites/undead_crawler.png",
    "/sprites/undead_eye.png",
    "/sprites/undead_feader.png",
    "/sprites/undead_ghoul.png",
    "/sprites/undead_hand.png",
    "/sprites/undead_revenant.png",
    "/sprites/undead_scarab.png",
    "/sprites/undead_stumbler.png",

    // Add as many as you have in your /sprites folder
];

/**************************************************/

// This proxy prevents WebAssembly.LinkingError from being thrown
// prettier-ignore
window.createWasmImportsProxy = (target = {}) => {
    console.log("imports", target);
    return new Proxy(target, {
      get: (target, namespace) => {
          // Stub each undefined namespace with a Proxy
          target[namespace] = target[namespace] ?? new Proxy({}, {
              get: (_, prop) => {
                  // Generate a sub function for any accessed property
                  return (...args) => {
                      console.log(`Calling ${namespace}.${prop} with arguments:`, args);
                      // Implement the actual function logic here
                  };
              }
          });
          return target[namespace];
        }
    })
  };

/**************************************************/

try {
    // Initalize Turbo's WASM runtime
    await initTurbo();

    // Create the game's canvas
    const player = document.getElementById("player");

    // Initialize a temporary 2D context canvas for loading state
    const loading = document.createElement("canvas");
    player?.appendChild(loading);
    var context = loading.getContext("2d");
    context.fillStyle = "white";
    context.font = "bold 14px 04b03";
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillText("Loading...", loading.width / 2, loading.height / 2);

    // Fetch sprites
    const spriteData = await Promise.all(
        SPRITES.map(async (src) => {
            try {
                let res = await fetch(src);
                let buf = await res.arrayBuffer();
                return [src.replace(/^.*[\\/]/, "").replace(/.(png|jpg|jpeg|gif)$/, ""), buf];
            } catch (err) {
                console.error("Could not fetch sprite:", src);
                return null;
            }
        }).filter((x) => !!x)
    );

    // Remove loading state
    player?.removeChild(loading);

    // Append game canvas
    const canvas = document.createElement("canvas");
    player?.appendChild(canvas);

    // Run game
    await turbo.run(canvas, spriteData, {
        source: WASM_SRC,
        meta: {
            appName: APP_NAME,
            appVersion: APP_VERSION,
            appAuthor: APP_AUTHOR,
            appDescription: APP_DESCRIPTION,
        },
        config: {
            resolution: RESOLUTION,
        },
    });
} catch (err) {
    console.error("Turbo failed to initialize", err);
}
