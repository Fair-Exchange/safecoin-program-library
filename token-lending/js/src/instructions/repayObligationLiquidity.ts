import { TOKEN_PROGRAM_ID } from '@safecoin/safe-token';
import { PublicKey, SYSVAR_CLOCK_PUBKEY, TransactionInstruction } from '@safecoin/web3.js';
import { struct, u8 } from '@solana/buffer-layout';
import { LENDING_PROGRAM_ID } from '../constants';
import { u64 } from '@safecoin/buffer-layout-utils';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
    liquidityAmount: bigint;
}

const DataLayout = struct<Data>([u8('instruction'), u64('liquidityAmount')]);

export const repayObligationLiquidityInstruction = (
    liquidityAmount: number | bigint,
    sourceLiquidity: PublicKey,
    destinationLiquidity: PublicKey,
    repayReserve: PublicKey,
    obligation: PublicKey,
    lendingMarket: PublicKey,
    transferAuthority: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: LendingInstruction.RepayObligationLiquidity,
            liquidityAmount: BigInt(liquidityAmount),
        },
        data
    );

    const keys = [
        { pubkey: sourceLiquidity, isSigner: false, isWritable: true },
        { pubkey: destinationLiquidity, isSigner: false, isWritable: true },
        { pubkey: repayReserve, isSigner: false, isWritable: true },
        { pubkey: obligation, isSigner: false, isWritable: true },
        { pubkey: lendingMarket, isSigner: false, isWritable: false },
        { pubkey: transferAuthority, isSigner: true, isWritable: false },
        { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ];

    return new TransactionInstruction({
        keys,
        programId: LENDING_PROGRAM_ID,
        data,
    });
};
