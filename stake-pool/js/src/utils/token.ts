import { Connection, PublicKey, TransactionInstruction } from '@safecoin/web3.js';
import {
  AccountInfo,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MintInfo,
  Token,
  TOKEN_PROGRAM_ID,
} from '@safecoin/safe-token';
import { AccountLayout } from '../layouts';

const FAILED_TO_FIND_ACCOUNT = 'Failed to find account';
const INVALID_ACCOUNT_OWNER = 'Invalid account owner';

export async function getTokenMint(
  connection: Connection,
  tokenMintPubkey: PublicKey,
): Promise<MintInfo | undefined> {
  // @ts-ignore
  const token = new Token(connection, tokenMintPubkey, TOKEN_PROGRAM_ID, null);
  return token.getMintInfo();
}

/**
 * Retrieve the associated account or create one if not found.
 * This account may then be used as a `transfer()` or `approve()` destination
 */
export async function addAssociatedTokenAccount(
  connection: Connection,
  owner: PublicKey,
  mint: PublicKey,
  instructions: TransactionInstruction[],
) {
  const associatedAddress = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint,
    owner,
  );

  let rentFee = 0;

  // This is the optimum logic, considering TX fee, client-side computation,
  // RPC roundtrips and guaranteed idempotent.
  // Sadly we can't do this atomically;
  try {
    const account = await connection.getAccountInfo(associatedAddress);
    if (!account) {
      // noinspection ExceptionCaughtLocallyJS
      throw new Error(FAILED_TO_FIND_ACCOUNT);
    }
  } catch (err: any) {
    // INVALID_ACCOUNT_OWNER can be possible if the associatedAddress has
    // already been received some lamports (= became system accounts).
    // Assuming program derived addressing is safe, this is the only case
    // for the INVALID_ACCOUNT_OWNER in this code-path
    if (err.message === FAILED_TO_FIND_ACCOUNT || err.message === INVALID_ACCOUNT_OWNER) {
      instructions.push(
        Token.createAssociatedTokenAccountInstruction(
          ASSOCIATED_TOKEN_PROGRAM_ID,
          TOKEN_PROGRAM_ID,
          mint,
          associatedAddress,
          owner,
          owner,
        ),
      );
      rentFee = await connection.getMinimumBalanceForRentExemption(AccountLayout.span);
    } else {
      throw err;
    }
    console.warn(err);
  }

  return {
    associatedAddress,
    rentFee,
  };
}

export async function getTokenAccount(
  connection: Connection,
  tokenAccountAddress: PublicKey,
  expectedTokenMint: PublicKey,
): Promise<AccountInfo | void> {
  try {
    const account = await connection.getAccountInfo(tokenAccountAddress);
    if (!account) {
      // noinspection ExceptionCaughtLocallyJS
      throw new Error(`Invalid account ${tokenAccountAddress.toBase58()}`);
    }
    const tokenAccount = AccountLayout.decode(account.data) as AccountInfo;
    if (tokenAccount.mint?.toBase58() != expectedTokenMint.toBase58()) {
      // noinspection ExceptionCaughtLocallyJS
      throw new Error(
        `Invalid token mint for ${tokenAccountAddress}, expected mint is ${expectedTokenMint}`,
      );
    }
    return tokenAccount;
  } catch (error) {
    console.log(error);
  }
}
