// @flow

import {Keypair, Connection} from '@safecoin/web3.js';

/**
 * Create a new system account and airdrop it some lamports
 *
 * @private
 */
export async function newSystemAccountWithAirdrop(
  connection: Connection,
  lamports: number = 1,
): Promise<Keypair> {
  const account = new Keypair();
  await connection.requestAirdrop(account.publicKey, lamports);
  return account;
}
