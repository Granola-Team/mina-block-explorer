use leptos::*;
use leptos_router::*;

use crate::common::{components::*, functions::*};
use serde::{Deserialize, Serialize};

#[derive(Params, PartialEq)]
struct URLParams {
    ledger_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StakesResponse {
    data: Vec<Stake>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Stake {
    stake: f64,
    block_chance: String,
    delegates: i32,
    name: String,
    percent_of_stake: String,
    id: Id,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    delegate: String,
}

impl TableData for StakesResponse {
    fn get_columns(&self) -> Vec<String> {
        vec![
            String::from("Key"),
            String::from("Name"),
            String::from("Stake"),
            String::from("% Of Total Stake"),
            String::from("Block Win %"),
            String::from("Delegators"),
        ]
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        let mut rows = Vec::new();
        for stake in &self.data {
            let data = vec![
                stake.id.delegate.to_string(),
                stake.name.to_string(),
                stake.stake.to_string(),
                stake.percent_of_stake.to_string(),
                stake.block_chance.to_string(),
                stake.delegates.to_string(),
            ]
            .into_iter()
            .map(convert_to_span)
            .collect();
            rows.push(data);
        }
        rows
    }
}

// async fn load_data(ledger_hash: &str) -> Result<StakesResponse, MyError> {
//     let response = reqwest::get("https://minaexplorer.com/staking-data/".to_owned()+ledger_hash)
//         .await
//         .map_err(|e| MyError::NetworkError(e.to_string()))?;

//     if response.status().is_success() {
//         let summary = response
//             .json::<StakesResponse>()
//             .await
//             .map_err(|e| MyError::ParseError(e.to_string()))?;
//         Ok(summary)
//     } else {
//         Err(MyError::NetworkError("Failed to fetch data".into()))
//     }
// }

fn get_data() -> Result<StakesResponse, serde_json::Error> {
    let data = r#"
    {
        "draw": 2,
        "recordsTotal": 147880,
        "recordsFiltered": 147880,
        "data": [
            {
                "id": {
                    "delegate": "B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9"
                },
                "stake": 101598797.2216,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "9.26%",
                "blockChance": "12.04%"
            },
            {
                "id": {
                    "delegate": "B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN"
                },
                "stake": 66512700.95540742,
                "delegates": 6556,
                "name": "MinaExplorer",
                "percentOfStake": "6.06%",
                "blockChance": "8.06%"
            },
            {
                "id": {
                    "delegate": "B62qjsFTBw4TVwRRxNVrmwJfQqXfmMC4DVa2moCe9f8ErvBYd6f7npr"
                },
                "stake": 65523133.70641628,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "5.97%",
                "blockChance": "7.94%"
            },
            {
                "id": {
                    "delegate": "B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6"
                },
                "stake": 39954694.71314049,
                "delegates": 14847,
                "name": "Auro Wallet",
                "percentOfStake": "3.64%",
                "blockChance": "4.92%"
            },
            {
                "id": {
                    "delegate": "B62qoA5XwfEVnXbcrzphGH1TVuqxeJ5bhX7vTS3hcxpQFHnStG3MQk9"
                },
                "stake": 36986872.65819663,
                "delegates": 46,
                "name": "Finoa Consensus Services",
                "percentOfStake": "3.37%",
                "blockChance": "4.56%"
            },
            {
                "id": {
                    "delegate": "B62qopHVr6nGCsQgrvsBsoxDm1E5CEdMkDSN3jneRnxKpR5iiXnTbas"
                },
                "stake": 36979152.796303585,
                "delegates": 24,
                "name": "InfStones",
                "percentOfStake": "3.37%",
                "blockChance": "4.56%"
            },
            {
                "id": {
                    "delegate": "B62qj287L1bwP9XguURbxW5cneTRD8Kde4vx3fbeZCNxNxyMzXdsYLP"
                },
                "stake": 35208410.44654576,
                "delegates": 11,
                "name": "Unknown",
                "percentOfStake": "3.21%",
                "blockChance": "4.35%"
            },
            {
                "id": {
                    "delegate": "B62qmjZSQHakvWz7ZMkaaVW7ye1BpxdYABAMoiGk3u9bBaLmK5DJPkR"
                },
                "stake": 26050344.870069284,
                "delegates": 7,
                "name": "Coinlist Wallet",
                "percentOfStake": "2.37%",
                "blockChance": "3.24%"
            },
            {
                "id": {
                    "delegate": "B62qqx9cufbZEn5hDgfftjURCX3sTZYMuRLXVQYCPWL9P9hwXbrju1f"
                },
                "stake": 22131330.732494995,
                "delegates": 3,
                "name": "Polychain",
                "percentOfStake": "2.02%",
                "blockChance": "2.76%"
            },
            {
                "id": {
                    "delegate": "B62qrYipbTfEx5GoJf99uU2iAcW2jgAvnoy1Wrj4WeMEnnZutTiKhDe"
                },
                "stake": 21299232.483663946,
                "delegates": 3,
                "name": "Unknown",
                "percentOfStake": "1.94%",
                "blockChance": "2.65%"
            },
            {
                "id": {
                    "delegate": "B62qqV16g8s744GHM6Dph1uhW4fggYwyvtDnVSoRUyYqNvTir3Rqqzx"
                },
                "stake": 15744386.979750432,
                "delegates": 403,
                "name": "Minascan Pool | Staketab",
                "percentOfStake": "1.43%",
                "blockChance": "1.97%"
            },
            {
                "id": {
                    "delegate": "B62qmkGkjvFwmkqv6erSmTGMx9ABhuxJqpCi4gyUtxFDwif97j2X5zp"
                },
                "stake": 15616555.879847348,
                "delegates": 4,
                "name": "Mina Community",
                "percentOfStake": "1.42%",
                "blockChance": "1.95%"
            },
            {
                "id": {
                    "delegate": "B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5GFasVQ"
                },
                "stake": 14409252.747335799,
                "delegates": 9,
                "name": "Kraken Wallet",
                "percentOfStake": "1.31%",
                "blockChance": "1.80%"
            },
            {
                "id": {
                    "delegate": "B62qrQiw9JhUumq457sMxicgQ94Z1WD9JChzJu19kBE8Szb5T8tcUAC"
                },
                "stake": 13056250.113537757,
                "delegates": 3287,
                "name": "Piconbello",
                "percentOfStake": "1.19%",
                "blockChance": "1.64%"
            },
            {
                "id": {
                    "delegate": "B62qpbpoWmfjZ12jA8TYrS8QktRDYGYjz2cj41tuUe1f2UP2wfpKvXs"
                },
                "stake": 11370369.78032723,
                "delegates": 3,
                "name": "Unknown",
                "percentOfStake": "1.04%",
                "blockChance": "1.43%"
            },
            {
                "id": {
                    "delegate": "B62qpYmDbDJAyADVkJzydoz7QeZy1ZTiWeH1LSuyMxXezvu5mAQi53U"
                },
                "stake": 10653393.09284456,
                "delegates": 551,
                "name": "Everstake",
                "percentOfStake": "0.97%",
                "blockChance": "1.34%"
            },
            {
                "id": {
                    "delegate": "B62qj9SeVBtbZCeGUVznWUqECvFrFU8DoHLQxFtHXX1Dk1BB7dfW4AP"
                },
                "stake": 10112397.59622088,
                "delegates": 2,
                "name": "Binance Staking",
                "percentOfStake": "0.92%",
                "blockChance": "1.27%"
            },
            {
                "id": {
                    "delegate": "B62qrHzjcZbYSsrcXVgGko7go1DzSEBfdQGPon5X4LEGExtNJZA4ECj"
                },
                "stake": 9635079.011988772,
                "delegates": 75,
                "name": "ZKValidator",
                "percentOfStake": "0.88%",
                "blockChance": "1.21%"
            },
            {
                "id": {
                    "delegate": "B62qs2Lw5WZNSjd8eHBUZXFYyRjV8oKtrZMFDn1S1Ye62G71xCQJMYM"
                },
                "stake": 9465953.7598279,
                "delegates": 301,
                "name": "P2P.ORG",
                "percentOfStake": "0.86%",
                "blockChance": "1.19%"
            },
            {
                "id": {
                    "delegate": "B62qijDC2gCTtcqYGnUAc9YgH2Uw4fzr8xEKKL4faZmWyAypgEe3oWC"
                },
                "stake": 8212167.062123263,
                "delegates": 90,
                "name": "Bit Cat\ud83d\udc31",
                "percentOfStake": "0.75%",
                "blockChance": "1.03%"
            },
            {
                "id": {
                    "delegate": "B62qq6ZYPG5JsjZnGJ3pADmRn6hU6qy13EhraTSymjSgyEDwoDR9Gd6"
                },
                "stake": 7846052.572830256,
                "delegates": 2185,
                "name": "Carbonara \ud83c\udf5d- WeStake.club\u26a1\ufe0f",
                "percentOfStake": "0.71%",
                "blockChance": "0.99%"
            },
            {
                "id": {
                    "delegate": "B62qiburnzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzmp7r7UN6X"
                },
                "stake": 7457968.377536917,
                "delegates": 2,
                "name": "Mina Burn Address",
                "percentOfStake": "0.68%",
                "blockChance": "0.94%"
            },
            {
                "id": {
                    "delegate": "B62qkj3CCj2V3pBPPufCuWSkoP2RwjAZkgPZfLKFUae3hPoBEhRtmyo"
                },
                "stake": 7004495.07363868,
                "delegates": 4,
                "name": "BiXinKeLePool",
                "percentOfStake": "0.64%",
                "blockChance": "0.88%"
            },
            {
                "id": {
                    "delegate": "B62qpsikYYhTaAXw8XdgGhQtLsnecHF89LZdW2bGTa4aj4mePWPHxPe"
                },
                "stake": 6862894.66691833,
                "delegates": 66,
                "name": "6block",
                "percentOfStake": "0.63%",
                "blockChance": "0.86%"
            },
            {
                "id": {
                    "delegate": "B62qqHrWLyKdivLz1gCF9YX85dqCsX6E5NymcjctbA8zfxvi57WXw82"
                },
                "stake": 6571322.794665816,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.60%",
                "blockChance": "0.83%"
            },
            {
                "id": {
                    "delegate": "B62qrEEjPjUoSQcfVQkF3vmY4Mo8zbRPiGxVBzXKLgRFxoe6ZugzK5e"
                },
                "stake": 6403053.619344531,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.58%",
                "blockChance": "0.81%"
            },
            {
                "id": {
                    "delegate": "B62qkz3wMhNri9NyNQjs9XG8EEPHhJDDWKfRX9ZutYtMyu4UvudkHV8"
                },
                "stake": 6352152.637448998,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.58%",
                "blockChance": "0.80%"
            },
            {
                "id": {
                    "delegate": "B62qouNvgzGaA3fe6G9mKtktCfsEinqj27eqTSvDu4jSKReDEx7A8Vx"
                },
                "stake": 5863634.433094579,
                "delegates": 1,
                "name": "Binance Wallet",
                "percentOfStake": "0.53%",
                "blockChance": "0.74%"
            },
            {
                "id": {
                    "delegate": "B62qm2hqYFLcx2vxrPR1szE7dBq1AY1aoiKKz4wbbpC8JW4YQM7p8Ge"
                },
                "stake": 5543630.312617465,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.50%",
                "blockChance": "0.70%"
            },
            {
                "id": {
                    "delegate": "B62qiW9Qwv9UnKfNKdBm6hRLNDobv46rVhX1trGdB35YCNT33CSCVt5"
                },
                "stake": 5498764.736232303,
                "delegates": 1,
                "name": "Bybit",
                "percentOfStake": "0.50%",
                "blockChance": "0.69%"
            },
            {
                "id": {
                    "delegate": "B62qmzHPH3PU2LBvmn51aYw1dMyg8Noms9WsEknLeDWT8ZnCKBfN8NT"
                },
                "stake": 4797726.296448079,
                "delegates": 1,
                "name": "Crypto.com Wallet",
                "percentOfStake": "0.44%",
                "blockChance": "0.60%"
            },
            {
                "id": {
                    "delegate": "B62qnSjFGjZ39C8iDGMErbFSX5NvKVyvjvHxvf7U1oDmyeU6ynxXZZF"
                },
                "stake": 4765107.495904759,
                "delegates": 586,
                "name": "PhDSOON",
                "percentOfStake": "0.43%",
                "blockChance": "0.60%"
            },
            {
                "id": {
                    "delegate": "B62qrae3PEBj66KV2obWnzVxMjDCMuFWnyzxEzvLkQutaKPmWtfUPm3"
                },
                "stake": 4633896.569374602,
                "delegates": 247,
                "name": "atomi * 0% fee",
                "percentOfStake": "0.42%",
                "blockChance": "0.58%"
            },
            {
                "id": {
                    "delegate": "B62qm7vP2JPj1d8XDmGUiv3GtwAfzuaxrdNsiXdWmZ7QqXZtzpVyGPG"
                },
                "stake": 4436443.720482315,
                "delegates": 1,
                "name": "Gate Wallet",
                "percentOfStake": "0.40%",
                "blockChance": "0.56%"
            },
            {
                "id": {
                    "delegate": "B62qmFf6UZn2sg3j8bYLGmMinzS2FHX6hDM71nFxAfMhvh4hnGBtkBD"
                },
                "stake": 4039370.343276643,
                "delegates": 163,
                "name": "Chorus One",
                "percentOfStake": "0.37%",
                "blockChance": "0.51%"
            },
            {
                "id": {
                    "delegate": "B62qjSytpSK7aEauBprjXDSZwc9ai4YMv9tpmXLQK14Vy941YV36rMz"
                },
                "stake": 3575248.35735872,
                "delegates": 2345,
                "name": "SleziSatoshi",
                "percentOfStake": "0.33%",
                "blockChance": "0.45%"
            },
            {
                "id": {
                    "delegate": "B62qjCuPisQjLW7YkB22BR9KieSmUZTyApftqxsAuB3U21r3vj1YnaG"
                },
                "stake": 3452527.53552133,
                "delegates": 685,
                "name": "Figment",
                "percentOfStake": "0.31%",
                "blockChance": "0.44%"
            },
            {
                "id": {
                    "delegate": "B62qs2P91UjdhngetBJ57C56HQ8t5V7ECAYWBvpkaC45ovXNgnzqfG6"
                },
                "stake": 3332126.520186265,
                "delegates": 581,
                "name": "TheNOP.io",
                "percentOfStake": "0.30%",
                "blockChance": "0.42%"
            },
            {
                "id": {
                    "delegate": "B62qrmF3BrDqZ3k9wckyf4x6qAdZTsBRXQcXsgn5uR4tCeGzUREFkbK"
                },
                "stake": 3178676.279285188,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.29%",
                "blockChance": "0.40%"
            },
            {
                "id": {
                    "delegate": "B62qqWTHw1LzB52Z52Xu9TZmLgctX2jWWuvKsSuaPhjmgAmDsjNpGdh"
                },
                "stake": 3055553.44959389,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.28%",
                "blockChance": "0.39%"
            },
            {
                "id": {
                    "delegate": "B62qrecVjpoZ4Re3a5arN6gXZ6orhmj1enUtA887XdG5mtZfdUbBUh4"
                },
                "stake": 2880234.640497075,
                "delegates": 16,
                "name": "Unknown",
                "percentOfStake": "0.26%",
                "blockChance": "0.36%"
            },
            {
                "id": {
                    "delegate": "B62qoy8z1RnC9PNwfz4JLLjmjnFmC8HQnnXzwZKqfq3BbVFN86FPPhB"
                },
                "stake": 2772802.451781268,
                "delegates": 1,
                "name": "MEXC Wallet",
                "percentOfStake": "0.25%",
                "blockChance": "0.35%"
            },
            {
                "id": {
                    "delegate": "B62qj28AitWwLTU3HAhtoW34nJ6LkyHU7XKm5wC84q1RtF4ho1yEgGn"
                },
                "stake": 2566169.9826796637,
                "delegates": 8,
                "name": "HashQuark",
                "percentOfStake": "0.23%",
                "blockChance": "0.32%"
            },
            {
                "id": {
                    "delegate": "B62qjr26Ff74z8uAr95iAfhG4PGdhzN9ZHcFtUhbUV5RHrXt8E1GQLL"
                },
                "stake": 2419937.165556,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.22%",
                "blockChance": "0.31%"
            },
            {
                "id": {
                    "delegate": "B62qpXqPzauUXLnsAQFnYHMCiV9pRqG2wqbJ4pL936SVANHa66zkkQj"
                },
                "stake": 2370866.096025339,
                "delegates": 164,
                "name": "LowFeeValidation",
                "percentOfStake": "0.22%",
                "blockChance": "0.30%"
            },
            {
                "id": {
                    "delegate": "B62qoazqR1ag2hDwjkSSm6qV3eJtkiPvVPKhfVyeea7TehBAWu4dWJ5"
                },
                "stake": 2344052.728312571,
                "delegates": 76,
                "name": "StakeYourMina - Dheeraj",
                "percentOfStake": "0.21%",
                "blockChance": "0.30%"
            },
            {
                "id": {
                    "delegate": "B62qqD9f8CGy5QeFj1h3nsit3Zp2GDBntjEoVCdfj4SPQuubRW1CYXp"
                },
                "stake": 2331737.919062426,
                "delegates": 362,
                "name": "TowerStake",
                "percentOfStake": "0.21%",
                "blockChance": "0.29%"
            },
            {
                "id": {
                    "delegate": "B62qqch9XkiTS8BLUDSM1sayfXNAtnYnQFChktYG1bfCJkDMUqs98Xr"
                },
                "stake": 2298135.011157231,
                "delegates": 34,
                "name": "BitNordic",
                "percentOfStake": "0.21%",
                "blockChance": "0.29%"
            },
            {
                "id": {
                    "delegate": "B62qmyesxfYnG3SY9KET8jmEzK5PTWRZQhTpcfcywkeUJbzWXqqXEbE"
                },
                "stake": 2170761.020732186,
                "delegates": 1,
                "name": "Unknown",
                "percentOfStake": "0.20%",
                "blockChance": "0.27%"
            },
            {
                "id": {
                    "delegate": "B62qm4C3DDfZr4EFKsWgMYkLAsXytAhnVzBUjJojNm9nyv9ApdJmcsc"
                },
                "stake": 2157552.605535889,
                "delegates": 9,
                "name": "Unknown",
                "percentOfStake": "0.20%",
                "blockChance": "0.27%"
            }
        ]
    }"#;

    let r: Result<StakesResponse, serde_json::Error> = serde_json::from_str(data);

    r
}

#[component]
pub fn StakesPage() -> impl IntoView {
    let data = get_data();

    view! {
        {match data {
            Ok(data) => view! {
                <TableSection section_heading="Stakes".to_owned()>
                    <Table data=data/>
                </TableSection>
             },
            Err(err) => view! {<div> { format!("Error: {:#?}", err)}</div> }.into_view()
        }}
    }
}
