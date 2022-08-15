import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";
import * as assert from "assert";
import * as bs58 from "bs58";
import {
	ASSOCIATED_PROGRAM_ID,
	TOKEN_PROGRAM_ID,
} from "@project-serum/anchor/dist/cjs/utils/token";
import {
	LAMPORTS_PER_SOL,
	PublicKey,
	Transaction,
	SYSVAR_RENT_PUBKEY,
	sendAndConfirmTransaction,
	SYSVAR_EPOCH_SCHEDULE_PUBKEY,
} from "@solana/web3.js";
import {
	createAssociatedTokenAccount,
	createMint,
	mintTo,
} from "@solana/spl-token";

describe("vesting", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.Vesting as Program<Vesting>;

	const connection = anchor.getProvider().connection;

	// it("#01 can make and claim a vestment (SIMPLE)", async () => {
	// 	// Before sending the transaction to the blockchain.
	// 	const vestor = anchor.web3.Keypair.generate();
	// 	const beneficiary = anchor.web3.Keypair.generate();

	// 	const tx1 = await connection.requestAirdrop(
	// 		vestor.publicKey,
	// 		LAMPORTS_PER_SOL
	// 	);
	// 	const tx2 = await connection.requestAirdrop(
	// 		beneficiary.publicKey,
	// 		LAMPORTS_PER_SOL
	// 	);

	// 	await connection.confirmTransaction(tx1);
	// 	await connection.confirmTransaction(tx2);

	// 	const tokenMint = await createMint(
	// 		connection,
	// 		vestor,
	// 		vestor.publicKey,
	// 		null,
	// 		0
	// 	);

	// 	const vestorTokenAccount = await createAssociatedTokenAccount(
	// 		connection,
	// 		vestor,
	// 		tokenMint,
	// 		vestor.publicKey
	// 	);

	// 	const beneficiaryTokenAccount = await createAssociatedTokenAccount(
	// 		connection,
	// 		beneficiary,
	// 		tokenMint,
	// 		beneficiary.publicKey
	// 	);

	// 	await mintTo(
	// 		connection,
	// 		vestor,
	// 		tokenMint,
	// 		vestorTokenAccount,
	// 		vestor,
	// 		1000
	// 	);

	// 	const [vestedTokens] = await PublicKey.findProgramAddress(
	// 		[Buffer.from("vested-tokens"), beneficiary.publicKey.toBuffer()],
	// 		program.programId
	// 	);

	// 	const [vestment, vestmentBump] = await PublicKey.findProgramAddress(
	// 		[Buffer.from("vestment"), vestedTokens.toBuffer()],
	// 		program.programId
	// 	);

	// 	const tx3 = await program.rpc.makeVestment(
	// 		new anchor.BN(100),
	// 		null,
	// 		new anchor.BN(5),
	// 		4,
	// 		{
	// 			accounts: {
	// 				vestment: vestment,
	// 				vestor: vestor.publicKey,
	// 				vestorTokenAccount: vestorTokenAccount,
	// 				beneficiary: beneficiary.publicKey,
	// 				vestedTokens: vestedTokens,
	// 				vestedTokensMint: tokenMint,
	// 				tokenProgram: TOKEN_PROGRAM_ID,
	// 				rent: SYSVAR_RENT_PUBKEY,
	// 				systemProgram: anchor.web3.SystemProgram.programId,
	// 			},
	// 			signers: [vestor],
	// 		}
	// 	);

	// 	await connection.confirmTransaction(tx3);
	// 	console.log("Vestment state: ");
	// 	console.log(await program.account.vestment.fetch(vestment));

	// 	console.log("Vestor: ");
	// 	console.log(await connection.getTokenAccountBalance(vestorTokenAccount));

	// 	console.log("Vestment: ");
	// 	console.log(await connection.getTokenAccountBalance(vestedTokens));

	// 	await program.rpc.claimVestment({
	// 		accounts: {
	// 			vestment: vestment,
	// 			beneficiary: beneficiary.publicKey,
	// 			beneficiaryTokenAccount: beneficiaryTokenAccount,
	// 			vestedTokens: vestedTokens,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			systemProgram: anchor.web3.SystemProgram.programId,
	// 		},
	// 		signers: [beneficiary],
	// 	});

	// 	console.log("Vestment state: ");
	// 	console.log(await program.account.vestment.fetch(vestment));

	// 	console.log("Beneficiary: ");
	// 	console.log(
	// 		await connection.getTokenAccountBalance(beneficiaryTokenAccount)
	// 	);

	// 	console.log("Vestment: ");
	// 	console.log(await connection.getTokenAccountBalance(vestedTokens));
	// });

	it("#02 can make and claim a vestment (COMPLEX)", async () => {
		const vestor = anchor.web3.Keypair.generate();
		const beneficiary = anchor.web3.Keypair.generate();

		const tx1 = await connection.requestAirdrop(
			vestor.publicKey,
			LAMPORTS_PER_SOL
		);
		const tx2 = await connection.requestAirdrop(
			beneficiary.publicKey,
			LAMPORTS_PER_SOL
		);

		await connection.confirmTransaction(tx1);
		await connection.confirmTransaction(tx2);

		const tokenMint = await createMint(
			connection,
			vestor,
			vestor.publicKey,
			null,
			0
		);

		const vestorTokenAccount = await createAssociatedTokenAccount(
			connection,
			vestor,
			tokenMint,
			vestor.publicKey
		);

		const beneficiaryTokenAccount = await createAssociatedTokenAccount(
			connection,
			beneficiary,
			tokenMint,
			beneficiary.publicKey
		);

		const txMint = await mintTo(
			connection,
			vestor,
			tokenMint,
			vestorTokenAccount,
			vestor,
			1000
		);
		const txMint2 = await mintTo(
			connection,
			beneficiary,
			tokenMint,
			beneficiaryTokenAccount,
			vestor,
			1000
		);

		await connection.confirmTransaction(txMint);
		await connection.confirmTransaction(txMint2);

		console.log("Beneficiary pubkey:\n");
		console.log(beneficiary.publicKey);
		console.log("\n");
		console.log("Vestor SPL balance:\n");
		console.log(await connection.getTokenAccountBalance(vestorTokenAccount));
		console.log("\n");

		const [ledger] = await PublicKey.findProgramAddress(
			[
				Buffer.from("ledger"),
				tokenMint.toBuffer(),
				beneficiary.publicKey.toBuffer(),
			],
			program.programId
		);
		console.log("Ledger pubkey:\n");
		console.log(ledger);
		console.log("\n");

		let ledgercina;
		try {
			ledgercina = await program.account.ledger.fetch(ledger.toString());
			console.log("Ledgercina pubkey:\n");
			console.log(ledgercina);
			console.log("\n");
		} catch (error) {
			const tx0 = await program.rpc.makeLedger({
				accounts: {
					ledger: ledger,
					vestor: vestor.publicKey,
					vestorTokenAccount: vestorTokenAccount,
					beneficiary: beneficiary.publicKey,
					vestedTokensMint: tokenMint,
					tokenProgram: TOKEN_PROGRAM_ID,
					rent: SYSVAR_RENT_PUBKEY,
					systemProgram: anchor.web3.SystemProgram.programId,
				},
				signers: [vestor],
			});
			await connection.confirmTransaction(tx0);
			ledgercina = await program.account.ledger.fetch(ledger.toString());
			console.log("Ledgercina pubkey:\n");
			console.log(ledgercina);
			console.log("\n");
		}
		let ledgerVCount = ledgercina.vestmentCount;
		console.log("Ledger vestment count: \n");
		console.log(ledgerVCount);
		console.log("\n");

		const [oldVestment] = await PublicKey.findProgramAddress(
			[
				Buffer.from("vestment"),
				ledger.toBuffer(),
				new anchor.BN(ledgerVCount).toArrayLike(Buffer), //new anchor.BN(ledgerVCount).toBuffer("le", 8)
			],
			program.programId
		);
		console.log("Old vestment pubkey:\n");
		console.log(oldVestment);
		console.log("\n");

		let oldVestmentcina;
		let newVestment = oldVestment;
		try {
			oldVestmentcina = await program.account.vestment.fetch(oldVestment);
			console.log("Old vestmentcina pubkey:\n");
			console.log(oldVestmentcina);
			console.log("\n");

			if (oldVestmentcina.isActive == true) {
				console.log("A vestment is already active");
				return;
			}
			ledgerVCount++;
			[newVestment] = await PublicKey.findProgramAddress(
				[
					Buffer.from("vestment"),
					ledger.toBuffer(),
					new anchor.BN(ledgerVCount).toArrayLike(Buffer),
				],
				program.programId
			);
		} catch (error) {
			[newVestment] = await PublicKey.findProgramAddress(
				[
					Buffer.from("vestment"),
					ledger.toBuffer(),
					new anchor.BN(1).toArrayLike(Buffer),
				],
				program.programId
			);
		}
		console.log("New vestment pubkey:");
		console.log(newVestment);
		console.log("\n");

		const [vestedTokens] = await PublicKey.findProgramAddress(
			[Buffer.from("vested-tokens"), newVestment.toBuffer()],
			program.programId
		);

		console.log("Vestment token account pubkey:\n");
		console.log(vestedTokens);
		console.log("\n");

		var cliff = 10;
		var numberOfPeriods = 4;
		var period = 5;
		var amount = 100;
		const tx3 = await program.rpc.makeVestment(
			new anchor.BN(amount),
			null,
			new anchor.BN(period),
			numberOfPeriods,
			{
				accounts: {
					ledger: ledger,
					vestment: newVestment,
					vestor: vestor.publicKey,
					vestorTokenAccount: vestorTokenAccount,
					beneficiary: beneficiary.publicKey,
					vestedTokens: vestedTokens,
					vestedTokensMint: tokenMint,
					tokenProgram: TOKEN_PROGRAM_ID,
					rent: SYSVAR_RENT_PUBKEY,
					systemProgram: anchor.web3.SystemProgram.programId,
				},
				signers: [vestor],
			}
		);

		await connection.confirmTransaction(tx3);

		console.log("Ledger state: ");
		console.log(await program.account.ledger.fetch(ledger));

		console.log("Vestment state: ");
		console.log(await program.account.vestment.fetch(newVestment));

		console.log("Vestor: ");
		console.log(await connection.getTokenAccountBalance(vestorTokenAccount));

		console.log("Vestment tokens: ");
		console.log(await connection.getTokenAccountBalance(vestedTokens));

		function sleep(milliseconds) {
			const date = Date.now();
			let currentDate = null;
			do {
				currentDate = Date.now();
			} while (currentDate - date < milliseconds);
		}
		sleep(5000);

		console.log("Ok pre claima");
		// const txClaim1 = await program.rpc.claimVestment({
		// 	accounts: {
		// 		ledger: ledger,
		// 		vestment: newVestment,
		// 		beneficiary: beneficiary.publicKey,
		// 		beneficiaryTokenAccount: beneficiaryTokenAccount,
		// 		vestedTokens: vestedTokens,
		// 		vestedTokensMint: tokenMint,
		// 		tokenProgram: TOKEN_PROGRAM_ID,
		// 		systemProgram: anchor.web3.SystemProgram.programId,
		// 	},
		// 	signers: [beneficiary],
		// });

		const txClaim1 = await program.methods
			.claimVestment()
			.accounts({
				ledger: ledger,
				vestment: newVestment,
				beneficiary: beneficiary.publicKey,
				beneficiaryTokenAccount: beneficiaryTokenAccount,
				vestedTokens: vestedTokens,
				vestedTokensMint: tokenMint,
				tokenProgram: TOKEN_PROGRAM_ID,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([beneficiary])
			.rpc();

		await connection.confirmTransaction(txClaim1);

		console.log("Ledger state: ");
		console.log(await program.account.ledger.fetch(ledger));

		console.log("Vestment state: ");
		console.log(await program.account.vestment.fetch(newVestment));

		console.log("Beneficiary: ");
		console.log(
			await connection.getTokenAccountBalance(beneficiaryTokenAccount)
		);

		console.log("Vestment: ");
		console.log(await connection.getTokenAccountBalance(vestedTokens));

		sleep(5000);

		await program.rpc.claimVestment({
			accounts: {
				ledger: ledger,
				vestment: newVestment,
				beneficiary: beneficiary.publicKey,
				beneficiaryTokenAccount: beneficiaryTokenAccount,
				vestedTokens: vestedTokens,
				vestedTokensMint: tokenMint,
				tokenProgram: TOKEN_PROGRAM_ID,
				systemProgram: anchor.web3.SystemProgram.programId,
			},
			signers: [beneficiary],
		});

		console.log("Ledger state: ");
		console.log(await program.account.ledger.fetch(ledger));

		console.log("Vestment state: ");
		console.log(await program.account.vestment.fetch(newVestment));

		console.log("Beneficiary: ");
		console.log(
			await connection.getTokenAccountBalance(beneficiaryTokenAccount)
		);

		console.log("Vestment: ");
		console.log(await connection.getTokenAccountBalance(vestedTokens));

		// console.log("User tokens: ");
		// console.log(
		//   await connection.getTokenAccountBalance(beneficiaryTokenAccount)
		// );
		// console.log("Vestment tokens: ");
		// console.log(await connection.getTokenAccountBalance(vestedTokens));

		//var currentTimeInSeconds = Math.floor(Date.now() / 1000); //unix timestamp in seconds

		// const passedPeriods = Math.round(
		//   (currentTimeInSeconds - vestmentTimeinSeconds - cliff) / period
		// );
		// console.log("Passed periods: " + passedPeriods);

		// let amountPerPeriod;
		// if (passedPeriods < numberOfPeriods) {
		//   amountPerPeriod = amount / numberOfPeriods;
		// } else {
		//   amountPerPeriod = amount;
		// }

		//console.log("Amount per period: " + amountPerPeriod);

		//let amountToClaim = amountPerPeriod * passedPeriods;
	});
});
