import 'dotenv/config'
import { SigningCosmWasmClient, Secp256k1HdWallet } from "cosmwasm"
import * as fs from "fs";
import { Decimal } from "@cosmjs/math";
//import { Coin } from "coin";
// This is your rpc endpoint
const rpcEndpoint = "https://testnet-rpc.orai.io"

const mnemonic = process.env.MNEMONIC

async function main() {
    const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: "orai" })
    const client = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        wallet,
        {
            gasPrice: {
                denom: "orai",
                //minimum fee per gas
                amount: Decimal.fromUserInput("0.001", 6)
            }
        }
    );
    const account = await wallet.getAccounts()
    const address = account[0].address
    // get orai balance
    console.log(await client.getBalance(address, "orai"))

    // địa chỉ ví contract sau khi đã deploy
    const contract_address = process.env.CONTRACT_ADDRESS
    //const tokenContract = "orai1u356paa3dhadknurayc0dyf8x7k5cdsr6trvcmxe2a5eyyc6yufqutkp65";
    const fee = "auto"
    //=====================================DEPLOY========================================

    //wasm -> wasmCode
    const path = "./artifacts/smart_wallet.wasm"
    //const path = "/home/hieu/project_name/artifacts/project_name.wasm"
    const wasmCode = new Uint8Array(fs.readFileSync(path))

    //upload code on chain
    const upload = await client.upload(address, wasmCode, fee)
    console.log(upload)

    
    // const instantiate_msg = {
    //     owner: address,
    //     deposit_token : "orai1u356paa3dhadknurayc0dyf8x7k5cdsr6trvcmxe2a5eyyc6yufqutkp65",
    // };
    // const InstantiateOptions ={
    //     admin : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx",
    // }
    // const res = await client.instantiate(address, upload.codeId, instantiate_msg, "cosmwasm-base", fee,InstantiateOptions)
    // console.log(res)
    

    const migrate_msg = {
            migrate:{}
            //staker_info :{user : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
            //get_rewardamount :{staker : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
    } 
    const res = await client.migrate(address,contract_address,upload.codeId,migrate_msg,fee)
    console.log(res)

    
    // const query_msg = {
    //     native_token_info :{user : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
    //     //token_info :{user : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
    // }
    // const query_example = await client.queryContractSmart(contract_address,query_msg)
    // console.log(query_example)

    // const execute_msg = {   
    //     // deposit_token : {
    //     //     amount : "100",
    //     // }
    //     // with_draw_token : {
    //     // }
    //     // deposit_native_token :{
    //     // }
    //     with_draw_native_token : {
    //     }
    // } 
    //const funds = Coin('orai','0.01');
    //  const execute_example = await client.execute(address, contract_address, execute_msg,fee,'',[{ denom: "orai", amount: "1" }]);
    //  //const execute_example = await client.execute(address, contract_address, execute_msg,fee);
    //   console.log(execute_example)

     const query_msg1 = {
        native_token_info :{user : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
        //token_info :{user : "orai15d8rnqeywwy6c0vkj3fyd8lw6tudfrzgkh2yrx"}
    } 
    const query_example1 = await client.queryContractSmart(contract_address,query_msg1)
    console.log(query_example1)
    //===================================================================================
}


main();