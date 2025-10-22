pub enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

struct BaseAttribute{
    account_number: String,
    account_owner: String,
    currency: String,
    address_owner: String,
    bik_swift: String,
    period_list: String,
    
}

struct BalanceInfo {
}

struct TransactionInfo {
}

struct ContragentInfo{
    name: String,
    inn: String,
    kpp: String,
    ogrn: String,
    address: String,
    bank: String,
    country: String,
}

struct AdditionInfo{

}
