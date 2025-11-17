use crate::models::camt053::{BkToCstmAttribute, NtryAttribute};
pub(crate) struct DocumentMt940 {
    pub(crate) document: Vec<BkToCstmAttribute>
}

impl DocumentMt940 {
    pub(crate) fn extract_field_6x_mt940(record_camt: &BkToCstmAttribute, record_write: &mut String) {
        for balance in &record_camt.stmt.bal {
            match balance.tp.cd_or_party.cd.as_ref() {
                "OPBD" => { record_write.push_str(":60F:") },
                "CLBD" => { record_write.push_str(":62F:") },
                "CLAV" => { record_write.push_str(":62M:") },
                "ITAV" => { record_write.push_str(":64:") },
                "FPAV" => { record_write.push_str(":65:") },
                _ => {}
            }
            record_write.push_str(balance.tp.cd_or_party.cd.as_ref());
            let dt = balance.dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2..10]);
            }
            record_write.push_str(balance.ccy.as_ref());
            record_write.push_str(balance.amt.replace(".", ",").as_ref());
            record_write.push_str("\n");
        }
    }

    pub(crate) fn extract_field_61_86_mt940(record_camt: &Vec<NtryAttribute>, record_write: &mut String) {
        for ntry in record_camt {
            record_write.push_str(":61:");
            let mut dt = ntry.val_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[2..10]);
            }
            dt = ntry.book_dt.dt.replace("-", "");
            if dt.len() >= 10 {
                record_write.push_str(&dt[4..10]);
            }
            if ntry.cdt_dbt_ind == "CRDT" {
                record_write.push_str(":C")
            } else { record_write.push_str(":D") };
            record_write.push_str(ntry.amt.replace(".", ",").as_ref());
            record_write.push_str(ntry.bx_tx_cd.prtry.cd.as_ref());
            if !ntry.ntry_dtls.btch.tx_dtls.is_empty() {
                record_write.push_str(ntry.ntry_dtls.btch.tx_dtls[0].refs.end_to_end_id.as_ref());
            }
            for tx_dtls in &ntry.ntry_dtls.btch.tx_dtls {
                record_write.push_str(":86:/NREF/");
                record_write.push_str(tx_dtls.refs.end_to_end_id.as_ref());
                record_write.push_str("\n");
                if !tx_dtls.rltd_pties.cdtr.nm.is_empty() {
                    record_write.push_str("/CRNM/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr.nm.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.cdtr_acct.other.id.is_empty() {
                    record_write.push_str("/CACT/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_agts.cdtr_agt.bic.is_empty() {
                    record_write.push_str("/CBIC/");
                    record_write.push_str(tx_dtls.rltd_agts.cdtr_agt.bic.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rmt_inf.ustrd.is_empty() {
                    record_write.push_str("/REMI/");
                    record_write.push_str(tx_dtls.rmt_inf.ustrd.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.addtl_tx_inf.is_empty() {
                    record_write.push_str("/OPRP/");
                    record_write.push_str(tx_dtls.addtl_tx_inf.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr_acct.other.id.is_empty() {
                    record_write.push_str("/DACT/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.amt_dtls.amt.is_empty() {
                    record_write.push_str("/OAMT/");
                    record_write.push_str(tx_dtls.amt_dtls.amt.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr.id.othr.id.is_empty() {
                    record_write.push_str("/DCID/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr.id.othr.id.as_ref());
                    record_write.push_str("\n");
                }
            }
        }
    }
}
