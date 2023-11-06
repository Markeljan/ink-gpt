# ink-gpt
🦑ink gpteee◼️

Substrate Contracts UI
Substrate Contracts UI is a web application for deploying and interacting with WASM smart contracts on Substrate blockchains that include the FRAME Contracts Pallet.

❗Only compatible with nodes that use WeightV2. For older nodes use this deployment❗

Supported smart contracts:

ink!
ask!
solang
We recommend running a Substrate Contracts Node for local development or using the Contracts parachain on the Rococo Testnet.

See FAUCETS.md to learn about how to obtain free testnet tokens of supported chains.

Features
Contract instantiation
Once you have a compiled contract and you are connected to a node, you can use the hosted version of Contracts UI to add a new contract on-chain.

There are 2 instantiation methods:

Upload New Contract Code
In Substrate, contract code is stored only once on-chain and shared between instances. If the contract has never been instantiated before in it's current form, you need to choose the Upload New Contract Code option and provide the .contract code bundle that was generated when compiling the ink! contract. The bundle contains the contract metadata and the WASM code. Make sure your versions for the node, ink! and cargo-contract are up to date, otherwise the .contract bundle will be invalid.

Use Existing Contract Code
If you need to re-instantiate a contract from a different owner or for other reasons, you can use the "Use Existing Contract Code" option. You will have to provide a code hash that already exists on-chain or choose one that belongs to a code bundle previously uploaded via our UI. If the code hash is on-chain but the contract was not uploaded via Contracts UI, you will also have to provide the metadata.json file in the next step.

The UI will interpret the contract metadata and output a guided instantiation form.

Interaction
After a successful instantiation, you will be redirected to the contract page where you can call contract messages (methods) and get a log of results.

Run the app locally
Install dependencies

yarn
Run dev server

yarn start
Contributing to the project
Please see our Contribution Guidelines before opening an issue or PR.
