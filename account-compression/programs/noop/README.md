<p align="center">
  <a href="https://solana.com">
    <img alt="Safecoin" src="https://i.imgur.com/IKyzQ6T.png" width="250" />
  </a>
</p>

# SPL Noop Rust SDK

This is crate provides a wrapper for invoking `spl-noop`, which does nothing. 
It's primary use is circumventing log truncation when emitting application data by `invoke`-ing `spl-noop` with event data.

`spl-noop` and this crate's implementation are targeted towards supporting [account-compression](https://github.com/fair-exchange/safecoin-program-library/tree/master/account-compression) and may be subject to change.
