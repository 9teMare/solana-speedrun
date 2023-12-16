import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { assert, expect } from "chai";
import { TaleOfKentridge } from "../target/types/tale_of_kentridge";
import { PlayerLayout } from "../types";

describe("TaleOfKentridge", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TaleOfKentridge as Program<TaleOfKentridge>;
    const wallet = provider.wallet;
    const walletSeed = wallet.publicKey.toBuffer();
    const [dataAccount, bump] = PublicKey.findProgramAddressSync([Buffer.from("seed"), walletSeed], program.programId);

    it("Test 1 - Is initialized", async () => {
        try {
            await program.methods.new(walletSeed, [bump]).accounts({ dataAccount }).rpc();

            assert(true, "program initialized");
        } catch (err) {
            console.log(err);
        }
    });

    it("Test 2 - Onboard player1", async () => {
        const address1 = wallet.publicKey;
        await program.methods.onboardPlayer(address1, "9te").accounts({ dataAccount }).rpc();

        const player = await program.methods.getPlayer(address1).accounts({ dataAccount }).view();
        expect(player.nickname).to.equal("9te");
        expect(player.playerAddress.toString()).to.equal(address1.toString());
        expect(player.isPlayerExist).is.true;
        expect(player.isMatching).is.false;
        expect(player.isInGame).is.false;
    });

    it("Test 3 - Onboard player1 but exists", async () => {
        const address1 = wallet.publicKey;
        try {
            await program.methods.onboardPlayer(address1, "9te").accounts({ dataAccount }).rpc();

            assert(false, "should've failed but didn't ");
        } catch (err) {
            expect(
                (err.logs as string[]).some((msg) =>
                    msg.includes("Program log: runtime_error: PLAYER_ALREADY_EXIST require condition failed in tale-of-kentridge.sol")
                )
            ).to.be.true;
        }
    });

    it("Test 4 - Onboard player2", async () => {
        const address2 = new anchor.web3.PublicKey("Hiy6JSQwKTidbFq5PpeiepThw1vALSFKijYSDCRDDq1L");
        await program.methods.onboardPlayer(address2, "phantom").accounts({ dataAccount }).rpc();

        const player = await program.methods.getPlayer(address2).accounts({ dataAccount }).view();
        expect(player.nickname).to.equal("phantom");
        expect(player.playerAddress.toString()).to.equal(address2.toString());
        expect(player.isPlayerExist).is.true;
        expect(player.isMatching).is.false;
        expect(player.isInGame).is.false;
    });

    it("Test 5 - Player1 enqueue", async () => {
        const address1 = wallet.publicKey;
        await program.methods.enqueuePlayer(address1).accounts({ dataAccount }).rpc();
        const player1 = await program.methods.getPlayer(address1).accounts({ dataAccount }).view();
        expect(player1.isMatching).is.true;
        expect(player1.isInGame).is.false;

        const queue = await program.methods.getQueue().accounts({ dataAccount }).view();

        expect(queue.length).to.equal(1);
    });

    it("Test 6 - Player2 enqueue", async () => {
        const address1 = wallet.publicKey;
        const address2 = new anchor.web3.PublicKey("Hiy6JSQwKTidbFq5PpeiepThw1vALSFKijYSDCRDDq1L");
        await program.methods.enqueuePlayer(address2).accounts({ dataAccount }).rpc();
        const player1 = await program.methods.getPlayer(address1).accounts({ dataAccount }).view();
        const player2 = await program.methods.getPlayer(address2).accounts({ dataAccount }).view();

        // After player2 enqueue, both player1 and player2 should be in game
        expect(player1.isMatching).is.false;
        expect(player1.isInGame).is.true;
        expect(player2.isMatching).is.false;
        expect(player2.isInGame).is.true;

        // And the queue should be empty
        const queue = await program.methods.getQueue().accounts({ dataAccount }).view();
        expect(queue.length).to.equal(0);
    });

    it("Test 7 - Get roomId", async () => {
        console.log(program.idl.events[4].fields[0].index);
    });
});
