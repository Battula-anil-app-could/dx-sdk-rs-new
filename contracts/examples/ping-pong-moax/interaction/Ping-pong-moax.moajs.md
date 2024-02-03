# Ping-pong

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@dharitrinetwork/moajs');
let { moaSys, Moax, wallets: { alice, bob, carol, dan } } = await moajs.setupInteractive("local-testnet");

let pingPong = await moaSys.loadWrapper("contracts/examples/ping-pong-moax");

await pingPong.sender(alice).gas(150_000_000).call.deploy(Moax(0.5), 2 * 60, null, Moax(1.5));

await pingPong.gas(20_000_000).sender(alice).value(Moax(0.5)).ping("note 1");

await pingPong.sender(bob).value(Moax(0.5)).ping(null);
await pingPong.sender(carol).value(Moax(0.5)).ping(null);

// this fails because of the balance limit of 1.5 moax
await pingPong.sender(dan).value(Moax(0.5).ping(null);

await pingPong.pongAll();

```
