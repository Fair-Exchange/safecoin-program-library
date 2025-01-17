# `@safecoin/safe-token`

A TypeScript library for interacting with the SPL Token and Token-2022 programs.

## Links

- [TypeScript Docs](https://solana-labs.github.io/safecoin-program-library/token/js/)
- [FAQs (Frequently Asked Questions)](#faqs)
- [Install](#install)
- [Build from Source](#build-from-source)

## FAQs

### How can I get support?

Please ask questions in the Safecoin Stack Exchange: https://solana.stackexchange.com/

If you've found a bug or you'd like to request a feature, please
[open an issue](https://github.com/fair-exchange/safecoin-program-library/issues/new).

### No export named Token

Please see [upgrading from 0.1.x](#upgrading-from-0.1.x).

## Install

```shell
npm install --save @safecoin/safe-token @safecoin/web3.js
```
_OR_
```shell
yarn add @safecoin/safe-token @safecoin/web3.js
```

## Build from Source

0. Prerequisites

* Node 16+
* NPM 8+

1. Clone the project:
```shell
git clone https://github.com/fair-exchange/safecoin-program-library.git
```

2. Navigate to the library:
```shell
cd safecoin-program-library/token/js
```

3. Install the dependencies:
```shell
npm install
```

4. Build the library:
```shell
npm run build
```

5. Build the on-chain programs:
```shell
npm run test:build-programs
```

6. Run the tests:
```shell
npm run test
```

7. Run the example:
```shell
npm run example
```

## Upgrading

### Upgrading from 0.2.0

There are no breaking changes from 0.2.0, only new functionality for Token-2022.

### Upgrading from 0.1.x

When upgrading from safe-token 0.1.x, you may see the following error in your code:

```
import {TOKEN_PROGRAM_ID, Token, AccountLayout} from '@safecoin/safe-token';
                          ^^^^^
SyntaxError: The requested module '@safecoin/safe-token' does not provide an export named 'Token'
```

The `@safecoin/safe-token` library as of version 0.2.0 does not have the `Token`
class. Instead the actions are split up and exported separately.

To use the old version, install it with:

```
npm install @safecoin/safe-token@0.1.8
```

Otherwise you can find documentation on how to use new versions on the
[SPL docs](https://spl.solana.com/token) or
[Safecoin Cookbook](https://solanacookbook.com/references/token.html).
