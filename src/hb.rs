use alloc::vec::Vec;
use nx::result::*;
use crate::ns::{ApplicationControlProperty, ApplicationId, ApplicationRecord, ApplicationView, ApplicationViewDeprecated, ApplicationViewWithPromotionInfo, PromotionInfo};

pub const fn gen_application_view(app_id: ApplicationId) -> ApplicationView {
    ApplicationView {
        app_id,
        unk_1: 0,
        unk_flags: 0xF13,
        unk_2: [0; 0x10],
        unk_3: 0,
        unk_4: 0,
        unk_reserved: [0; 0x2],
        unk_6: [0; 0x8],
        unk_7: [0; 0x10],
        unk_8: 0,
        unk_9: 0,
        unk_reserved_2: [0; 0xB]
    }
}

pub const fn gen_deprecated_application_view(app_id: ApplicationId) -> ApplicationViewDeprecated {
    convert_application_view_to_deprecated(gen_application_view(app_id))
}

pub const fn gen_application_view_with_promotion_info(app_id: ApplicationId) -> ApplicationViewWithPromotionInfo {
    ApplicationViewWithPromotionInfo {
        view: gen_application_view(app_id),
        info: PromotionInfo {
            todo_data: [0; 0x20]
        }
    }
}

pub const fn gen_application_record(app_id: ApplicationId) -> ApplicationRecord {
    ApplicationRecord {
        app_id,
        unk_type: 0x3,
        unk_2: 2,
        unk_3: [0; 0x6],
        unk_4: 0,
        unk_reserved: [0; 0x7]
    }
}

pub const fn convert_application_view_to_deprecated(view: ApplicationView) -> ApplicationViewDeprecated {
    ApplicationViewDeprecated {
        app_id: view.app_id,
        unk_1: view.unk_1,
        unk_flags: view.unk_flags,
        unk_2: view.unk_2,
        unk_3: view.unk_3,
        unk_4: view.unk_4,
        unk_reserved: [0; 0x2],
        unk_7: view.unk_7,
        unk_8: view.unk_8,
        unk_9: view.unk_9,
        unk_reserved_3: [0; 0x3]
    }
}

const TEST_APP_ID: ApplicationId = ApplicationId(0x0500ABBACDDCEFFE);

pub fn is_extra_application(app_id: ApplicationId) -> bool {
    // TODO
    app_id == TEST_APP_ID
}

pub fn get_extra_application_view(app_id: ApplicationId) -> Result<ApplicationView> {
    if is_extra_application(app_id) {
        Ok(gen_application_view(app_id))
    }
    else {
        Err(ResultCode::new(0xBEEF1))
    }
}

pub fn get_extra_application_view_with_promotion_info(app_id: ApplicationId) -> Result<ApplicationViewWithPromotionInfo> {
    if is_extra_application(app_id) {
        Ok(gen_application_view_with_promotion_info(app_id))
    }
    else {
        Err(ResultCode::new(0xBEEF1))
    }
}

pub fn get_extra_application_control_data(app_id: ApplicationId) -> Result<(Vec<u8>, Vec<u8>)> {
    if is_extra_application(app_id) {
        // TODO
        let icon_data = include_bytes!("nashe.jpg");
        let nacp_data = include_bytes!("nashe.nacp");

        if nacp_data.len() != core::mem::size_of::<ApplicationControlProperty>() {
            Err(ResultCode::new(0xBEEF5))
        }
        else {
            Ok((nacp_data.to_vec(), icon_data.to_vec()))
        }
    }
    else {
        Err(ResultCode::new(0xBEEF2))
    }
}

pub fn get_extra_application_records() -> Vec<ApplicationRecord> {
    vec![gen_application_record(TEST_APP_ID)]
}