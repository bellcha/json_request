use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BankData{
    meta: Meta,
    data: Vec<Data>,
    totals: Count
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count{
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta{
    total: i32,
    parameters: Parameters,
    index: Index,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data{
    data: FinancialData,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters{
    filters: String,
    fields: String,
    limit: String,
    offset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index{
    name: String,
    #[serde(rename = "createTimestamp")]
    create_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialData {
    #[serde(rename = "REPDTE")]
    report_date: String,
    #[serde(rename = "ASSET")]
    asset: i32,
    #[serde(rename = "EEFFQR")]
    eff_ratio: Option<f32>,
    #[serde(rename = "NETINC")]
    net_income: i32,
    #[serde(rename = "DEP")]
    total_deposits: i32,
    #[serde(rename = "LNLSNET")]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
    total_loans: i32,
    #[serde(rename = "ROAQ")]
    roa: f32,
    #[serde(rename = "ROEQ")]
    roe: f32,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{

    let bank_data:BankData = reqwest::Client::new()
        .get("https://banks.data.fdic.gov/api/financials?filters=CERT%3A12203&fields=REPDTE%2CASSET%2CEEFFQR%2CDEP%2CNETINC%2CLNLSNET%2CROAQ%2CROEQ&sort_by=REPDTE&sort_order=DESC&limit=10000&offset=0&agg_limit=1&format=json")
        .send()
        .await?
        .json()
        .await?;

        for i in bank_data.data{

            if i.data.asset > 600000{

            println!("Total Assets: {:#?}", i.data.asset);
            println!("Total Loans: {:#?}", i.data.total_loans);
            println!("Total Deposits: {:#?}", i.data.total_deposits);
            println!("Net Income: {:#?}", i.data.net_income);
            println!("\n")
            }
        }      

        Ok(())
    
}