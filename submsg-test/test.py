import cwpy
import requests
import yaml, json
import base64

def get_umlg_from_faucet(addr):
    data = {"denom":"umlg", "address":addr} 
    r = requests.post("https://faucet.malaga-420.cosmwasm.com/credit", json=data)
    if "ok" in r.text:
        return True
    else:
        return False

def code_id_from_log(log):
    logs = json.loads(yaml.load(log, Loader=yaml.FullLoader)['raw_log'])[0]
    code_id = logs["events"][-1]["attributes"][0]["value"]
    return int(code_id)
    
def store(_filename):
    filename = _filename
    with open(filename, "rb") as f:
        wasmBytes = f.read()
    res = w.tx_wasm_store("wooz3k", wasmBytes)
    code_id = code_id_from_log(res)
    return code_id

def instantiate(_imsg, code_id):
    imsg = _imsg
    res = w.tx_wasm_instantiate("wooz3k", code_id, "wooz3k", imsg, 100)
    logs = json.loads(yaml.load(res, Loader=yaml.FullLoader)['raw_log'])[0]
    for e in logs['events']:
        if e["type"] == "instantiate":
            o = { x["key"]:x["value"] for x in e["attributes"]}
            assert(int(o["code_id"]) == code_id)
            return o["_contract_address"]

def execute(_msg, contract_addr):
    msg = _msg
    res = w.tx_wasm_execute("wooz3k", contract_addr, msg, 1000)
    print(res)

def query_smart(qry, contract_addr):
    res = w.query_contract_state_smart(contract_addr, qry)
    print(res)

if __name__ == "__main__":
    w = cwpy.wallet("malaga-420", "https://rpc.malaga-420.cosmwasm.com:443")
    mnemonic = "praise convince exercise deputy rookie disagree grocery leopard sure tumble bronze cave text spend box chalk antique music slight vendor require rib host pool"
    w.add_key_with_mnemonic("wooz3k", mnemonic)
    addr = w.get_key("wooz3k")
    #print(addr)
    get_umlg_from_faucet(addr)

    #path = "/mnt/c/Users/wooz3k/Desktop/cosmwasm/submsg-test/artifacts/"

    # _filename = path + "submsg_test.wasm"
    # submsg_code_id = store(_filename)
    # init_test = b"{}"
    # submsg_addr = instantiate(init_test, submsg_code_id)

    #submsg_code_id = 1802
    #submsg_addr = "wasm18jpc79h0vtfra033ae3m445x9lr0m7wakxvqhslt89vx002dyygs2q4d80"

    path = "/mnt/c/Users/wooz3k/Desktop/cosmwasm/flow-test/artifacts/"
    _filename = path + "flow_test.wasm"
    flow_code_id = store(_filename)
    init_test = b"{}"
    flow_addr = instantiate(init_test, flow_code_id)
    #print(flow_addr)
    #flow_addr = "wasm1mt0j5r5xa5t7dc53a7lgfww0d36wrkq9hhqd5f7n3pv6u02hplms7nhjl2"
    test_msg = json.dumps({"flow": {}})
    execute(test_msg, flow_addr)

    '''path = "/mnt/c/Users/wooz3k/Desktop/cosmwasm/reentry/artifacts/"
    _filename = path + "reentry.wasm"
    reentry_code_id = store(_filename)
    init_test = b"{}"
    print(instantiate(init_test, reentry_code_id))'''

    # _filename = path + "terraswap_pair.wasm"
    # pair_code_id = store(_filename)
    # pair_code_id = 1761

    # _filename = path + "terraswap_factory.wasm"
    # factory_code_id = store(_filename)
    # factory_code_id = 1762

    # init_factory = b"""{ "pair_code_id": 1761, "token_code_id": 1760 }"""
    # wooz3k_factory = instantiate(init_factory, 1762)
    # wooz3k_factory = "wasm1gzs0yla3tftkrexp8rdpk3s66m0d9kz0qluslcd04mgzyx2mhtnsuq0wv8"
    
    # native_msg = json.dumps({"add_native_token_decimals": {"denom": "umlg", "decimals": 6}})
    # execute(native_msg, wooz3k_factory)

    # create_msg = json.dumps({ "create_pair": { "asset_infos": [ { "native_token": { "denom": "umlg" } }, { "token": { "contract_addr": wooz3k_token } } ] } })
    # execute(create_msg, wooz3k_factory)

    #wooz3k_pair = "wasm1taueqyzzeq27tr5mh50e2nsxqu55tut88u5rpl099fc65njqa8wq57ygnk"
    #lptoken_addr = "wasm1rkhcftp36ap9hk4d2kcn6jp7gdfrgk5mj5re3p4nz8he3q2eqlpsnx9m54"

    #approve_msg = json.dumps({"increase_allowance":{"spender": wooz3k_pair, "amount": str(2**127-1), "expires": None}})
    #execute(approve_msg, wooz3k_token)

    #liquidity_msg = json.dumps({ "provide_liquidity": {"assets": [{"info": {"token": {"contract_addr": wooz3k_token}}, "amount": "100"}, {"info": {"native_token": {"denom": "umlg"}}, "amount": "100"}]}})
    #execute(liquidity_msg, wooz3k_pair)

    #balance_msg = json.dumps({"balance": {"address": addr}})
    #print(query_smart(balance_msg, wooz3k_token))

    #send_msg = json.dumps({ "send": { "contract": wooz3k_pair, "amount": "100", "msg": base64.b64encode(json.dumps({"swap": {"belief_price": None, "max_spread": None, "to_addr": None}}).encode()).decode()}})
    #execute(send_msg, wooz3k_token)

    #swap_msg = json.dumps({"swap": {"offer_asset": {"info": {"native_token": {"denom": "umlg"}}, "amount": "100"}}})
    #execute(swap_msg, wooz3k_pair)

    #balance_msg = json.dumps({"balance": {"address": addr}})
    #print(query_smart(balance_msg, wooz3k_token))
