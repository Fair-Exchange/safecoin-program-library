import { createMemoInstruction } from '../../src';
import { Connection, Keypair, Transaction, LAMPORTS_PER_SAFE, sendAndConfirmTransaction } from '@safecoin/web3.js';

test('transaction: live', async () => {
    const url = 'http://localhost:8328';
    const connection = new Connection(url, 'confirmed');
    await connection.getVersion();
    const signer = new Keypair(); // also fee-payer

    const airdropSignature = await connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SAFE / 10);
    await connection.confirmTransaction(airdropSignature, 'confirmed');

    const memoTx = new Transaction().add(createMemoInstruction('this is a test memo', [signer.publicKey]));
    await sendAndConfirmTransaction(connection, memoTx, [signer], {
        preflightCommitment: 'confirmed',
    });
});
