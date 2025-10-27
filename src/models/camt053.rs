use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Document{
    #[serde(rename="@xlmns")]
    xlmns: String,
    bk_to_cstm: BkToCstmAttribute
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct BkToCstmAttribute{
    grp_hdr: HeaderAttribute,
    stmt: StatementAttribute,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct HeaderAttribute{
    msg_id: String, //message id
    crd_dt_tm: String, //datetime create file
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct StatementAttribute{
    id: String, //id
    electr_seq_nb: u32, //ElctrncSeqNb
    lgl_seq_nv: u32, //LglSeqNb
    cred_dt_tm: String, //CreDtTm
    fr_to_dt: FromToDtAttribute, //FrToDt
    acct: AccAttribute, //Acct
    bal: Vec<BalanceAttribute>, //Bal
    txs_summry: TxsSummryAttribute, //TxsSummry
    ntry: Vec<NtryAttribute> //Ntry

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct NtryAttribute{
    ntry_ref: u32, //NtryRef
    amt: f64, //Amt
    #[serde(rename="@Ccy")]
    ccy: String, //Ccy
    cdt_dbt_ind: String, //CdtDbtInd
    sts: String, //Sts
    book_dt: DtAttribute, //BookgDt
    val_dt: DtAttribute, //ValDt
    acct_svrc_ref: String, //AcctSvcrRef
    bx_tx_cd: BxTxCdAttribute, //BkTxCd
    ntry_dtls: NtryDtlsAttribute//NtryDtls

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct NtryDtlsAttribute{
    btch: BtchAttribute, //Btch
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct BtchAttribute{
    nb_of_txs: u32, //NbOfTxs
    tx_dtls: Vec<TxDtlsAttribute>//TxDtls
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
enum TxDtlsAttribute {
    Refs { end_to_end_id: String },
    AmtDtls { tx_amt: AmtAttribute },
    BxTxCd { partry: CdAttribute },
    RltdPties { cdtr_acct: IdTxDtlsAttribute },
    RmtInf { strd: CdtrRefInfAttribute },
    RltdDts { accpt_nc_dt_tm: String }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CdtrRefInfAttribute{
    tp: CdOrPrtryAttribute,//Tp
    ref_cdtr: String //Ref
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CdOrPrtryAttribute{
    cd_or_prty: CdAttribute//CdOrPrtry
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct IdTxDtlsAttribute{
    id: String, //Id
    other: IdDtldAttribute//Other
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct IdDtldAttribute{
    id: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AmtAttribute{
    #[serde(rename="@Ccy")]
    ccy: f64 //Ccy
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct BxTxCdAttribute{
    domn: DomnAttribute, //Domn
    prtry: PrtryAttribute//Prtry
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct DomnAttribute{
    cd: String, //Cd
    fmly: FmlyAttribute, //Fmly
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FmlyAttribute{
    cd: String, //Cd
    sub_fmly_cd: String//SubFmlyCd
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PrtryAttribute{
    cd: String, //cd
    issr: String//Issr
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TxsSummryAttribute{
    ttl_ntries: TtlNtriesAttribute, //TtlNtries
    ttl_cdt_ntries: TtlCdtDbtNtriesAttribute,//TtlCdtNtries
    ttl_dbt_ntries: TtlCdtDbtNtriesAttribute//TtlDbtNtries

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TtlNtriesAttribute{
    nb_of_ntries: u32, //NbOfNtries
    ttl_net_ntry_amt: f64,//TtlNetNtryAmt
    cdt_dbt_ind: String//CdtDbtInd

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TtlCdtDbtNtriesAttribute{
    nb_of_ntries: u32, //NbOfNtries
    sum: f64, //Sum
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct BalanceAttribute{
    tp: TpBalanceAttribute, // tp
    amt: f64,
    #[serde(rename="@Ccy")]
    ccy: String,
    dt:  DtAttribute
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct DtAttribute{
    dt: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TpBalanceAttribute{
    cd_or_party: CdAttribute
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CdAttribute{
    cd: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FromToDtAttribute{
    fr_dt_tm: String, //FrDtTm
    to_dt_tm: String, //ToDtTm
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AccAttribute{
    id: IdIbanAttribute, //id
    #[serde(rename="@Ccy")]
    ccy: String, //Ccy
    nm: String, //nm
    ownr: OwnerAttribute, //ownr
    svcr: SvcrAttribute, //svcr


}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct IdIbanAttribute{
    iban: String, //IBAN
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct SvcrAttribute{
    fin_inst_id: FinInstIdAttribute //FinInstnId
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FinInstIdAttribute{
    bic: String, //BIC
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct OwnerAttribute{
    nm: String, //nm
    pstl_addr: PostalAddressAttribute, //pstl_addr
    bldg_nb: u32, //BldgNb
    pst_cd: u32, //PstCd
    twn_nm: String, //TwnNm
    ctry: String, //Ctry
    id: IdAttribute, //Id
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct IdAttribute{
    org_id: OrgIdAttribute//OrgId
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct OrgIdAttribute{
    othr: OtherAttribute //Othr
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct OtherAttribute{
    id: String, //id
    schme_nm: ShemeNumberAttribute //SchmeNm
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ShemeNumberAttribute{
    cd: String, //cd
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PostalAddressAttribute{
    strt_nm: String, //strt_nm

}
