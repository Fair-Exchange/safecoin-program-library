import { Buffer } from 'buffer';
import { LAMPORTS_PER_SAFE, PublicKey } from '@safecoin/web3.js';

// Public key that identifies the SPL Stake Pool program.
export const STAKE_POOL_PROGRAM_ID = new PublicKey('spNkUopdHp8YxSdsajeNuSKZm51w9kPt48SDDPSHFvo');

// Maximum number of validators to update during UpdateValidatorListBalance.
export const MAX_VALIDATORS_TO_UPDATE = 5;

// Seed for ephemeral stake account
export const EPHEMERAL_STAKE_SEED_PREFIX = Buffer.from('ephemeral');

// Seed used to derive transient stake accounts.
export const TRANSIENT_STAKE_SEED_PREFIX = Buffer.from('transient');

// Minimum amount of staked SAFE required in a validator stake account to allow
// for merges without a mismatch on credits observed
export const MINIMUM_ACTIVE_STAKE = LAMPORTS_PER_SAFE;
