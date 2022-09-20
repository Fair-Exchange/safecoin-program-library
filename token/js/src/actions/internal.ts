import type { Signer } from '@safecoin/web3.js';
import { PublicKey } from '@safecoin/web3.js';

/** @internal */
export function getSigners(signerOrMultisig: Signer | PublicKey, multiSigners: Signer[]): [PublicKey, Signer[]] {
    return signerOrMultisig instanceof PublicKey
        ? [signerOrMultisig, multiSigners]
        : [signerOrMultisig.publicKey, [signerOrMultisig]];
}
