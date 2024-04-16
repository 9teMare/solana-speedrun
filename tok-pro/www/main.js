import initTurbo, * as turbo from "./pkg/turbo_genesis_host_wasm_bindgen.js";

/**************************************************/
/* CONFIGURATION                                  */
/**************************************************/


const APP_NAME = "";
const APP_VERSION = "";
const APP_AUTHOR = "";
const APP_DESCRIPTION = "";
const RESOLUTION = [256, 144];
const WASM_SRC = "tok_pro.wasm";
        

const SPRITES = ["./sprites/undead_revenant.png","./sprites/humanoid_occultist.png","./sprites/paper.png","./sprites/royal_soldier.png","./sprites/royal_swordman.png","./sprites/royal_priest.png","./sprites/selecting_red.png","./sprites/nus.png","./sprites/royal_knight.png","./sprites/potion.png","./sprites/humanoid_wanderer.png","./sprites/undead_archer.png","./sprites/royal_cleric.png","./sprites/humanoid_fox.png","./sprites/humanoid_assasin.png","./sprites/road.png","./sprites/squarepaper.png","./sprites/humanoid_pikeman.png","./sprites/undead.png","./sprites/undead_crawler.png","./sprites/undead_scarab.png","./sprites/undead_eye.png","./sprites/undead_stumbler.png","./sprites/humanoid_wolfrider.png","./sprites/royal_angel.png","./sprites/royal_shield.png","./sprites/hand.png","./sprites/selecting_yellow.png","./sprites/grass.png","./sprites/royal_paladin.png","./sprites/deck.png","./sprites/humanoid_tinker.png","./sprites/royal.png","./sprites/confirm.png","./sprites/mana_empty.png","./sprites/tree.png","./sprites/mana.png","./sprites/punch.png","./sprites/royal_crusade.png","./sprites/undead_hand.png","./sprites/humanoid_fanatic.png","./sprites/selecting.png","./sprites/card_attack.png","./sprites/paper_selected.png","./sprites/humanoid_archer.png","./sprites/humanoid_grunt.png","./sprites/humanoid.png","./sprites/royal_champion.png","./sprites/undead_bones.png","./sprites/undead_feader.png","./sprites/undead_ghoul.png",];

/**************************************************/

// This proxy prevents WebAssembly.LinkingError from being thrown
// prettier-ignore
window.createWasmImportsProxy = (target = {}) => {
  console.log(target);
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

window.turboSolUser = window.turboSolUser ?? (() => null);
window.turboSolGetAccount = window.turboSolGetAccount ?? (async () => {});
window.turboSolSignAndSendTransaction =
  window.turboSolSignAndSendTransaction ?? (async () => {});

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
        return [
          src.replace(/^.*[\\/]/, "").replace(/.(png|jpg|jpeg|gif)$/, ""),
          buf,
        ];
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

  // Initialize nipple (aka virtual analog stick)
  initializeNipple(canvas);

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

function initializeNipple(canvas) {
  const presses = {
    up: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowUp",
        code: "ArrowUp",
      }),
    },
    down: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowDown",
        code: "ArrowDown",
      }),
    },
    left: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowLeft",
        code: "ArrowLeft",
      }),
    },
    right: {
      keydown: new KeyboardEvent("keydown", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
      keyup: new KeyboardEvent("keyup", {
        key: "ArrowRight",
        code: "ArrowRight",
      }),
    },
  };
  let active = null;
  nipplejs
    .create()
    .on("dir:up", (e) => {
      console.log(e);
      if (active && active !== presses.up) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.up.keydown);
      active = presses.up;
    })
    .on("dir:down", (e) => {
      console.log(e);
      if (active && active !== presses.down) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.down.keydown);
      active = presses.down;
    })
    .on("dir:left", (e) => {
      console.log(e);
      if (active && active !== presses.left) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.left.keydown);
      active = presses.left;
    })
    .on("dir:right", (e) => {
      console.log(e);
      if (active && active !== presses.right) {
        canvas.dispatchEvent(active.keyup);
      }
      canvas.dispatchEvent(presses.right.keydown);
      active = presses.right;
    })
    .on("end", (e) => {
      console.log(e);
      if (active) {
        canvas.dispatchEvent(active.keyup);
      }
      active = null;
    });
}
