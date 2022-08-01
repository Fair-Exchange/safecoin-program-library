import { TOKEN_PROGRAM_ID } from '@safecoin/safe-token';
import { PublicKey, SYSVAR_CLOCK_PUBKEY, TransactionInstruction } from '@safecoin/web3.js';
import { struct, u8 } from '@safecoin/buffer-layout';
import { LENDING_PROGRAM_ID } from '../constants';
import { u64 } from '@safecoin/buffer-layout-utils';
import { LendingInstruction } from './instruction';

interface Data {
    instruction: number;
    liquidityAmount: bigint;
}

const DataLayout = struct<Data>([u8('instruction'), u64('liquidityAmount')]);

export const depositReserveLiquidityInstruction = (
    liquidityAmount: number | bigint,
    sourceLiquidity: PublicKey,
    destinationCollateral: PublicKey,
    reserve: PublicKey,
    reserveLiquiditySupply: PublicKey,
    reserveCollateralMint: PublicKey,
    lendingMarket: PublicKey,
    lendingMarketAuthority: PublicKey,
    transferAuthority: PublicKey
): TransactionInstruction => {
    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: LendingInstruction.DepositReserveLiquidity,
            liquidityAmount: BigInt(liquidityAmount),
        },
        data
    );

    const keys = [
        { pubkey: sourceLiquidity, isSigner: false, isWritable: true },
        { pubkey: destinationCollateral, isSigner: false, isWritable: true },
        { pubkey: reserve, isSigner: false, isWritable: true },
        { pubkey: reserveLiquiditySupply, isSigner: false, isWritable: true },
        { pubkey: reserveCollateralMint, isSigner: false, isWritable: true },
        { pubkey: lendingMarket, isSigner: false, isWritable: false },
        { pubkey: lendingMarketAuthority, isSigner: false, isWritable: false },
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
