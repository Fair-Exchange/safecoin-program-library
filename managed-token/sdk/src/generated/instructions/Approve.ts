/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@safecoin/safe-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@safecoin/web3.js'

/**
 * @category Instructions
 * @category Approve
 * @category generated
 */
export type ApproveInstructionArgs = {
  instructionArgs: beet.bignum
}
/**
 * @category Instructions
 * @category Approve
 * @category generated
 */
export const ApproveStruct = new beet.BeetArgsStruct<
  ApproveInstructionArgs & {
    instructionDiscriminator: number
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['instructionArgs', beet.u64],
  ],
  'ApproveInstructionArgs'
)
/**
 * Accounts required by the _Approve_ instruction
 *
 * @property [] mint
 * @property [_writable_] account
 * @property [**signer**] owner
 * @property [**signer**] upstreamAuthority
 * @property [] delegate
 * @property [] freezeAuthority
 * @category Instructions
 * @category Approve
 * @category generated
 */
export type ApproveInstructionAccounts = {
  mint: web3.PublicKey
  account: web3.PublicKey
  owner: web3.PublicKey
  upstreamAuthority: web3.PublicKey
  delegate: web3.PublicKey
  freezeAuthority: web3.PublicKey
  tokenProgram?: web3.PublicKey
}

export const approveInstructionDiscriminator = 6

/**
 * Creates a _Approve_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category Approve
 * @category generated
 */
export function createApproveInstruction(
  accounts: ApproveInstructionAccounts,
  args: ApproveInstructionArgs,
  programId = new web3.PublicKey('mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS')
) {
  const [data] = ApproveStruct.serialize({
    instructionDiscriminator: approveInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.account,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.owner,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.upstreamAuthority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.delegate,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.freezeAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
