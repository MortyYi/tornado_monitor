pub enum TokenTypes {
    ETH,
    DAI,
    cDAI,
    USDC,
}

struct Tornado {
    token_type: TokenTypes,
    denomination: f32,
    is_single_contract: bool,
    contracts: Vec<U160>,
}

pub struct Monitor {
    tornados: Vec<Tornado>
}

let monitor_configurator: Vec<Tornado> =  Monitor {
    [
        Tornado {
            token_type: TokenTypes.ETH,
            denomination: 0.1,
            is_single_contract: false,
            contract: ["0x94A1B5CdB22c43faab4AbEb5c74999895464Ddaf", "0xb541fc07bC7619fD4062A54d96268525cBC6FfEF", "0x12D66f87A04A9E220743712cE6d9bB1B5616B8Fc"],
        },
        Tornado {
            token_type: TokenTypes.ETH,
            denomination: 1,
            is_single_contract: true,
            contract: ["0x47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936"],
        },
        Tornado {
            token_type: TokenTypes.ETH,
            denomination: 10,
            is_single_contract: true,
            contract: ["0x910Cbd523D972eb0a6f4cAe4618aD62622b39DbF"],
        },
        Tornado {
            token_type: TokenTypes.ETH,
            denomination: 100,
            is_single_contract: true,
            contract: ["0xA160cdAB225685dA1d56aa342Ad8841c3b53f291"],
        },
        Tornado {
            token_type: TokenTypes.DAI,
            denomination: 100,
            is_single_contract: true,
            contract: ["0xD4B88Df4D29F5CedD6857912842cff3b20C8Cfa3"],
        },
        Tornado {
            token_type: TokenTypes.DAI,
            denomination: 1000,
            is_single_contract: true,
            contract: ["0xFD8610d20aA15b7B2E3Be39B396a1bC3516c7144"],
        },
        Tornado {
            token_type: TokenTypes.DAI,
            denomination: 10000,
            is_single_contract: true,
            contract: ["0xF60dD140cFf0706bAE9Cd734Ac3ae76AD9eBC32A"],
        },
        Tornado {
            token_type: TokenTypes.cDAI,
            denomination: 5000,
            is_single_contract: true,
            contract: ["0x22aaA7720ddd5388A3c0A3333430953C68f1849b"],
        },
        Tornado {
            token_type: TokenTypes.cDAI,
            denomination: 50000,
            is_single_contract: true,
            contract: ["0xBA214C1c1928a32Bffe790263E38B4Af9bFCD659"],
        },
        Tornado {
            token_type: TokenTypes.cDAI,
            denomination: 500000,
            is_single_contract: true,
            contract: ["0xb1C8094B234DcE6e03f10a5b673c1d8C69739A00"],
        },
        Tornado {
            token_type: TokenTypes.USDC,
            denomination: 100,
            is_single_contract: true,
            contract: ["0x4736dCf1b7A3d580672CcE6E7c65cd5cc9cFBa9D"],
        },
        Tornado {
            token_type: TokenTypes.USDC,
            denomination: 1000,
            is_single_contract: true,
            contract: ["0xd96f2B1c14Db8458374d9Aca76E26c3D18364307"],
        }
    ]
}

impl Monitor {
    pub run(&self) {

    }
    pub default() -> Self {
        Self {
            tornados: monitor_configurator
        }
    }
}