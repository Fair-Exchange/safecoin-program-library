/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@safecoin/web3.js'

/**
 * @category Instructions
 * @category ReplaceLeaf
 * @category generated
 */
export type ReplaceLeafInstructionArgs = {
  root: number[] /* size: 32 */
  previousLeaf: number[] /* size: 32 */
  newLeaf: number[] /* size: 32 */
  index: number
}
/**
 * @category Instructions
 * @category ReplaceLeaf
 * @category generated
 */
export const replaceLeafStruct = new beet.BeetArgsStruct<
  ReplaceLeafInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['root', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['previousLeaf', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['newLeaf', beet.uniformFixedSizeArray(beet.u8, 32)],
    ['index', beet.u32],
  ],
  'ReplaceLeafInstructionArgs'
)
/**
 * Accounts required by the _replaceLeaf_ instruction
 *
 * @property [_writable_] merkleTree
 * @property [**signer**] authority
 * @property [] logWrapper
 * @category Instructions
 * @category ReplaceLeaf
 * @category generated
 */
export type ReplaceLeafInstructionAccounts = {
  merkleTree: web3.PublicKey
  authority: web3.PublicKey
  logWrapper: web3.PublicKey
}

export const replaceLeafInstructionDiscriminator = [
  204, 165, 76, 100, 73, 147, 0, 128,
]

/**
 * Creates a _ReplaceLeaf_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category ReplaceLeaf
 * @category generated
 */
export function createReplaceLeafInstruction(
  accounts: ReplaceLeafInstructionAccounts,
  args: ReplaceLeafInstructionArgs,
  programId = new web3.PublicKey('GRoLLzvxpxxu2PGNJMMeZPyMxjAUH9pKqxGXV9DGiceU')
) {
  const [data] = replaceLeafStruct.serialize({
    instructionDiscriminator: replaceLeafInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.merkleTree,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.authority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: accounts.logWrapper,
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
