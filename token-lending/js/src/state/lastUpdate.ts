import { struct } from '@safecoin/buffer-layout';
import { bool, u64 } from '@safecoin/buffer-layout-utils';

export interface LastUpdate {
    slot: bigint;
    stale: boolean;
}

/** @internal */
export const LastUpdateLayout = struct<LastUpdate>([u64('slot'), bool('stale')], 'lastUpdate');
