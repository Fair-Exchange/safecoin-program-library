import { TOKEN_PROGRAM_ID } from '@safecoin/safe-token';
import { PublicKey, SYSVAR_CLOCK_PUBKEY, TransactionInstruction } from '@safecoin/web3.js';
import { struct, u8 } from '@solana/buffer-layout';
import { LENDING_PROGRAM_ID } from '../constants';
import { u64 } from '@safecoin/buffer-layout-utils';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
    collateralAmount: bigint;
}

const DataLayout = struct<Data>([u8('instruction'), u64('collateralAmount')]);

export const withdrawObligationCollateralInstruction = (
    collateralAmount: number | bigint,
    sourceCollateral: PublicKey,
    destinationCollateral: PublicKey,
    withdrawReserve: PublicKey,
    obligation: PublicKey,
    lendingMarket: PublicKey,
    lendingMarketAuthority: PublicKey,
    obligationOwner: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: LendingInstruction.WithdrawObligationCollateral,
            collateralAmount: BigInt(collateralAmount),
        },
        data
    );

    const keys = [
        { pubkey: sourceCollateral, isSigner: false, isWritable: true },
        { pubkey: destinationCollateral, isSigner: false, isWritable: true },
        { pubkey: withdrawReserve, isSigner: false, isWritable: false },
        { pubkey: obligation, isSigner: false, isWritable: true },
        { pubkey: lendingMarket, isSigner: false, isWritable: false },
        { pubkey: lendingMarketAuthority, isSigner: false, isWritable: false },
        { pubkey: obligationOwner, isSigner: true, isWritable: false },
        { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
        keys,
        programId: LENDING_PROGRAM_ID,
        data,
    });
};
