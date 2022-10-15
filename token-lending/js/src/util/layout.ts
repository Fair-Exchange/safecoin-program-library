import { AccountInfo, PublicKey } from '@safecoin/web3.js';

export type Parser<T> = (
    pubkey: PublicKey,
    info: AccountInfo<Uint8Array>
) =>
    | {
          pubkey: PublicKey;
          info: AccountInfo<Uint8Array>;
          data: T;
      }
    | undefined;
