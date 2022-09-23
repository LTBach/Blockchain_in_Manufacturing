
function getConfig(network) {
    switch (network) {
        case "development":
        case "testnet":
            return {
                networkId: 'testnet',
                nodeUrl: 'https://rpc.testnet.near.org',
                walletUrl: 'https://wallet.testnet.near.org',
                helperUrl: 'https://helper.testnet.near.org',
                explorerUrl: 'https://explorer.testnet.near.org',
                paymentContract: "uit-payment-contract.vbidev.testnet",
                account: {
                    accountId: "hidang.testnet",
                    privateKey: "ed25519:XYtCwJRrm8YyRocxvsNDS6BgEBdjDmfnhmpVSNWViuJhkQpeSfJLLLWk5aNDPEcyjfq7f3KWrnhQUC3e85FVASw" // cat ~/.near-credentials/testnet/vbidev.testnet.json
                }
            }

        // case "production":
        // case "mainnet":
        //     return {
        //         networkId: 'mainnet',
        //         nodeUrl: 'https://rpc.mainnet.near.org',
        //         walletUrl: 'https://wallet.near.org',
        //         helperUrl: 'https://helper.mainnet.near.org',
        //         explorerUrl: 'https://explorer.mainnet.near.org',
        //         paymentContract: "uit-payment-contract.vbidev.near",
        //         account: {
        //             accountId: "vbidev.near",
        //             privateKey: "private_key"
        //         }
        //     }
        default:
            throw Error("Can not get network config");
            break;
    }
}


export default {
    mongodb: {
        // url: "mongodb+srv://vunguyen:1Qaz2Wsx3Edc@pikavu.syfpkbq.mongodb.net/ecommerce",
        url: "mongodb+srv://hidang:Xt0rlZMUD5drnwdx@cluster0.sfp9dfc.mongodb.net/?retryWrites=true&w=majority",
        options: {}
    },
    getConfig
}