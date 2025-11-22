use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct DocumentCamt053 {
    #[serde(rename="@xlmns")]
    xlmns: String,
    #[serde(rename="@xlmns:xsi")]
    xmlns_xsi: String,
    #[serde(rename="@xsi:schemaLocation")]
    xsi_schema_location: String,
    pub(crate) bk_to_cstmr_stmt: Vec<BkToCstmrStmt>
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BkToCstmrStmt {
    pub(crate) grp_hdr: HeaderAttribute,
    pub(crate) stmt: StatementAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct HeaderAttribute{
    pub(crate) msg_id: String, //message id
    pub(crate) cre_dt_tm: String, //datetime create file
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct StatementAttribute{
    pub(crate) id: String, //id
    pub(crate) elctrnc_seq_nb: String, //ElctrncSeqNb
    pub(crate) lgl_seq_nv: String, //LglSeqNb
    cre_dt_tm: String, //CreDtTm
    pub(crate) fr_to_dt: FromToDtAttribute, //FrToDt
    pub(crate) acct: AcctAttribute, //Acct
    pub(crate) bal: Vec<BalanceAttribute>, //Bal
    pub(crate) txs_summry: TxsSummryAttribute, //TxsSummry
    pub(crate) ntry: Vec<NtryAttribute> //Ntry

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub (crate) struct NtryAttribute{
    ntry_ref: u32, //NtryRef
    pub(crate) amt: String, //Amt
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String, //Ccy
    pub(crate) cdt_dbt_ind: String, //CdtDbtInd
    sts: String, //Sts
    pub(crate) bookg_dt: DtAttribute, //BookgDt
    pub(crate) val_dt: DtAttribute, //ValDt
    pub(crate) acct_svcr_ref: String, //AcctSvcrRef
    pub(crate) bx_tx_cd: BxTxCdAttribute, //BkTxCd
    pub(crate) ntry_dtls: NtryDtlsAttribute//NtryDtls

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct NtryDtlsAttribute{
    pub(crate) btch: BtchAttribute, //Btch
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BtchAttribute{
    nb_of_txs: u32, //NbOfTxs
   pub(crate) tx_dtls: Vec<TxDtlsAttribute>//TxDtls
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxDtlsAttribute {
    pub(crate) refs: EndToEndIdAttribute, //Refs
    pub(crate) amt_dtls: TxAmtAttribute, //AmtDtls
    bx_tx_cd: BxTxCdAttribute, //BxTxCd
    pub(crate) rltd_pties: RltdPtiesAttribute, //RltdPties
    pub(crate) rmt_inf: RmtInfAttribute, //RmtInf
    rltd_dts: RltdDtsAttribute, //RltdDts
    pub(crate) rltd_agts: CdtrAgtAttribute, //RltdAgts
    pub(crate) addtl_tx_inf: String, //AddtlTxInf
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdtrAgtAttribute{
    pub(crate) cdtr_agt: FinInstIdAttribute,
    pub(crate) dbtr_agt: FinInstIdAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct EndToEndIdAttribute{
    pub(crate) end_to_end_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxAmtAttribute{
    pub(crate) end_to_end_id: String,
    pub(crate) amt: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RltdPtiesAttribute{
    pub(crate) cdtr_acct: IdTxDtlsAttribute,
    pub(crate) dbtr_acct: IdTxDtlsAttribute,
    pub(crate) cdtr: CdtrAttribue,
    pub(crate) dbtr: DbtrAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct DbtrAttribute{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrvtIdAttribute {
    pub(crate) othr: IdDtldAttribute
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdtrAttribue{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RmtInfAttribute{
    strd: CdtrRefInfAttribute,
    pub(crate) ustrd: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RltdDtsAttribute{
    pub(crate) accpt_nc_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct CdtrRefInfAttribute{
    tp: CdOrPrtryAttribute,//Tp
    ref_cdtr: String //Ref
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct CdOrPrtryAttribute{
    cd_or_prtry: CdAttribute//CdOrPrtry
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdTxDtlsAttribute{
    id: String, //Id
    pub(crate) other: IdDtldAttribute//Other
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdDtldAttribute{
    pub(crate) id: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct AmtAttribute{
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String,//Ccy
    pub(crate) amt: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BxTxCdAttribute{
    domn: DomnAttribute, //Domn
    pub(crate) prtry: PrtryAttribute//Prtry
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct DomnAttribute{
    cd: String, //Cd
    fmly: FmlyAttribute, //Fmly
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct FmlyAttribute{
    cd: String, //Cd
    sub_fmly_cd: String//SubFmlyCd
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrtryAttribute{
    pub(crate) cd: String, //cd
    pub(crate) issr: String//Issr
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxsSummryAttribute{
    pub(crate) ttl_ntries: TtlNtriesAttribute, //TtlNtries
    pub(crate) ttl_cdt_ntries: TtlCdtDbtNtriesAttribute,//TtlCdtNtries
    pub(crate) ttl_dbt_ntries: TtlCdtDbtNtriesAttribute//TtlDbtNtries

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TtlNtriesAttribute{
    pub(crate) nb_of_ntries: String, //NbOfNtries
    ttl_net_ntry_amt: f64,//TtlNetNtryAmt
    cdt_dbt_ind: String//CdtDbtInd

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TtlCdtDbtNtriesAttribute{
    pub(crate) nb_of_ntries: u32, //NbOfNtries
    pub(crate) sum: String, //Sum
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BalanceAttribute{
    pub(crate) tp: TpBalanceAttribute, // tp
    pub(crate) amt: AmtAttribute,
    pub(crate) dt:  DtAttribute,
    #[serde(skip_serializing)]
    pub(crate) cd: String
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct DtAttribute{
    pub(crate) dt: String,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TpBalanceAttribute{
    pub(crate) cd_or_party: CdAttribute
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdAttribute{
    pub(crate) cd: String,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct FromToDtAttribute{
    pub(crate) fr_dt_tm: String, //FrDtTm
    pub(crate) to_dt_tm: String, //ToDtTm
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct AcctAttribute {
    pub(crate) id: IdIbanAttribute, //id
    pub(crate) ccy: String, //Ccy
    nm: String, //nm
    pub(crate) ownr: OwnerAttribute, //ownr
    pub(crate) svcr: SvcrAttribute, //svcr


}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
pub(crate) struct IdIbanAttribute{
    #[serde(rename="IBAN")]
    iban: String, //IBAN
    #[serde(rename="Othr")]
    pub(crate) othr: OtherAttribute,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct SvcrAttribute{
    pub(crate) fin_instn_id: FinInstIdAttribute //FinInstnId
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
pub(crate)  struct FinInstIdAttribute{
    #[serde(rename = "BIC")]
    pub(crate) bic: String, //BIC
    #[serde(rename = "PascalCase")]
    pub(crate) nm: String
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OwnerAttribute{
    pub(crate) nm: String, //nm
    pstl_addr: PostalAddressAttribute, //pstl_addr
    bldg_nb: u32, //BldgNb
    pst_cd: u32, //PstCd
    twn_nm: String, //TwnNm
    ctry: String, //Ctry
    pub(crate) id: IdAttribute, //Id
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdAttribute{
    pub(crate) org_id: OrgIdAttribute//OrgId
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OrgIdAttribute{
    pub(crate) othr: OtherAttribute //Othr
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OtherAttribute{
    pub(crate) id: String, //id
    schme_nm: ShemeNumberAttribute //SchmeNm
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct ShemeNumberAttribute{
    cd: String, //cd
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
struct PostalAddressAttribute{
    strt_nm: String, //strt_nm

}

impl Default for DocumentCamt053{
    fn default() -> Self {
        Self {
            xlmns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xsi_schema_location: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd".to_string(),
            bk_to_cstmr_stmt: Vec::new()
        }
    }
}

impl DtAttribute {
    pub(crate) fn format_dt(dt_str: &str) -> Self {
        Self {
            dt: if dt_str.len() > 5 {
                format!("20{}-{}-{}", dt_str[0..2].to_string(),
                        dt_str[2..4].to_string(), dt_str[4..6].to_string())
            } else {
                "1979-01-01".to_string()
            }
        }
    }
}
