import { Keypair, Connection } from "@safecoin/web3.js";

export async function newAccountWithLamports(
  connection: Connection,
  lamports = 1000000
): Promise<Keypair> {
  const account = new Keypair();
  const signature = await connection.requestAirdrop(
    account.publicKey,
    lamports
  );
  await connection.confirmTransaction(signature, "singleGossip");
  return account;
}
