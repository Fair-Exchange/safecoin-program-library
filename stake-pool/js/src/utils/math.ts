import BN from 'bn.js';
import { LAMPORTS_PER_SAFE } from '@safecoin/web3.js';

export function solToLamports(amount: number): number {
  if (isNaN(amount)) return Number(0);
  return Number(amount * LAMPORTS_PER_SAFE);
}

export function lamportsToSafe(lamports: number | BN): number {
  if (typeof lamports === 'number') {
    return Math.abs(lamports) / LAMPORTS_PER_SAFE;
  }

  let signMultiplier = 1;
  if (lamports.isNeg()) {
    signMultiplier = -1;
  }

  const absLamports = lamports.abs();
  const lamportsString = absLamports.toString(10).padStart(10, '0');
  const splitIndex = lamportsString.length - 9;
  const solString = lamportsString.slice(0, splitIndex) + '.' + lamportsString.slice(splitIndex);
  return signMultiplier * parseFloat(solString);
}
