import { PublicKey } from '@safecoin/web3.js';

/** Address of the SPL Token program */
export const TOKEN_PROGRAM_ID = new PublicKey('ToKLx75MGim1d1jRusuVX8xvdvvbSDESVaNXpRA9PHN');

/** Address of the SPL Token 2022 program */
export const TOKEN_2022_PROGRAM_ID = new PublicKey('ZToGWcF1Qh9H7te1MmABiGsFUKvj5zXPQ2QnTqoHpHN');

/** Address of the SPL Associated Token Account program */
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('AToD9iqHSc2fhEP9Jp7UYA6mRjHQ4CTWyzCsw8X3tH7K');

/** Address of the special mint for wrapped native SAFE in safe-token */
export const NATIVE_MINT = new PublicKey('Safe111111111111111111111111111111111111111');

/** Address of the special mint for wrapped native SAFE in safe-token-2022 */
export const NATIVE_MINT_2022 = new PublicKey('9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP');

/** Check that the token program provided is not `Tokenkeg...`, useful when using extensions */
export function programSupportsExtensions(programId: PublicKey): boolean {
    if (programId === TOKEN_PROGRAM_ID) {
        return false;
    } else {
        return true;
    }
}
