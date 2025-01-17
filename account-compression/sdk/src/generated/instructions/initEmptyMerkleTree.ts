/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@safecoin/web3.js';

/**
 * @category Instructions
 * @category InitEmptyMerkleTree
 * @category generated
 */
export type InitEmptyMerkleTreeInstructionArgs = {
  maxDepth: number;
  maxBufferSize: number;
};
/**
 * @category Instructions
 * @category InitEmptyMerkleTree
 * @category generated
 */
export const initEmptyMerkleTreeStruct = new beet.BeetArgsStruct<
  InitEmptyMerkleTreeInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['maxDepth', beet.u32],
    ['maxBufferSize', beet.u32],
  ],
  'InitEmptyMerkleTreeInstructionArgs'
);
/**
 * Accounts required by the _initEmptyMerkleTree_ instruction
 *
 * @property [_writable_] merkleTree
 * @property [**signer**] authority
 * @property [] noop
 * @category Instructions
 * @category InitEmptyMerkleTree
 * @category generated
 */
export type InitEmptyMerkleTreeInstructionAccounts = {
  merkleTree: web3.PublicKey;
  authority: web3.PublicKey;
  noop: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const initEmptyMerkleTreeInstructionDiscriminator = [
  191, 11, 119, 7, 180, 107, 220, 110,
];

/**
 * Creates a _InitEmptyMerkleTree_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitEmptyMerkleTree
 * @category generated
 */
export function createInitEmptyMerkleTreeInstruction(
  accounts: InitEmptyMerkleTreeInstructionAccounts,
  args: InitEmptyMerkleTreeInstructionArgs,
  programId = new web3.PublicKey('cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK')
) {
  const [data] = initEmptyMerkleTreeStruct.serialize({
    instructionDiscriminator: initEmptyMerkleTreeInstructionDiscriminator,
    ...args,
  });
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
      pubkey: accounts.noop,
      isWritable: false,
      isSigner: false,
    },
  ];

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc);
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
