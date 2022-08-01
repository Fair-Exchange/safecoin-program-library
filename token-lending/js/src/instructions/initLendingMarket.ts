import { TOKEN_PROGRAM_ID } from '@safecoin/safe-token';
import { PublicKey, SYSVAR_RENT_PUBKEY, TransactionInstruction } from '@safecoin/web3.js';
import { blob, struct, u8 } from '@safecoin/buffer-layout';
import { LENDING_PROGRAM_ID, ORACLE_PROGRAM_ID } from '../constants';
import { publicKey } from '@safecoin/buffer-layout-utils';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
    owner: PublicKey;
    quoteCurrency: Uint8Array;
}

const DataLayout = struct<Data>([u8('instruction'), publicKey('owner'), blob(32, 'quoteCurrency')]);

export const initLendingMarketInstruction = (
    owner: PublicKey,
    quoteCurrency: Uint8Array,
    lendingMarket: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: LendingInstruction.InitLendingMarket,
            owner,
            quoteCurrency,
        },
        data
    );

    const keys = [
        { pubkey: lendingMarket, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: ORACLE_PROGRAM_ID, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
        keys,
        programId: LENDING_PROGRAM_ID,
        data,
    });
};
