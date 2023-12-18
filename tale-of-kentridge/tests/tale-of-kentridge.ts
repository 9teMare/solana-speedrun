import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { assert, expect } from "chai";
import { TaleOfKentridge } from "../target/types/tale_of_kentridge";
import { EventData, Race } from "../types";

const getEventLogs = async (program: anchor.Program, pubKey: PublicKey, log?: boolean) => {
    const signatures = await program.provider.connection
        .getSignaturesForAddress(pubKey, {}, "confirmed")
        .then((res) => res.map(({ signature }) => signature));

    log && console.log("confirmed transactions", signatures);

    const parsedTxs = await anchor.getProvider().connection.getParsedTransactions(signatures, "confirmed");

    if (!parsedTxs || parsedTxs?.every((tx) => tx.meta?.err !== null)) {
        throw new Error("Invalid signature");
    }

    const eventParser = new anchor.EventParser(program.programId, new anchor.BorshCoder(program.idl));

    const events = parsedTxs.map((tx) => eventParser.parseLogs(tx.meta.logMessages, true).next().value);

    return events;
};

describe("TaleOfKentridge", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TaleOfKentridge as Program<TaleOfKentridge>;
    const wallet = provider.wallet;
    const walletSeed = wallet.publicKey.toBuffer();
    const [dataAccount, bump] = PublicKey.findProgramAddressSync([Buffer.from("seed"), walletSeed], program.programId);
    console.log("dataAccount", dataAccount.toString());
    const address1 = wallet.publicKey;
    const address2 = new anchor.web3.PublicKey("Hiy6JSQwKTidbFq5PpeiepThw1vALSFKijYSDCRDDq1L");

    it("Test 1 - Is initialized", async () => {
        try {
            await program.methods.new(walletSeed, [bump]).accounts({ dataAccount }).rpc();

            assert(true, "program initialized");
        } catch (err) {
            console.log(err);
            assert(false, "program failed to initialize");
        }
    });

    it("Test 2 - Onboard player1", async () => {
        await program.methods.onboardPlayer(address1, "9te").accounts({ dataAccount }).rpc();

        const player = await program.methods.getPlayer(address1).accounts({ dataAccount }).view();
        expect(player.nickname).to.equal("9te");
        expect(player.playerAddress.toString()).to.equal(address1.toString());
        expect(player.isPlayerExist).is.true;
        expect(player.isMatching).is.false;
        expect(player.isInGame).is.false;
    });

    it("Test 3 - Onboard player1 but exists", async () => {
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
        await program.methods.onboardPlayer(address2, "phantom").accounts({ dataAccount }).rpc();

        const player = await program.methods.getPlayer(address2).accounts({ dataAccount }).view();
        expect(player.nickname).to.equal("phantom");
        expect(player.playerAddress.toString()).to.equal(address2.toString());
        expect(player.isPlayerExist).is.true;
        expect(player.isMatching).is.false;
        expect(player.isInGame).is.false;
    });

    it("Test 5 - Init cards", async () => {
        try {
            await program.methods.initCards(address1, "ROYAL").accounts({ dataAccount }).rpc();
            await program.methods.initCards(address2, "HUMANOID").accounts({ dataAccount }).rpc();

            assert(true, "Player initialized cards");
        } catch (err) {
            console.log(err);
            assert(false, "Player failed to initialize cards");
        }
    });

    it("Test 6 - Get player's cards", async () => {
        try {
            const cards1 = await program.methods.getPlayerCards(address1).accounts({ dataAccount }).view();
            const cards2 = await program.methods.getPlayerCards(address2).accounts({ dataAccount }).view();

            // console.log(cards1);
            // console.log(cards2);
            expect(cards1.length).to.equal(30);
            expect(cards2.length).to.equal(30);
        } catch (err) {
            console.log(err);
            assert(false, "Failed to get cards");
        }
    });

    it("Test 7 - Player1 enqueue", async () => {
        await program.methods.enqueuePlayer(address1).accounts({ dataAccount }).rpc();
        const player1 = await program.methods.getPlayer(address1).accounts({ dataAccount }).view();
        expect(player1.isMatching).is.true;
        expect(player1.isInGame).is.false;

        const queue = await program.methods.getQueue().accounts({ dataAccount }).view();

        expect(queue.length).to.equal(1);
    });

    it("Test 8 - Player2 enqueue", async () => {
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

    it("Test 9 - Get roomId", async () => {
        const events = await getEventLogs(program, dataAccount);

        const { roomId, player1, player2 } = (events as EventData[]).find((event) => event.name === "JoinRoom").data;
        expect(roomId).to.be.string;
        const address1 = wallet.publicKey;
        const address2 = new anchor.web3.PublicKey("Hiy6JSQwKTidbFq5PpeiepThw1vALSFKijYSDCRDDq1L");
        expect(player1.equals(address1)).to.be.true;
        expect(player2.equals(address2)).to.be.true;
    });

    it("Test 10 - Get players in room", async () => {
        const events = await getEventLogs(program, dataAccount);

        const { roomId, player1, player2 } = (events as EventData[]).find((event) => event.name === "JoinRoom").data;

        const playersInRoom = await program.methods.getPlayersInRoom(roomId).accounts({ dataAccount }).view();
        expect(playersInRoom.length).to.equal(2);
        expect(playersInRoom[0].equals(player1)).to.be.true;
        expect(playersInRoom[1].equals(player2)).to.be.true;
    });
});
