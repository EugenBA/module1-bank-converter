pub enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}
struct BkToCstmAttribute{
    grp_hdr: HeaderAttribute,
    stmt: StatementAttribute,
}

struct HeaderAttribute{
    msg_id: String, //message id
    crd_dt_tm: String, //datetime create file
 }

struct StatementAttribute{
    id: String, //id
    electr_seq_nb: u32, //ElctrncSeqNb
    lgl_seq_nv: u32, //LglSeqNb
    cred_dt_tm: String, //CreDtTm
    fr_to_dt: FromToDtAttribute, //FrToDt
    acct: AccAttribute, //Acct

}

struct FromToDtAttribute{
    fr_dt_tm: String, //FrDtTm
    to_dt_tm: String, //ToDtTm
}

struct AccAttribute{
    id_iban: String, //id/IBAN
    ccy: String, //Ccy
    nm: String, //nm
    ownr: OwnerAttribute, //ownr
    svcr: SvcrAttribute, //svcr

}

struct SvcrAttribute{
    fin_inst_id: FinInstIdAttribute//FinInstnId
}

struct FinInstIdAttribute{
    bic: String, //BIC
}

struct OwnerAttribute{
    nm: String, //nm
    pstl_addr: PostalAddressAttribute, //pstl_addr
    bldg_nb: u32, //BldgNb
    pst_cd: u32, //PstCd
    twn_nm: String, //TwnNm
    ctry: String, //Ctry
    id: IdAttribute, //Id
}


struct IdAttribute{
    org_id: OrgIdAttribute//OrgId
}

struct OrgIdAttribute{
    othr: OtherAttribute //Othr
}

struct OtherAttribute{
    id: String, //id
    schme_nm: ShemeNumberAttribute //SchmeNm
}

struct ShemeNumberAttribute{
    cd: String, //cd
}
struct PostalAddressAttribute{
    strt_nm: String, //strt_nm


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
