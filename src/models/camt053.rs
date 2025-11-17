use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct DocumentCamt053 {
    #[serde(rename="@xlmns")]
    xlmns: String,
    #[serde(rename="@xlmns:xsi")]
    xmlns_xsi: String,
    #[serde(rename="@xsi:schemaLocation")]
    xsi_schema_location: String,
    pub(crate) bk_to_cstm: Vec<BkToCstmAttribute>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct BkToCstmAttribute{
    pub(crate) grp_hdr: HeaderAttribute,
    pub(crate) stmt: StatementAttribute,
    #[serde(skip_serializing)]
    forward_st_line: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct HeaderAttribute{
    pub(crate) msg_id: String, //message id
    pub(crate) crd_dt_tm: String, //datetime create file
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate)  struct StatementAttribute{
    pub(crate) id: String, //id
    pub(crate)  electr_seq_nb: String, //ElctrncSeqNb
    pub(crate) lgl_seq_nv: String, //LglSeqNb
    cred_dt_tm: String, //CreDtTm
    pub(crate) fr_to_dt: FromToDtAttribute, //FrToDt
    pub(crate) acct: AccAttribute, //Acct
    pub(crate) bal: Vec<BalanceAttribute>, //Bal
    txs_summry: TxsSummryAttribute, //TxsSummry
    pub(crate) ntry: Vec<NtryAttribute> //Ntry

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub (crate) struct NtryAttribute{
    ntry_ref: u32, //NtryRef
    pub(crate) amt: String, //Amt
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String, //Ccy
    pub(crate) cdt_dbt_ind: String, //CdtDbtInd
    sts: String, //Sts
    pub(crate) book_dt: DtAttribute, //BookgDt
    pub(crate) val_dt: DtAttribute, //ValDt
    pub(crate) acct_svrc_ref: String, //AcctSvcrRef
    pub(crate) bx_tx_cd: BxTxCdAttribute, //BkTxCd
    pub(crate) ntry_dtls: NtryDtlsAttribute//NtryDtls

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct NtryDtlsAttribute{
    pub(crate) btch: BtchAttribute, //Btch
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct BtchAttribute{
    nb_of_txs: u32, //NbOfTxs
   pub(crate) tx_dtls: Vec<TxDtlsAttribute>//TxDtls
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct CdtrAgtAttribute{
    pub(crate) cdtr_agt: FinInstIdAttribute,
    pub(crate) dbtr_agt: FinInstIdAttribute,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct EndToEndIdAttribute{
    pub(crate) end_to_end_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct TxAmtAttribute{
    pub(crate) end_to_end_id: String,
    pub(crate) amt: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct RltdPtiesAttribute{
    pub(crate) cdtr_acct: IdTxDtlsAttribute,
    pub(crate) dbtr_acct: IdTxDtlsAttribute,
    pub(crate) cdtr: CdtrAttribue,
    pub(crate) dbtr: DbtrAttribute,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct DbtrAttribute{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct PrvtIdAttribute {
    pub(crate) othr: IdDtldAttribute
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct CdtrAttribue{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct RmtInfAttribute{
    strd: CdtrRefInfAttribute,
    pub(crate) ustrd: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct RltdDtsAttribute{
    pub(crate) accpt_nc_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct CdtrRefInfAttribute{
    tp: CdOrPrtryAttribute,//Tp
    ref_cdtr: String //Ref
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct CdOrPrtryAttribute{
    cd_or_prty: CdAttribute//CdOrPrtry
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct IdTxDtlsAttribute{
    id: String, //Id
    pub(crate) other: IdDtldAttribute//Other
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct IdDtldAttribute{
    pub(crate) id: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct AmtAttribute{
    #[serde(rename="@Ccy")]
    ccy: String//Ccy
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct BxTxCdAttribute{
    domn: DomnAttribute, //Domn
    pub(crate) prtry: PrtryAttribute//Prtry
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct DomnAttribute{
    cd: String, //Cd
    fmly: FmlyAttribute, //Fmly
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct FmlyAttribute{
    cd: String, //Cd
    sub_fmly_cd: String//SubFmlyCd
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct PrtryAttribute{
    pub(crate) cd: String, //cd
    pub(crate) issr: String//Issr
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct TxsSummryAttribute{
    ttl_ntries: TtlNtriesAttribute, //TtlNtries
    ttl_cdt_ntries: TtlCdtDbtNtriesAttribute,//TtlCdtNtries
    ttl_dbt_ntries: TtlCdtDbtNtriesAttribute//TtlDbtNtries

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct TtlNtriesAttribute{
    nb_of_ntries: u32, //NbOfNtries
    ttl_net_ntry_amt: f64,//TtlNetNtryAmt
    cdt_dbt_ind: String//CdtDbtInd

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct TtlCdtDbtNtriesAttribute{
    nb_of_ntries: u32, //NbOfNtries
    sum: f64, //Sum
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct BalanceAttribute{
    pub(crate) tp: TpBalanceAttribute, // tp
    pub(crate) amt: String,
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String,
    pub(crate) dt:  DtAttribute,
    #[serde(skip_serializing)]
    pub(crate) cd: String
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct DtAttribute{
    pub(crate) dt: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct TpBalanceAttribute{
    pub(crate) cd_or_party: CdAttribute
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct CdAttribute{
    pub(crate) cd: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct FromToDtAttribute{
    pub(crate) fr_dt_tm: String, //FrDtTm
    pub(crate) to_dt_tm: String, //ToDtTm
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate)  struct AccAttribute{
    pub(crate) id: IdIbanAttribute, //id
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String, //Ccy
    nm: String, //nm
    pub(crate) ownr: OwnerAttribute, //ownr
    pub(crate) svcr: SvcrAttribute, //svcr


}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct IdIbanAttribute{
    iban: String, //IBAN
    pub(crate) othr: OtherAttribute,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate)  struct SvcrAttribute{
    pub(crate) fin_inst_id: FinInstIdAttribute //FinInstnId
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate)  struct FinInstIdAttribute{
    pub(crate) bic: String, //BIC
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct OwnerAttribute{
    pub(crate) nm: String, //nm
    pstl_addr: PostalAddressAttribute, //pstl_addr
    bldg_nb: u32, //BldgNb
    pst_cd: u32, //PstCd
    twn_nm: String, //TwnNm
    ctry: String, //Ctry
    pub(crate) id: IdAttribute, //Id
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct IdAttribute{
    pub(crate) org_id: OrgIdAttribute//OrgId
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct OrgIdAttribute{
    pub(crate) othr: OtherAttribute //Othr
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub(crate) struct OtherAttribute{
    pub(crate) id: String, //id
    schme_nm: ShemeNumberAttribute //SchmeNm
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct ShemeNumberAttribute{
    cd: String, //cd
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
struct PostalAddressAttribute{
    strt_nm: String, //strt_nm

}

impl DocumentCamt053 {
    pub(crate) fn default() -> Self {
        Self {
            xlmns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xsi_schema_location: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd".to_string(),
            bk_to_cstm: Vec::new()
        }
    }
}

impl BkToCstmAttribute {
    pub(crate) fn default() -> Self{
        Self{
            grp_hdr: HeaderAttribute::default(),
            stmt: StatementAttribute::default(),
            forward_st_line: "ForwardStLine".to_string(),
        }
    }
}

impl HeaderAttribute {
    pub(crate) fn default() -> Self {
        Self {
            msg_id: "MsgId".to_string(),
            crd_dt_tm: "1979-01-01T00:00:00".to_string(),
        }
    }
}

impl StatementAttribute {
    pub(crate) fn default() -> Self{
        Self{
            id: "Id".to_string(),
            electr_seq_nb: "0".to_string(),
            lgl_seq_nv: "0".to_string(),
            cred_dt_tm: "CredDtTm".to_string(),
            fr_to_dt: FromToDtAttribute::default(),
            acct: AccAttribute::default(),
            bal: vec![],
            txs_summry: TxsSummryAttribute::default(),
            ntry: vec![],
        }
    }
}

impl FromToDtAttribute {
    pub(crate) fn default() -> Self {
        Self {
            fr_dt_tm: "FrDtTm".to_string(),
            to_dt_tm: "ToDtTm".to_string(),
        }
    }
}

impl TxsSummryAttribute {   
    pub(crate) fn default() -> Self {
        Self {
            ttl_ntries: TtlNtriesAttribute {
                nb_of_ntries: 0,
                ttl_net_ntry_amt: 0.0,
                cdt_dbt_ind: "CdtDbtInd".to_string(),
            },
            ttl_cdt_ntries: TtlCdtDbtNtriesAttribute { nb_of_ntries: 0, sum: 0.0 },
            ttl_dbt_ntries: TtlCdtDbtNtriesAttribute { nb_of_ntries: 0, sum: 0.0 },
        }
    }
}

impl AccAttribute {
    pub(crate) fn default() -> Self{
        Self{
            id: IdIbanAttribute { iban: "Iban".to_string(),
                othr: OtherAttribute { id: "".to_string(),
                    schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
            ccy: "Ccy".to_string(),
            nm: "Nm".to_string(),
            ownr: OwnerAttribute::default(),
            svcr: SvcrAttribute { fin_inst_id: FinInstIdAttribute { bic: "Bic".to_string() } },
        }
    }
}

impl OwnerAttribute {
    pub(crate) fn default() -> Self {
        Self {
            nm: "Nm".to_string(),
            pstl_addr: PostalAddressAttribute { strt_nm: "StrtNm".to_string() },
            bldg_nb: 0,
            pst_cd: 0,
            twn_nm: "TwnNm".to_string(),
            ctry: "Ctry".to_string(),
            id: IdAttribute { org_id: OrgIdAttribute { 
                othr: OtherAttribute { 
                    id: "Id".to_string(), 
                    schme_nm: ShemeNumberAttribute { 
                        cd: "Cd".to_string() } } } },
        }
    }
    
}

impl BalanceAttribute {
    pub(crate) fn default() -> Self{
        Self{
            tp: TpBalanceAttribute { cd_or_party: CdAttribute { cd: "Cd".to_string() } },
            amt: "".to_string(),
            ccy: "Ccy".to_string(),
            dt: DtAttribute { dt: "DT".to_string() },
            cd: "Cd".to_string(),
        }
    }

}
impl NtryAttribute {
    pub(crate) fn default() -> Self{
        Self{
            ntry_ref: 0,
            amt: "Amt".to_string(),
            ccy: "Ccy".to_string(),
            cdt_dbt_ind: "CdtDbtInd".to_string(),
            sts: "Sts".to_string(),
            book_dt: DtAttribute { dt: "Dt".to_string() },
            val_dt: DtAttribute { dt: "Dt".to_string() },
            acct_svrc_ref: "AcctSvrcRef".to_string(),
            bx_tx_cd: BxTxCdAttribute::default(),
            ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute { nb_of_txs: 0, tx_dtls: vec![] } },
        }
    }
}

impl BxTxCdAttribute {
    fn default() -> Self{
        Self{
            domn: DomnAttribute { cd: "Cd".to_string(),
                fmly: FmlyAttribute { cd: "Cd".to_string(),
                    sub_fmly_cd: "SubFmlyCd".to_string() } },
            prtry: PrtryAttribute { cd: "Cd".to_string(), issr: "Issr".to_string() },


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

impl NtryDtlsAttribute{
    pub(crate) fn default() -> Self{
        Self{
            btch: BtchAttribute { nb_of_txs: 0, tx_dtls: Vec::new() },
        }
    }
}

impl TxDtlsAttribute {
    pub(crate) fn default() -> Self {
        Self {
            refs: EndToEndIdAttribute { end_to_end_id: "".to_string() },
            amt_dtls: TxAmtAttribute { end_to_end_id: "".to_string(), amt: "".to_string() },
            bx_tx_cd: BxTxCdAttribute::default(),
            rltd_pties: RltdPtiesAttribute {
                cdtr_acct: IdTxDtlsAttribute {
                    id: "Id".to_string(),
                    other: IdDtldAttribute {
                        id: "Id".to_string()
                    }
                },
                dbtr_acct: IdTxDtlsAttribute {
                    id: "".to_string(),
                    other: IdDtldAttribute { id: "".to_string() }
                },
                cdtr: CdtrAttribue { id: PrvtIdAttribute { othr: IdDtldAttribute {
                    id: "".to_string() } },
                    nm: "".to_string() },
                dbtr: DbtrAttribute { id: PrvtIdAttribute { othr: IdDtldAttribute {
                    id: "".to_string() } },
                    nm: "".to_string() },
            },
            rmt_inf: RmtInfAttribute {
                strd: CdtrRefInfAttribute {
                    tp: CdOrPrtryAttribute {
                        cd_or_prty: CdAttribute {
                            cd: "".to_string()
                        }
                    },
                    ref_cdtr: "".to_string()
                },
                ustrd: "".to_string(),
            },
            rltd_dts: RltdDtsAttribute { accpt_nc_dt_tm: "".to_string() },
            rltd_agts: CdtrAgtAttribute { cdtr_agt: FinInstIdAttribute { bic: "".to_string() } },
            addtl_tx_inf: "".to_string(),
        }
    }
}

