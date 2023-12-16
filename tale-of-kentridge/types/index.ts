import { u32, u8, struct, cstr } from "@solana/buffer-layout";
import { bool, publicKey } from "@solana/buffer-layout-utils";
import { PublicKey } from "@solana/web3.js";

export interface RawPlayer {
    playerAddress: PublicKey;
    nickname: string;
    isPlayerExist: boolean;
    isMatching: boolean;
    isInGame: boolean;
}

export const PlayerLayout = struct<RawPlayer>([
    publicKey("playerAddress"),
    cstr("nickname"),
    bool("isPlayerExist"),
    bool("isMatching"),
    bool("isInGame"),
]);
