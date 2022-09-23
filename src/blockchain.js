import config from "./configs";
import { keyStores, utils, connect, transactions, providers } from "near-api-js";

async function getAccountConnected(params) {
    let network = "testnet";
    let networkConfig = config.getConfig(network);

    // Tao keyPair và keyStore
    const keyPair = utils.KeyPair.fromString(networkConfig.account.privateKey);
    const keyStore = new keyStores.InMemoryKeyStore();

    keyStore.setKey(network, networkConfig.account.accountId, keyPair);

    // Connect account
    const near = await connect({
        keyStore,
        headers: {},
        ...networkConfig
    });

    return await near.account(networkConfig.account.accountId);
}

// near view uit-payment-contract.vbidev.testnet get_order '{"order_id": "order_1"}'
async function view(receipent, method, params) {
    let account = await getAccountConnected();
    return await account.viewFunction(
        receipent,
        method,
        params
    );
}

// near call uit-payment-contract.vbidev.testnet pay_order '{"order_id": "order_1", "order_amount": "100000"}' --accountId vbidev.testnet --deposit 1 --gas 30000000000
async function call(receipent, method, params, attacted_deposit, attacted_gas) {
    let account = await getAccountConnected();

    return await account.functionCall({
        contractId: receipent,
        methodName: method,
        args: params,
        gas: attacted_gas,
        attachedDeposit: attacted_deposit
    });
}

async function getSignUrl(account_id, method, params, deposit, gas, receiver_id, meta, callback_url, network) {
    if(!network) network = "testnet";
        console.log("Params: ", params);
        const deposit_value = typeof deposit == 'string' ? deposit : utils.format.parseNearAmount('' + deposit);
        const actions = [method === '!transfer' ? transactions.transfer(deposit_value) : transactions.functionCall(method, Buffer.from(JSON.stringify(params)), gas, deposit_value)];
        const keypair = utils.KeyPair.fromRandom('ed25519');
        const provider = new providers.JsonRpcProvider({url: 'https://rpc.' + network + '.near.org'});
        const block = await provider.block({finality: 'final'});
        const txs = [transactions.createTransaction(account_id, keypair.publicKey, receiver_id, 1, actions, utils.serialize.base_decode(block.header.hash))];
        const newUrl = new URL('sign', 'https://wallet.' + network + '.near.org/');
        newUrl.searchParams.set('transactions', txs.map(transaction =>utils.serialize.serialize(transactions.SCHEMA, transaction)).map(serialized => Buffer.from(serialized).toString('base64')).join(','));
        newUrl.searchParams.set('callbackUrl', callback_url);
        if (meta)
            newUrl.searchParams.set('meta', meta);
        return newUrl.href;
}
function parseNearAmount(amount){
    return utils.format.parseNearAmount('' + amount);
}

export default {
    getSignUrl,
    view,
    call,
    parseNearAmount
}