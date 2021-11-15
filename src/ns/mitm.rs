use super::*;
use alloc::vec::Vec;
use nx::ipc::sf;
use nx::service;
use client::{AsyncResult, AsyncValue, AsyncValueAndProgress, ProgressAsyncResult, ProgressMonitorForDeleteUserSaveDataAll, ApplicationResource, RequestServerStopper, GameCardStopper};
use crate::hb;

pub struct ReadOnlyApplicationControlDataInterface {
    session: sf::Session
}

impl ReadOnlyApplicationControlDataInterface {
    pub fn new() -> Self {
        Self {
            session: sf::Session::new()
        }
    }
}

impl sf::IObject for ReadOnlyApplicationControlDataInterface {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec! [
            ipc_cmif_interface_make_command_meta!(get_application_control_data: 0),
            ipc_cmif_interface_make_command_meta!(get_application_desired_language: 1),
            ipc_cmif_interface_make_command_meta!(convert_application_language_to_language_code: 2),
            ipc_cmif_interface_make_command_meta!(convert_language_code_to_application_language: 3),
            ipc_cmif_interface_make_command_meta!(select_application_desired_language: 4)
        ]
    }
}

impl IReadOnlyApplicationControlDataInterface for ReadOnlyApplicationControlDataInterface {
    fn get_application_control_data(&mut self, source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IReadOnlyApplicationControlDataInterface -> get_application_control_data [source: {:?}, app_id: {:?}]\n", source, app_id);

        if hb::is_extra_application(app_id) {
            let (nacp_data, icon_data) = hb::get_extra_application_control_data(app_id)?;
            
            unsafe {
                core::ptr::copy(nacp_data.as_ptr(), out_buf.buf as *mut u8, nacp_data.len());
                core::ptr::copy(icon_data.as_ptr(), out_buf.buf.offset(core::mem::size_of::<ApplicationControlProperty>() as isize) as *mut u8, icon_data.len());
            }

            Ok((nacp_data.len() + icon_data.len()) as u32)
        }
        else {
            client::get_read_only_application_control_data_interface().get_application_control_data(source, app_id, out_buf)
        }
    }

    fn get_application_desired_language(&mut self, lang_bitmask: u8) -> Result<u8> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IReadOnlyApplicationControlDataInterface -> get_application_desired_language\n");
        client::get_read_only_application_control_data_interface().get_application_desired_language(lang_bitmask)
    }

    fn convert_application_language_to_language_code(&mut self, app_lang: u8) -> Result<CString<0x8>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IReadOnlyApplicationControlDataInterface -> convert_application_language_to_language_code\n");
        client::get_read_only_application_control_data_interface().convert_application_language_to_language_code(app_lang)
    }

    fn convert_language_code_to_application_language(&mut self, lang_code: CString<0x8>) -> Result<u8> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IReadOnlyApplicationControlDataInterface -> convert_language_code_to_application_language\n");
        client::get_read_only_application_control_data_interface().convert_language_code_to_application_language(lang_code)
    }

    fn select_application_desired_language(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IReadOnlyApplicationControlDataInterface -> select_application_desired_language\n");
        todo!("select_application_desired_language")
    }
}

pub struct ApplicationManagerInterface {
    session: sf::Session
}

impl ApplicationManagerInterface {
    pub fn new() -> Self {
        Self {
            session: sf::Session::new()
        }
    }
}

impl sf::IObject for ApplicationManagerInterface {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec! [
            ipc_cmif_interface_make_command_meta!(list_application_record: 0),
            ipc_cmif_interface_make_command_meta!(generate_application_record_count: 1),
            ipc_cmif_interface_make_command_meta!(get_application_record_update_system_event: 2),
            ipc_cmif_interface_make_command_meta!(get_application_view_deprecated: 3),
            ipc_cmif_interface_make_command_meta!(delete_application_entity: 4),
            ipc_cmif_interface_make_command_meta!(delete_application_completely: 5),
            ipc_cmif_interface_make_command_meta!(is_any_application_entity_redundant: 6),
            ipc_cmif_interface_make_command_meta!(delete_redundant_application_entity: 7),
            ipc_cmif_interface_make_command_meta!(is_application_entity_movable: 8),
            ipc_cmif_interface_make_command_meta!(move_application_entity: 9),
            ipc_cmif_interface_make_command_meta!(calculate_application_occupied_size: 11),
            ipc_cmif_interface_make_command_meta!(push_application_record: 16),
            ipc_cmif_interface_make_command_meta!(list_application_record_content_meta: 17),
            ipc_cmif_interface_make_command_meta!(launch_application_old: 19),
            ipc_cmif_interface_make_command_meta!(get_application_content_path: 21),
            ipc_cmif_interface_make_command_meta!(terminate_application: 22),
            ipc_cmif_interface_make_command_meta!(resolve_application_content_path: 23),
            ipc_cmif_interface_make_command_meta!(begin_install_application: 26),
            ipc_cmif_interface_make_command_meta!(delete_application_record: 27),
            ipc_cmif_interface_make_command_meta!(request_application_update_info: 30),
            ipc_cmif_interface_make_command_meta!(request_update_application: 31),
            ipc_cmif_interface_make_command_meta!(cancel_application_download: 32),
            ipc_cmif_interface_make_command_meta!(resume_application_download: 33),
            ipc_cmif_interface_make_command_meta!(update_version_list: 35),
            ipc_cmif_interface_make_command_meta!(push_launch_version: 36),
            ipc_cmif_interface_make_command_meta!(list_required_version: 37),
            ipc_cmif_interface_make_command_meta!(check_application_launch_version: 38),
            ipc_cmif_interface_make_command_meta!(check_application_launch_rights: 39),
            ipc_cmif_interface_make_command_meta!(get_application_logo_data: 40),
            ipc_cmif_interface_make_command_meta!(calculate_application_download_required_size: 41),
            ipc_cmif_interface_make_command_meta!(cleanup_sd_card: 42),
            ipc_cmif_interface_make_command_meta!(check_sd_card_mount_status: 43),
            ipc_cmif_interface_make_command_meta!(get_sd_card_mount_status_changed_event: 44),
            ipc_cmif_interface_make_command_meta!(get_game_card_attachment_event: 45),
            ipc_cmif_interface_make_command_meta!(get_game_card_attachment_info: 46),
            ipc_cmif_interface_make_command_meta!(get_total_space_size: 47),
            ipc_cmif_interface_make_command_meta!(get_free_space_size: 48),
            ipc_cmif_interface_make_command_meta!(get_sd_card_removed_event: 49),
            ipc_cmif_interface_make_command_meta!(get_game_card_update_detection_event: 52),
            ipc_cmif_interface_make_command_meta!(disable_application_auto_delete: 53),
            ipc_cmif_interface_make_command_meta!(enable_application_auto_delete: 54),
            ipc_cmif_interface_make_command_meta!(get_application_desired_language: 55),
            ipc_cmif_interface_make_command_meta!(set_application_terminate_result: 56),
            ipc_cmif_interface_make_command_meta!(clear_application_terminate_result: 57),
            ipc_cmif_interface_make_command_meta!(get_last_sd_card_mount_unexpected_result: 58),
            ipc_cmif_interface_make_command_meta!(convert_application_language_to_language_code: 59),
            ipc_cmif_interface_make_command_meta!(convert_language_code_to_application_language: 60),
            ipc_cmif_interface_make_command_meta!(get_background_download_stress_task_info: 61),
            ipc_cmif_interface_make_command_meta!(get_game_card_stopper: 62),
            ipc_cmif_interface_make_command_meta!(is_system_program_installed: 63),
            ipc_cmif_interface_make_command_meta!(start_apply_delta_task: 64),
            ipc_cmif_interface_make_command_meta!(get_request_server_stopper: 65),
            ipc_cmif_interface_make_command_meta!(get_background_apply_delta_stress_task_info: 66),
            ipc_cmif_interface_make_command_meta!(cancel_application_apply_delta: 67),
            ipc_cmif_interface_make_command_meta!(resume_application_apply_delta: 68),
            ipc_cmif_interface_make_command_meta!(calculate_application_apply_delta_required_size: 69),
            ipc_cmif_interface_make_command_meta!(resume_all: 70),
            ipc_cmif_interface_make_command_meta!(get_storage_size: 71),
            ipc_cmif_interface_make_command_meta!(request_download_application: 80),
            ipc_cmif_interface_make_command_meta!(request_download_add_on_content: 81),
            ipc_cmif_interface_make_command_meta!(download_application: 82),
            ipc_cmif_interface_make_command_meta!(check_application_resume_rights: 83),
            ipc_cmif_interface_make_command_meta!(get_dynamic_commit_event: 84),
            ipc_cmif_interface_make_command_meta!(request_update_application_2: 85),
            ipc_cmif_interface_make_command_meta!(enable_application_crash_report: 86),
            ipc_cmif_interface_make_command_meta!(is_application_crash_report_enabled: 87),
            ipc_cmif_interface_make_command_meta!(boost_system_memory_resource_limit: 90),
            ipc_cmif_interface_make_command_meta!(deprecated_launch_application: 91),
            ipc_cmif_interface_make_command_meta!(get_running_application_program_id: 92),
            ipc_cmif_interface_make_command_meta!(get_main_application_program_index: 93),
            ipc_cmif_interface_make_command_meta!(launch_application: 94),
            ipc_cmif_interface_make_command_meta!(get_application_launch_info: 95),
            ipc_cmif_interface_make_command_meta!(acquire_application_launch_info: 96),
            ipc_cmif_interface_make_command_meta!(get_main_application_program_index_by_application_launch_info: 97),
            ipc_cmif_interface_make_command_meta!(enable_application_all_thread_dump_on_crash: 98),
            ipc_cmif_interface_make_command_meta!(launch_dev_menu: 99),
            ipc_cmif_interface_make_command_meta!(reset_to_factory_settings: 100),
            ipc_cmif_interface_make_command_meta!(reset_to_factory_settings_without_user_save_data: 101),
            ipc_cmif_interface_make_command_meta!(reset_to_factory_settings_for_refurbishment: 102),
            ipc_cmif_interface_make_command_meta!(reset_to_factory_settings_with_platform_region: 103),
            ipc_cmif_interface_make_command_meta!(reset_to_factory_settings_with_platform_region_authentication: 104),
            ipc_cmif_interface_make_command_meta!(request_reset_to_factory_settings_securely: 105),
            ipc_cmif_interface_make_command_meta!(request_reset_to_factory_settings_with_platform_region_authentication_securely: 106),
            ipc_cmif_interface_make_command_meta!(calculate_user_save_data_statistics: 200),
            ipc_cmif_interface_make_command_meta!(delete_user_save_data_all: 201),
            ipc_cmif_interface_make_command_meta!(delete_user_system_save_data: 210),
            ipc_cmif_interface_make_command_meta!(delete_save_data: 211),
            ipc_cmif_interface_make_command_meta!(unregister_network_service_account: 220),
            ipc_cmif_interface_make_command_meta!(unregister_network_service_account_with_user_save_data_deletion: 221),
            ipc_cmif_interface_make_command_meta!(get_application_shell_event: 300),
            ipc_cmif_interface_make_command_meta!(pop_application_shell_event_info: 301),
            ipc_cmif_interface_make_command_meta!(launch_library_applet: 302),
            ipc_cmif_interface_make_command_meta!(terminate_library_applet: 303),
            ipc_cmif_interface_make_command_meta!(launch_system_applet: 304),
            ipc_cmif_interface_make_command_meta!(terminate_system_applet: 305),
            ipc_cmif_interface_make_command_meta!(launch_overlay_applet: 306),
            ipc_cmif_interface_make_command_meta!(terminate_overlay_applet: 307),
            ipc_cmif_interface_make_command_meta!(get_application_control_data: 400),
            ipc_cmif_interface_make_command_meta!(invalidate_all_application_control_cache: 401),
            ipc_cmif_interface_make_command_meta!(request_download_application_control_data: 402),
            ipc_cmif_interface_make_command_meta!(get_max_application_control_cache_count: 403),
            ipc_cmif_interface_make_command_meta!(invalidate_application_control_cache: 404),
            ipc_cmif_interface_make_command_meta!(list_application_control_cache_entry_info: 405),
            ipc_cmif_interface_make_command_meta!(get_application_control_property: 406),
            ipc_cmif_interface_make_command_meta!(list_application_title: 407),
            ipc_cmif_interface_make_command_meta!(list_application_icon: 408),
            ipc_cmif_interface_make_command_meta!(request_check_game_card_registration: 502),
            ipc_cmif_interface_make_command_meta!(request_game_card_registration_gold_point: 503),
            ipc_cmif_interface_make_command_meta!(request_register_game_card: 504),
            ipc_cmif_interface_make_command_meta!(get_game_card_mount_failure_event: 505),
            ipc_cmif_interface_make_command_meta!(is_game_card_inserted: 506),
            ipc_cmif_interface_make_command_meta!(ensure_game_card_access: 507),
            ipc_cmif_interface_make_command_meta!(get_last_game_card_mount_failure_result: 508),
            ipc_cmif_interface_make_command_meta!(list_application_id_on_game_card: 509),
            ipc_cmif_interface_make_command_meta!(get_game_card_platform_region: 510),
            ipc_cmif_interface_make_command_meta!(count_application_content_meta: 600),
            ipc_cmif_interface_make_command_meta!(list_application_content_meta_status: 601),
            ipc_cmif_interface_make_command_meta!(list_available_add_on_content: 602),
            ipc_cmif_interface_make_command_meta!(get_owned_application_content_meta_status: 603),
            ipc_cmif_interface_make_command_meta!(register_contents_external_key: 604),
            ipc_cmif_interface_make_command_meta!(list_application_content_meta_status_with_rights_check: 605),
            ipc_cmif_interface_make_command_meta!(get_content_meta_storage: 606),
            ipc_cmif_interface_make_command_meta!(list_available_add_on_content_new: 607),
            ipc_cmif_interface_make_command_meta!(list_availability_assured_add_on_content: 609),
            ipc_cmif_interface_make_command_meta!(push_download_task_list: 700),
            ipc_cmif_interface_make_command_meta!(clear_task_status_list: 701),
            ipc_cmif_interface_make_command_meta!(request_download_task_list: 702),
            ipc_cmif_interface_make_command_meta!(request_ensure_download_task: 703),
            ipc_cmif_interface_make_command_meta!(list_download_task_status: 704),
            ipc_cmif_interface_make_command_meta!(request_download_task_list_data: 705),
            ipc_cmif_interface_make_command_meta!(request_version_list: 800),
            ipc_cmif_interface_make_command_meta!(list_version_list: 801),
            ipc_cmif_interface_make_command_meta!(request_version_list_data: 802),
            ipc_cmif_interface_make_command_meta!(get_application_record: 900),
            ipc_cmif_interface_make_command_meta!(get_application_record_property: 901),
            ipc_cmif_interface_make_command_meta!(enable_application_auto_update: 902),
            ipc_cmif_interface_make_command_meta!(disable_application_auto_update: 903),
            ipc_cmif_interface_make_command_meta!(touch_application: 904),
            ipc_cmif_interface_make_command_meta!(request_application_update: 905),
            ipc_cmif_interface_make_command_meta!(is_application_update_requested: 906),
            ipc_cmif_interface_make_command_meta!(withdraw_application_update_request: 907),
            ipc_cmif_interface_make_command_meta!(list_application_record_installed_content_meta: 908),
            ipc_cmif_interface_make_command_meta!(withdraw_cleanup_add_on_contents_with_no_rights_recommendation: 909),
            ipc_cmif_interface_make_command_meta!(has_application_record: 910),
            ipc_cmif_interface_make_command_meta!(set_pre_installed_application: 911),
            ipc_cmif_interface_make_command_meta!(clear_pre_installed_application_flag: 912),
            ipc_cmif_interface_make_command_meta!(list_all_application_record: 913),
            ipc_cmif_interface_make_command_meta!(hide_application_record: 914),
            ipc_cmif_interface_make_command_meta!(show_application_record: 915),
            ipc_cmif_interface_make_command_meta!(is_application_auto_delete_disabled: 916),
            ipc_cmif_interface_make_command_meta!(request_verify_application_deprecated: 1000),
            ipc_cmif_interface_make_command_meta!(corrupt_application_for_debug: 1001),
            ipc_cmif_interface_make_command_meta!(request_verify_add_on_contents_rights: 1002),
            ipc_cmif_interface_make_command_meta!(request_verify_application: 1003),
            ipc_cmif_interface_make_command_meta!(corrupt_content_for_debug: 1004),
            ipc_cmif_interface_make_command_meta!(needs_update_vulnerability: 1200),
            ipc_cmif_interface_make_command_meta!(is_any_application_entity_installed: 1300),
            ipc_cmif_interface_make_command_meta!(delete_application_content_entities: 1301),
            ipc_cmif_interface_make_command_meta!(cleanup_unrecorded_application_entity: 1302),
            ipc_cmif_interface_make_command_meta!(cleanup_add_on_contents_with_no_rights: 1303),
            ipc_cmif_interface_make_command_meta!(delete_application_content_entity: 1304),
            ipc_cmif_interface_make_command_meta!(delete_application_completely_for_debug: 1308),
            ipc_cmif_interface_make_command_meta!(cleanup_unavailable_add_on_contents: 1309),
            ipc_cmif_interface_make_command_meta!(request_move_application_entity: 1310),
            ipc_cmif_interface_make_command_meta!(estimate_size_to_move: 1311),
            ipc_cmif_interface_make_command_meta!(has_movable_entity: 1312),
            ipc_cmif_interface_make_command_meta!(cleanup_orphan_contents: 1313),
            ipc_cmif_interface_make_command_meta!(check_precondition_satisfied_to_move: 1314),
            ipc_cmif_interface_make_command_meta!(prepare_shutdown: 1400),
            ipc_cmif_interface_make_command_meta!(format_sd_card: 1500),
            ipc_cmif_interface_make_command_meta!(needs_system_update_to_format_sd_card: 1501),
            ipc_cmif_interface_make_command_meta!(get_last_sd_card_format_unexpected_result: 1502),
            ipc_cmif_interface_make_command_meta!(insert_sd_card: 1504),
            ipc_cmif_interface_make_command_meta!(remove_sd_card: 1505),
            ipc_cmif_interface_make_command_meta!(get_sd_card_startup_status: 1506),
            ipc_cmif_interface_make_command_meta!(get_system_seed_for_pseudo_device_id: 1600),
            ipc_cmif_interface_make_command_meta!(reset_system_seed_for_pseudo_device_id: 1601),
            ipc_cmif_interface_make_command_meta!(list_application_downloading_content_meta: 1700),
            ipc_cmif_interface_make_command_meta!(get_application_view: 1701),
            ipc_cmif_interface_make_command_meta!(get_application_download_task_status: 1702),
            ipc_cmif_interface_make_command_meta!(get_application_view_download_error_context: 1703),
            ipc_cmif_interface_make_command_meta!(get_application_view_with_promotion_info: 1704),
            ipc_cmif_interface_make_command_meta!(is_patch_auto_deletable_application: 1705),
            ipc_cmif_interface_make_command_meta!(is_notification_setup_completed: 1800),
            ipc_cmif_interface_make_command_meta!(get_last_notification_info_count: 1801),
            ipc_cmif_interface_make_command_meta!(list_last_notification_info: 1802),
            ipc_cmif_interface_make_command_meta!(list_notification_task: 1803),
            ipc_cmif_interface_make_command_meta!(is_active_account: 1900),
            ipc_cmif_interface_make_command_meta!(request_download_application_prepurchased_rights: 1901),
            ipc_cmif_interface_make_command_meta!(get_application_ticket_info: 1902),
            ipc_cmif_interface_make_command_meta!(request_download_application_prepurchased_rights_for_account: 1903),
            ipc_cmif_interface_make_command_meta!(get_system_delivery_info: 2000),
            ipc_cmif_interface_make_command_meta!(select_latest_system_delivery_info: 2001),
            ipc_cmif_interface_make_command_meta!(verify_delivery_protocol_version: 2002),
            ipc_cmif_interface_make_command_meta!(get_application_delivery_info: 2003),
            ipc_cmif_interface_make_command_meta!(has_all_contents_to_deliver: 2004),
            ipc_cmif_interface_make_command_meta!(compare_application_delivery_info: 2005),
            ipc_cmif_interface_make_command_meta!(can_deliver_application: 2006),
            ipc_cmif_interface_make_command_meta!(list_content_meta_key_to_deliver_application: 2007),
            ipc_cmif_interface_make_command_meta!(needs_system_update_to_deliver_application: 2008),
            ipc_cmif_interface_make_command_meta!(estimate_required_size: 2009),
            ipc_cmif_interface_make_command_meta!(request_receive_application: 2010),
            ipc_cmif_interface_make_command_meta!(commit_receive_application: 2011),
            ipc_cmif_interface_make_command_meta!(get_receive_application_progress: 2012),
            ipc_cmif_interface_make_command_meta!(request_send_application: 2013),
            ipc_cmif_interface_make_command_meta!(get_send_application_progress: 2014),
            ipc_cmif_interface_make_command_meta!(compare_system_delivery_info: 2015),
            ipc_cmif_interface_make_command_meta!(list_not_committed_content_meta: 2016),
            ipc_cmif_interface_make_command_meta!(recover_download_task: 2017),
            ipc_cmif_interface_make_command_meta!(get_application_delivery_info_hash: 2018),
            ipc_cmif_interface_make_command_meta!(get_application_rights_on_client: 2050),
            ipc_cmif_interface_make_command_meta!(invalidate_rights_id_cache: 2051),
            ipc_cmif_interface_make_command_meta!(get_application_terminate_result: 2100),
            ipc_cmif_interface_make_command_meta!(get_raw_application_terminate_result: 2101),
            ipc_cmif_interface_make_command_meta!(create_rights_environment: 2150),
            ipc_cmif_interface_make_command_meta!(destroy_rights_environment: 2151),
            ipc_cmif_interface_make_command_meta!(activate_rights_environment: 2152),
            ipc_cmif_interface_make_command_meta!(deactivate_rights_environment: 2153),
            ipc_cmif_interface_make_command_meta!(force_activate_rights_context_for_exit: 2154),
            ipc_cmif_interface_make_command_meta!(update_rights_environment_status: 2155),
            ipc_cmif_interface_make_command_meta!(create_rights_environment_for_micro_application_preomia: 2156),
            ipc_cmif_interface_make_command_meta!(add_target_application_to_rights_environment: 2160),
            ipc_cmif_interface_make_command_meta!(set_users_to_rights_environment: 2161),
            ipc_cmif_interface_make_command_meta!(get_rights_environment_status: 2170),
            ipc_cmif_interface_make_command_meta!(get_rights_environment_status_changed_event: 2171),
            ipc_cmif_interface_make_command_meta!(request_extend_expiration_in_rights_environment: 2180),
            ipc_cmif_interface_make_command_meta!(get_result_of_extend_expiration_in_rights_environment: 2181),
            ipc_cmif_interface_make_command_meta!(set_active_rights_context_using_state_to_rights_environment: 2182),
            ipc_cmif_interface_make_command_meta!(get_rights_environment_handle_for_application: 2190),
            ipc_cmif_interface_make_command_meta!(get_rights_environment_count_for_debug: 2199),
            ipc_cmif_interface_make_command_meta!(get_game_card_application_copy_identifier: 2200),
            ipc_cmif_interface_make_command_meta!(get_installed_application_copy_identifier: 2201),
            ipc_cmif_interface_make_command_meta!(request_report_active_elicence: 2250),
            ipc_cmif_interface_make_command_meta!(list_event_log: 2300),
            ipc_cmif_interface_make_command_meta!(perform_auto_update_by_application_id: 2350),
            ipc_cmif_interface_make_command_meta!(request_no_download_rights_error_resolution: 2351),
            ipc_cmif_interface_make_command_meta!(request_resolve_no_download_rights_error: 2352),
            ipc_cmif_interface_make_command_meta!(get_application_download_task_info: 2353),
            ipc_cmif_interface_make_command_meta!(prioritize_application_background_task: 2354),
            ipc_cmif_interface_make_command_meta!(prefer_storage_efficient_update: 2355),
            ipc_cmif_interface_make_command_meta!(request_storage_efficient_update_preferible: 2356),
            ipc_cmif_interface_make_command_meta!(get_promotion_info: 2400),
            ipc_cmif_interface_make_command_meta!(count_promotion_info: 2401),
            ipc_cmif_interface_make_command_meta!(list_promotion_info: 2402),
            ipc_cmif_interface_make_command_meta!(import_promotion_json_for_debug: 2403),
            ipc_cmif_interface_make_command_meta!(clear_promotion_info_for_debug: 2404),
            ipc_cmif_interface_make_command_meta!(confirm_available_time: 2500),
            ipc_cmif_interface_make_command_meta!(create_application_resource: 2510),
            ipc_cmif_interface_make_command_meta!(get_application_resource: 2511),
            ipc_cmif_interface_make_command_meta!(launch_micro_application_preomia: 2513),
            ipc_cmif_interface_make_command_meta!(clear_task_of_async_task_manager: 2514),
            ipc_cmif_interface_make_command_meta!(cleanup_all_placeholder_and_fragments_if_no_task: 2515),
            ipc_cmif_interface_make_command_meta!(ensure_application_certificate: 2516),
            ipc_cmif_interface_make_command_meta!(create_application_instance: 2517),
            ipc_cmif_interface_make_command_meta!(update_qualification_for_debug: 2518),
            ipc_cmif_interface_make_command_meta!(is_qualification_transition_supported: 2519),
            ipc_cmif_interface_make_command_meta!(is_qualification_transition_supported_by_process_id: 2520),
            ipc_cmif_interface_make_command_meta!(get_rights_user_changed_event: 2521),
            ipc_cmif_interface_make_command_meta!(get_application_id_of_preomia: 2800),
            ipc_cmif_interface_make_command_meta!(register_device_lock_key: 3000),
            ipc_cmif_interface_make_command_meta!(unregister_device_lock_key: 3001),
            ipc_cmif_interface_make_command_meta!(verify_device_lock_key: 3002),
            ipc_cmif_interface_make_command_meta!(hide_application_icon: 3003),
            ipc_cmif_interface_make_command_meta!(show_application_icon: 3004),
            ipc_cmif_interface_make_command_meta!(hide_application_title: 3005),
            ipc_cmif_interface_make_command_meta!(show_application_title: 3006),
            ipc_cmif_interface_make_command_meta!(enable_game_card: 3007),
            ipc_cmif_interface_make_command_meta!(disable_game_card: 3008),
            ipc_cmif_interface_make_command_meta!(enable_local_content_share: 3009),
            ipc_cmif_interface_make_command_meta!(disable_local_content_share: 3010),
            ipc_cmif_interface_make_command_meta!(is_application_icon_hidden: 3011),
            ipc_cmif_interface_make_command_meta!(is_application_title_hidden: 3012),
            ipc_cmif_interface_make_command_meta!(is_game_card_enabled: 3013),
            ipc_cmif_interface_make_command_meta!(is_local_content_share_enabled: 3014),
            ipc_cmif_interface_make_command_meta!(get_application_certificate: 9999)
        ]
    }
}

impl IApplicationManagerInterface for ApplicationManagerInterface {
    fn list_application_record(&mut self, entry_offset: u32, out_record_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_record\n");
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> buf count: {}\n", out_record_buf.size / core::mem::size_of::<ApplicationRecord>());

        assert_eq!(entry_offset, 0);

        let orb = out_record_buf.clone();
        let out_record_buf_buf = out_record_buf.buf;
        let real_record_count = client::get_application_manager_interface().list_application_record(entry_offset, out_record_buf)?;
        let hb_records = hb::get_extra_application_records();
        let hb_record_count = hb_records.len();
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> Records -> real: {} + hb: {}\n", real_record_count, hb_record_count);

        let out_hb_record_buf = unsafe {
            sf::OutMapAliasBuffer::from_const(out_record_buf_buf.offset(real_record_count as isize * core::mem::size_of::<ApplicationRecord>() as isize), hb_record_count * core::mem::size_of::<ApplicationRecord>())
        };
        let out_hb_records = out_hb_record_buf.get_mut_slice::<ApplicationRecord>();
        for i in 0..hb_record_count {
            out_hb_records[i] = hb_records[i];
        }

        let orba = orb.get_slice::<ApplicationRecord>();
        for i in 0..(real_record_count as usize + hb_record_count) {
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> Record: {:?}\n", orba[i]);
        }

        Ok(real_record_count + hb_record_count as u32)
    }

    fn generate_application_record_count(&mut self) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> generate_application_record_count\n");
        
        let real_record_count = client::get_application_manager_interface().generate_application_record_count()?;
        let hb_record_count = hb::get_extra_application_records().len() as u64;
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> Records -> real: {} + hb: {}\n", real_record_count, hb_record_count);
        Ok(real_record_count + hb_record_count)
    }

    fn get_application_record_update_system_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_record_update_system_event\n");
        client::get_application_manager_interface().get_application_record_update_system_event()
    }

    fn get_application_view_deprecated(&mut self, in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_view_deprecated\n");

        let app_ids = in_app_ids.get_slice::<ApplicationId>();
        let mut real_app_ids = app_ids.to_vec();
        real_app_ids.retain(|app_id| !hb::is_extra_application(*app_id));

        let mut out_real_views: Vec<ApplicationViewDeprecated> = vec![unsafe { core::mem::zeroed() }; real_app_ids.len()];
        client::get_application_manager_interface().get_application_view_deprecated(sf::InMapAliasBuffer::from_array(&real_app_ids), sf::OutMapAliasBuffer::from_array(&out_real_views))?;

        let out_views_arr = out_views.get_mut_slice::<ApplicationViewDeprecated>();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for app_id in app_ids {
            if hb::is_extra_application(*app_id) {
                out_views_arr[i] = hb::gen_deprecated_application_view(*app_id);
            }
            else {
                out_views_arr[i] = out_real_views[j];
                j += 1;
            }
            i += 1;
        }

        Ok(())
    }

    fn delete_application_entity(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_entity\n");
        client::get_application_manager_interface().delete_application_entity(app_id)
    }

    fn delete_application_completely(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_completely\n");
        client::get_application_manager_interface().delete_application_completely(app_id)
    }

    fn is_any_application_entity_redundant(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_any_application_entity_redundant\n");
        client::get_application_manager_interface().is_any_application_entity_redundant()
    }

    fn delete_redundant_application_entity(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_redundant_application_entity\n");
        client::get_application_manager_interface().delete_redundant_application_entity()
    }

    fn is_application_entity_movable(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_redundant_application_entity\n");
        client::get_application_manager_interface().is_application_entity_movable(storage_id, app_id)
    }

    fn move_application_entity(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> move_application_entity\n");
        client::get_application_manager_interface().move_application_entity(storage_id, app_id)
    }

    fn calculate_application_occupied_size(&mut self, app_id: ApplicationId) -> Result<ApplicationOccupiedSize> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> calculate_application_occupied_size\n");
        client::get_application_manager_interface().calculate_application_occupied_size(app_id)
    }

    fn push_application_record(&mut self, last_modified_event: u8, app_id: ApplicationId, record_buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> push_application_record\n");
        client::get_application_manager_interface().push_application_record(last_modified_event, app_id, record_buf)
    }

    fn list_application_record_content_meta(&mut self, offset: u64, app_id: ApplicationId, out_meta_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_record_content_meta\n");
        client::get_application_manager_interface().list_application_record_content_meta(offset, app_id, out_meta_buf)
    }

    fn launch_application_old(&mut self, app_id: ApplicationId) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_application_old\n");
        client::get_application_manager_interface().launch_application_old(app_id)
    }

    fn get_application_content_path(&mut self, meta_type: ContentMetaType, app_id: ApplicationId, out_path: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_content_path\n");
        client::get_application_manager_interface().get_application_content_path(meta_type, app_id, out_path)
    }

    fn terminate_application(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> terminate_application\n");
        client::get_application_manager_interface().terminate_application(app_id)
    }

    fn resolve_application_content_path(&mut self, meta_type: ContentMetaType, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> resolve_application_content_path\n");
        client::get_application_manager_interface().resolve_application_content_path(meta_type, app_id)
    }

    fn begin_install_application(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> begin_install_application\n");
        client::get_application_manager_interface().begin_install_application(storage_id, app_id)
    }

    fn delete_application_record(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_record\n");
        client::get_application_manager_interface().delete_application_record(app_id)
    }

    fn request_application_update_info(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_application_update_info\n");
        client::get_application_manager_interface().request_application_update_info(app_id)
    }

    fn request_update_application(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_update_application\n");
        client::get_application_manager_interface().request_update_application(app_id)
    }

    fn cancel_application_download(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cancel_application_download\n");
        client::get_application_manager_interface().cancel_application_download(app_id)
    }

    fn resume_application_download(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> resume_application_download\n");
        client::get_application_manager_interface().resume_application_download(app_id)
    }

    fn update_version_list(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> update_version_list\n");
        client::get_application_manager_interface().update_version_list(buf)
    }

    fn push_launch_version(&mut self, version: u32, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> push_launch_version\n");
        client::get_application_manager_interface().push_launch_version(version, app_id)
    }

    fn list_required_version(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_required_version\n");
        client::get_application_manager_interface().list_required_version(out_buf)
    }

    fn check_application_launch_version(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> check_application_launch_version\n");
        client::get_application_manager_interface().check_application_launch_version(app_id)
    }

    fn check_application_launch_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> check_application_launch_rights\n");
        client::get_application_manager_interface().check_application_launch_rights(app_id)
    }

    fn get_application_logo_data(&mut self, app_id: ApplicationId, buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_logo_data\n");
        client::get_application_manager_interface().get_application_logo_data(app_id, buf, out_buf)
    }

    fn calculate_application_download_required_size(&mut self, app_id: ApplicationId) -> Result<(u64, u64)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> calculate_application_download_required_size\n");
        client::get_application_manager_interface().calculate_application_download_required_size(app_id)
    }

    fn cleanup_sd_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_sd_card\n");
        client::get_application_manager_interface().cleanup_sd_card()
    }

    fn check_sd_card_mount_status(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> check_sd_card_mount_status\n");
        client::get_application_manager_interface().check_sd_card_mount_status()
    }

    fn get_sd_card_mount_status_changed_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_sd_card_mount_status_changed_event\n");
        client::get_application_manager_interface().get_sd_card_mount_status_changed_event()
    }

    fn get_game_card_attachment_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_attachment_event\n");
        client::get_application_manager_interface().get_game_card_attachment_event()
    }

    fn get_game_card_attachment_info(&mut self) -> Result<(u64, u64)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_attachment_info\n");
        client::get_application_manager_interface().get_game_card_attachment_info()
    }

    fn get_total_space_size(&mut self, storage_id: StorageId) -> Result<usize> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_total_space_size\n");
        client::get_application_manager_interface().get_total_space_size(storage_id)
    }

    fn get_free_space_size(&mut self, storage_id: StorageId) -> Result<usize> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_free_space_size\n");
        client::get_application_manager_interface().get_free_space_size(storage_id)
    }

    fn get_sd_card_removed_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_sd_card_removed_event\n");
        client::get_application_manager_interface().get_sd_card_removed_event()
    }

    fn get_game_card_update_detection_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_update_detection_event\n");
        client::get_application_manager_interface().get_game_card_update_detection_event()
    }

    fn disable_application_auto_delete(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> disable_application_auto_delete\n");
        client::get_application_manager_interface().disable_application_auto_delete(app_id)
    }

    fn enable_application_auto_delete(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_application_auto_delete\n");
        client::get_application_manager_interface().enable_application_auto_delete(app_id)
    }

    fn get_application_desired_language(&mut self, lang_bitmask: u8) -> Result<u8> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_desired_language\n");
        client::get_application_manager_interface().get_application_desired_language(lang_bitmask)
    }

    fn set_application_terminate_result(&mut self, rc: ResultCode, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> set_application_terminate_result\n");
        client::get_application_manager_interface().set_application_terminate_result(rc, app_id)
    }

    fn clear_application_terminate_result(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> clear_application_terminate_result\n");
        client::get_application_manager_interface().clear_application_terminate_result(app_id)
    }

    fn get_last_sd_card_mount_unexpected_result(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_last_sd_card_mount_unexpected_result\n");
        client::get_application_manager_interface().get_last_sd_card_mount_unexpected_result()
    }

    fn convert_application_language_to_language_code(&mut self, app_lang: u8) -> Result<CString<0x8>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> convert_application_language_to_language_code\n");
        client::get_application_manager_interface().convert_application_language_to_language_code(app_lang)
    }

    fn convert_language_code_to_application_language(&mut self, lang_code: CString<0x8>) -> Result<u8> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> convert_language_code_to_application_language\n");
        client::get_application_manager_interface().convert_language_code_to_application_language(lang_code)
    }

    fn get_background_download_stress_task_info(&mut self) -> Result<(u64, u64)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_background_download_stress_task_info\n");
        client::get_application_manager_interface().get_background_download_stress_task_info()
    }

    fn get_game_card_stopper(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_stopper\n");
        client::get_application_manager_interface().get_game_card_stopper()
    }

    fn is_system_program_installed(&mut self, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_system_program_installed\n");
        client::get_application_manager_interface().is_system_program_installed(app_id)
    }

    fn start_apply_delta_task(&mut self, unk_app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> start_apply_delta_task\n");
        client::get_application_manager_interface().start_apply_delta_task(unk_app_id)
    }

    fn get_request_server_stopper(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_request_server_stopper\n");
        client::get_application_manager_interface().get_request_server_stopper()
    }

    fn get_background_apply_delta_stress_task_info(&mut self) -> Result<(u64, u64)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_background_apply_delta_stress_task_info\n");
        client::get_application_manager_interface().get_background_apply_delta_stress_task_info()
    }

    fn cancel_application_apply_delta(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cancel_application_apply_delta\n");
        client::get_application_manager_interface().cancel_application_apply_delta(app_id)
    }

    fn resume_application_apply_delta(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> resume_application_apply_delta\n");
        client::get_application_manager_interface().resume_application_apply_delta(app_id)
    }

    fn calculate_application_apply_delta_required_size(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<usize> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> calculate_application_apply_delta_required_size\n");
        client::get_application_manager_interface().calculate_application_apply_delta_required_size(storage_id, app_id)
    }

    fn resume_all(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> resume_all\n");
        client::get_application_manager_interface().resume_all()
    }

    fn get_storage_size(&mut self, storage_id: StorageId) -> Result<(usize, usize)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_storage_size\n");
        client::get_application_manager_interface().get_storage_size(storage_id)
    }

    fn request_download_application(&mut self, unk_storage_id: StorageId, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_application\n");
        client::get_application_manager_interface().request_download_application(unk_storage_id, app_id)
    }

    fn request_download_add_on_content(&mut self, unk_storage_id: StorageId, app_id: ApplicationId, buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_add_on_content\n");
        client::get_application_manager_interface().request_download_add_on_content(unk_storage_id, app_id, buf)
    }

    fn download_application(&mut self, unk_storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> download_application\n");
        client::get_application_manager_interface().download_application(unk_storage_id, app_id)
    }

    fn check_application_resume_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> check_application_resume_rights\n");
        client::get_application_manager_interface().check_application_resume_rights(app_id)
    }

    fn get_dynamic_commit_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_dynamic_commit_event\n");
        client::get_application_manager_interface().get_dynamic_commit_event()
    }

    fn request_update_application_2(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_update_application_2\n");
        client::get_application_manager_interface().request_update_application_2(app_id)
    }

    fn enable_application_crash_report(&mut self, unk_enable: bool) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_application_crash_report\n");
        client::get_application_manager_interface().enable_application_crash_report(unk_enable)
    }

    fn is_application_crash_report_enabled(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_application_crash_report_enabled\n");
        client::get_application_manager_interface().is_application_crash_report_enabled()
    }

    fn boost_system_memory_resource_limit(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> boost_system_memory_resource_limit\n");
        client::get_application_manager_interface().boost_system_memory_resource_limit(app_id)
    }

    fn deprecated_launch_application(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> deprecated_launch_application\n");
        client::get_application_manager_interface().deprecated_launch_application()
    }

    fn get_running_application_program_id(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_running_application_program_id\n");
        client::get_application_manager_interface().get_running_application_program_id()
    }

    fn get_main_application_program_index(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_main_application_program_index\n");
        client::get_application_manager_interface().get_main_application_program_index()
    }

    fn launch_application(&mut self, program_idx: u8, info: ApplicationLaunchInfo) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_application\n");
        client::get_application_manager_interface().launch_application(program_idx, info)
    }

    fn get_application_launch_info(&mut self, app_id: ApplicationId) -> Result<ApplicationLaunchInfo> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_launch_info\n");
        client::get_application_manager_interface().get_application_launch_info(app_id)
    }

    fn acquire_application_launch_info(&mut self, app_id: ApplicationId) -> Result<ApplicationLaunchInfo> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> acquire_application_launch_info\n");
        client::get_application_manager_interface().acquire_application_launch_info(app_id)
    }

    fn get_main_application_program_index_by_application_launch_info(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_main_application_program_index_by_application_launch_info\n");
        client::get_application_manager_interface().get_main_application_program_index_by_application_launch_info()
    }

    fn enable_application_all_thread_dump_on_crash(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_application_all_thread_dump_on_crash\n");
        client::get_application_manager_interface().enable_application_all_thread_dump_on_crash()
    }

    fn launch_dev_menu(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_dev_menu\n");
        client::get_application_manager_interface().launch_dev_menu()
    }

    fn reset_to_factory_settings(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_to_factory_settings\n");
        client::get_application_manager_interface().reset_to_factory_settings()
    }

    fn reset_to_factory_settings_without_user_save_data(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_to_factory_settings_without_user_save_data\n");
        client::get_application_manager_interface().reset_to_factory_settings_without_user_save_data()
    }

    fn reset_to_factory_settings_for_refurbishment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_to_factory_settings_for_refurbishment\n");
        client::get_application_manager_interface().reset_to_factory_settings_for_refurbishment()
    }

    fn reset_to_factory_settings_with_platform_region(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_to_factory_settings_with_platform_region\n");
        client::get_application_manager_interface().reset_to_factory_settings_with_platform_region()
    }

    fn reset_to_factory_settings_with_platform_region_authentication(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_to_factory_settings_with_platform_region_authentication\n");
        client::get_application_manager_interface().reset_to_factory_settings_with_platform_region_authentication()
    }

    fn request_reset_to_factory_settings_securely(&mut self, unk: u64, unk_2: sf::CopyHandle) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_reset_to_factory_settings_securely\n");
        client::get_application_manager_interface().request_reset_to_factory_settings_securely(unk, unk_2)
    }

    fn request_reset_to_factory_settings_with_platform_region_authentication_securely(&mut self, unk: u64, unk_2: u64, unk_3: sf::CopyHandle) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_reset_to_factory_settings_with_platform_region_authentication_securely\n");
        client::get_application_manager_interface().request_reset_to_factory_settings_with_platform_region_authentication_securely(unk, unk_2, unk_3)
    }

    fn calculate_user_save_data_statistics(&mut self, unk_1: u64, unk_2: u64) -> Result<(u64, u64)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> calculate_user_save_data_statistics\n");
        client::get_application_manager_interface().calculate_user_save_data_statistics(unk_1, unk_2)
    }

    fn delete_user_save_data_all(&mut self, uid: Uid) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_user_save_data_all\n");
        client::get_application_manager_interface().delete_user_save_data_all(uid)
    }

    fn delete_user_system_save_data(&mut self, uid: Uid, id: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_user_system_save_data\n");
        client::get_application_manager_interface().delete_user_system_save_data(uid, id)
    }

    fn delete_save_data(&mut self, space_id: SaveDataSpaceId, id: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_save_data\n");
        client::get_application_manager_interface().delete_save_data(space_id, id)
    }

    fn unregister_network_service_account(&mut self, uid: Uid) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> unregister_network_service_account\n");
        client::get_application_manager_interface().unregister_network_service_account(uid)
    }

    fn unregister_network_service_account_with_user_save_data_deletion(&mut self, space_id: SaveDataSpaceId, id: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> unregister_network_service_account_with_user_save_data_deletion\n");
        client::get_application_manager_interface().unregister_network_service_account_with_user_save_data_deletion(space_id, id)
    }

    fn get_application_shell_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_shell_event\n");
        client::get_application_manager_interface().get_application_shell_event()
    }

    fn pop_application_shell_event_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> pop_application_shell_event_info\n");
        client::get_application_manager_interface().pop_application_shell_event_info(out_buf)
    }

    fn launch_library_applet(&mut self, program_id: ProgramId) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_library_applet\n");
        client::get_application_manager_interface().launch_library_applet(program_id)
    }

    fn terminate_library_applet(&mut self, program_id: ProgramId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> terminate_library_applet\n");
        client::get_application_manager_interface().terminate_library_applet(program_id)
    }

    fn launch_system_applet(&mut self) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_system_applet\n");
        client::get_application_manager_interface().launch_system_applet()
    }

    fn terminate_system_applet(&mut self, program_id: ProgramId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> terminate_system_applet\n");
        client::get_application_manager_interface().terminate_system_applet(program_id)
    }

    fn launch_overlay_applet(&mut self) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_overlay_applet\n");
        client::get_application_manager_interface().launch_overlay_applet()
    }

    fn terminate_overlay_applet(&mut self, program_id: ProgramId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> terminate_overlay_applet\n");
        client::get_application_manager_interface().terminate_overlay_applet(program_id)
    }

    fn get_application_control_data(&mut self, source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_control_data [source: {:?}, app_id: {:?}]\n", source, app_id);

        if hb::is_extra_application(app_id) {
            let (nacp_data, icon_data) = hb::get_extra_application_control_data(app_id)?;
            
            unsafe {
                core::ptr::copy(nacp_data.as_ptr(), out_buf.buf as *mut u8, nacp_data.len());
                core::ptr::copy(icon_data.as_ptr(), out_buf.buf.offset(nacp_data.len() as isize) as *mut u8, icon_data.len());
            }

            Ok((nacp_data.len() + icon_data.len()) as u32)
        }
        else {
            client::get_application_manager_interface().get_application_control_data(source, app_id, out_buf)
        }
    }

    fn invalidate_all_application_control_cache(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> invalidate_all_application_control_cache\n");
        client::get_application_manager_interface().invalidate_all_application_control_cache()
    }

    fn request_download_application_control_data(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_application_control_data\n");
        client::get_application_manager_interface().request_download_application_control_data(app_id)
    }

    fn get_max_application_control_cache_count(&mut self) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_max_application_control_cache_count\n");
        client::get_application_manager_interface().get_max_application_control_cache_count()
    }

    fn invalidate_application_control_cache(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> invalidate_application_control_cache\n");
        client::get_application_manager_interface().invalidate_application_control_cache(app_id)
    }

    fn list_application_control_cache_entry_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_control_cache_entry_info\n");
        client::get_application_manager_interface().list_application_control_cache_entry_info(out_buf)
    }

    fn get_application_control_property(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_control_property\n");
        client::get_application_manager_interface().get_application_control_property()
    }

    fn list_application_title(&mut self, app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_title\n");
        client::get_application_manager_interface().list_application_title(app_id_buf, source, tmem_handle, tmem_size)
    }

    fn list_application_icon(&mut self, app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_icon\n");
        client::get_application_manager_interface().list_application_icon(app_id_buf, source, tmem_handle, tmem_size)
    }

    fn request_check_game_card_registration(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_check_game_card_registration\n");
        client::get_application_manager_interface().request_check_game_card_registration(app_id)
    }

    fn request_game_card_registration_gold_point(&mut self, uid: Uid, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_game_card_registration_gold_point\n");
        client::get_application_manager_interface().request_game_card_registration_gold_point(uid, app_id)
    }

    fn request_register_game_card(&mut self, unk: u32, uid: Uid, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_register_game_card\n");
        client::get_application_manager_interface().request_register_game_card(unk, uid, app_id)
    }

    fn get_game_card_mount_failure_event(&mut self) -> Result<sf::CopyHandle> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_mount_failure_event\n");
        client::get_application_manager_interface().get_game_card_mount_failure_event()
    }

    fn is_game_card_inserted(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_game_card_inserted\n");
        client::get_application_manager_interface().is_game_card_inserted()
    }

    fn ensure_game_card_access(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> ensure_game_card_access\n");
        client::get_application_manager_interface().ensure_game_card_access()
    }

    fn get_last_game_card_mount_failure_result(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_last_game_card_mount_failure_result\n");
        client::get_application_manager_interface().get_last_game_card_mount_failure_result()
    }

    fn list_application_id_on_game_card(&mut self, out_app_id_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_id_on_game_card\n");
        client::get_application_manager_interface().list_application_id_on_game_card(out_app_id_buf)
    }

    fn get_game_card_platform_region(&mut self) -> Result<GameCardCompatibilityType> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_platform_region\n");
        client::get_application_manager_interface().get_game_card_platform_region()
    }

    fn count_application_content_meta(&mut self, app_id: ApplicationId) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> count_application_content_meta\n");
        client::get_application_manager_interface().count_application_content_meta(app_id)
    }

    fn list_application_content_meta_status(&mut self, index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_content_meta_status\n");
        client::get_application_manager_interface().list_application_content_meta_status(index, app_id, out_buf)
    }

    fn list_available_add_on_content(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_available_add_on_content\n");
        client::get_application_manager_interface().list_available_add_on_content(unk_1, unk_2, out_buf)
    }

    fn get_owned_application_content_meta_status(&mut self, unk_1: u64, unk_2: u64) -> Result<ApplicationContentMetaStatus> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_owned_application_content_meta_status\n");
        client::get_application_manager_interface().get_owned_application_content_meta_status(unk_1, unk_2)
    }

    fn register_contents_external_key(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> register_contents_external_key\n");
        client::get_application_manager_interface().register_contents_external_key(unk_1, unk_2)
    }

    fn list_application_content_meta_status_with_rights_check(&mut self, index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_content_meta_status_with_rights_check\n");
        client::get_application_manager_interface().list_application_content_meta_status_with_rights_check(index, app_id, out_buf)
    }

    fn get_content_meta_storage(&mut self, unk_1: u64, unk_2: u64) -> Result<StorageId> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_content_meta_storage\n");
        client::get_application_manager_interface().get_content_meta_storage(unk_1, unk_2)
    }

    fn list_available_add_on_content_new(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_available_add_on_content_new\n");
        client::get_application_manager_interface().list_available_add_on_content_new(unk_1, unk_2, out_buf)
    }

    fn list_availability_assured_add_on_content(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_availability_assured_add_on_content\n");
        client::get_application_manager_interface().list_availability_assured_add_on_content()
    }

    fn push_download_task_list(&mut self, in_buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> push_download_task_list\n");
        client::get_application_manager_interface().push_download_task_list(in_buf)
    }

    fn clear_task_status_list(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> clear_task_status_list\n");
        client::get_application_manager_interface().clear_task_status_list()
    }

    fn request_download_task_list(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_task_list\n");
        client::get_application_manager_interface().request_download_task_list()
    }

    fn request_ensure_download_task(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_ensure_download_task\n");
        client::get_application_manager_interface().request_ensure_download_task()
    }

    fn list_download_task_status(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_download_task_status\n");
        client::get_application_manager_interface().list_download_task_status(out_buf)
    }

    fn request_download_task_list_data(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_task_list_data\n");
        client::get_application_manager_interface().request_download_task_list_data()
    }

    fn request_version_list(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_version_list\n");
        client::get_application_manager_interface().request_version_list()
    }

    fn list_version_list(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_version_list\n");
        client::get_application_manager_interface().list_version_list(out_buf)
    }

    fn request_version_list_data(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_version_list_data\n");
        client::get_application_manager_interface().request_version_list_data()
    }

    fn get_application_record(&mut self, app_id: ApplicationId) -> Result<ApplicationRecord> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_record\n");
        client::get_application_manager_interface().get_application_record(app_id)
    }

    fn get_application_record_property(&mut self, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_record_property\n");
        client::get_application_manager_interface().get_application_record_property(app_id, out_buf)
    }

    fn enable_application_auto_update(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_application_auto_update\n");
        client::get_application_manager_interface().enable_application_auto_update(app_id)
    }

    fn disable_application_auto_update(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> disable_application_auto_update\n");
        client::get_application_manager_interface().disable_application_auto_update(app_id)
    }

    fn touch_application(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> touch_application\n");
        client::get_application_manager_interface().touch_application(app_id)
    }

    fn request_application_update(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_application_update\n");
        client::get_application_manager_interface().request_application_update(unk_1, unk_2)
    }

    fn is_application_update_requested(&mut self, app_id: ApplicationId) -> Result<(bool, u32)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_application_update_requested\n");
        client::get_application_manager_interface().is_application_update_requested(app_id)
    }

    fn withdraw_application_update_request(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> withdraw_application_update_request\n");
        client::get_application_manager_interface().withdraw_application_update_request(app_id)
    }

    fn list_application_record_installed_content_meta(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_record_installed_content_meta\n");
        client::get_application_manager_interface().list_application_record_installed_content_meta(unk_1, unk_2, out_buf)
    }

    fn withdraw_cleanup_add_on_contents_with_no_rights_recommendation(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> withdraw_cleanup_add_on_contents_with_no_rights_recommendation\n");
        client::get_application_manager_interface().withdraw_cleanup_add_on_contents_with_no_rights_recommendation(app_id)
    }

    fn has_application_record(&mut self, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> has_application_record\n");
        client::get_application_manager_interface().has_application_record(app_id)
    }

    fn set_pre_installed_application(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> set_pre_installed_application\n");
        client::get_application_manager_interface().set_pre_installed_application()
    }

    fn clear_pre_installed_application_flag(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> clear_pre_installed_application_flag\n");
        client::get_application_manager_interface().clear_pre_installed_application_flag()
    }

    fn list_all_application_record(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_all_application_record\n");
        client::get_application_manager_interface().list_all_application_record()
    }

    fn hide_application_record(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> hide_application_record\n");
        client::get_application_manager_interface().hide_application_record()
    }

    fn show_application_record(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> show_application_record\n");
        client::get_application_manager_interface().show_application_record()
    }

    fn is_application_auto_delete_disabled(&mut self, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_application_auto_delete_disabled\n");
        client::get_application_manager_interface().is_application_auto_delete_disabled(app_id)
    }

    fn request_verify_application_deprecated(&mut self, app_id: ApplicationId, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_verify_application_deprecated\n");
        client::get_application_manager_interface().request_verify_application_deprecated(app_id, tmem_handle, tmem_size)
    }

    fn corrupt_application_for_debug(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> corrupt_application_for_debug\n");
        client::get_application_manager_interface().corrupt_application_for_debug(unk_1, unk_2)
    }

    fn request_verify_add_on_contents_rights(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_verify_add_on_contents_rights\n");
        client::get_application_manager_interface().request_verify_add_on_contents_rights(app_id)
    }

    fn request_verify_application(&mut self, unk: u32, app_id: ApplicationId, tmem: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_verify_application\n");
        client::get_application_manager_interface().request_verify_application(unk, app_id, tmem, tmem_size)
    }

    fn corrupt_content_for_debug(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> corrupt_content_for_debug\n");
        client::get_application_manager_interface().corrupt_content_for_debug()
    }

    fn needs_update_vulnerability(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> needs_update_vulnerability\n");
        client::get_application_manager_interface().needs_update_vulnerability()
    }

    fn is_any_application_entity_installed(&mut self, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_any_application_entity_installed\n");
        client::get_application_manager_interface().is_any_application_entity_installed(app_id)
    }

    fn delete_application_content_entities(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_content_entities\n");
        client::get_application_manager_interface().delete_application_content_entities(unk_1, unk_2)
    }

    fn cleanup_unrecorded_application_entity(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_unrecorded_application_entity\n");
        client::get_application_manager_interface().cleanup_unrecorded_application_entity(app_id)
    }

    fn cleanup_add_on_contents_with_no_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_add_on_contents_with_no_rights\n");
        client::get_application_manager_interface().cleanup_add_on_contents_with_no_rights(app_id)
    }

    fn delete_application_content_entity(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_content_entity\n");
        client::get_application_manager_interface().delete_application_content_entity(unk_1, unk_2)
    }

    fn delete_application_completely_for_debug(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> delete_application_completely_for_debug\n");
        client::get_application_manager_interface().delete_application_completely_for_debug()
    }

    fn cleanup_unavailable_add_on_contents(&mut self, app_id: ApplicationId, uid: Uid) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_unavailable_add_on_contents\n");
        client::get_application_manager_interface().cleanup_unavailable_add_on_contents(app_id, uid)
    }

    fn request_move_application_entity(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_move_application_entity\n");
        client::get_application_manager_interface().request_move_application_entity()
    }

    fn estimate_size_to_move(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> estimate_size_to_move\n");
        client::get_application_manager_interface().estimate_size_to_move()
    }

    fn has_movable_entity(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> has_movable_entity\n");
        client::get_application_manager_interface().has_movable_entity()
    }

    fn cleanup_orphan_contents(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_orphan_contents\n");
        client::get_application_manager_interface().cleanup_orphan_contents()
    }

    fn check_precondition_satisfied_to_move(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> check_precondition_satisfied_to_move\n");
        client::get_application_manager_interface().check_precondition_satisfied_to_move()
    }

    fn prepare_shutdown(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> prepare_shutdown\n");
        client::get_application_manager_interface().prepare_shutdown()
    }

    fn format_sd_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> format_sd_card\n");
        client::get_application_manager_interface().format_sd_card()
    }

    fn needs_system_update_to_format_sd_card(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> needs_system_update_to_format_sd_card\n");
        client::get_application_manager_interface().needs_system_update_to_format_sd_card()
    }

    fn get_last_sd_card_format_unexpected_result(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_last_sd_card_format_unexpected_result\n");
        client::get_application_manager_interface().get_last_sd_card_format_unexpected_result()
    }

    fn insert_sd_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> insert_sd_card\n");
        client::get_application_manager_interface().insert_sd_card()
    }

    fn remove_sd_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> remove_sd_card\n");
        client::get_application_manager_interface().remove_sd_card()
    }

    fn get_sd_card_startup_status(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_sd_card_startup_status\n");
        client::get_application_manager_interface().get_sd_card_startup_status()
    }

    fn get_system_seed_for_pseudo_device_id(&mut self) -> Result<[u8; 0x20]> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_system_seed_for_pseudo_device_id\n");
        client::get_application_manager_interface().get_system_seed_for_pseudo_device_id()
    }

    fn reset_system_seed_for_pseudo_device_id(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> reset_system_seed_for_pseudo_device_id\n");
        client::get_application_manager_interface().reset_system_seed_for_pseudo_device_id()
    }

    fn list_application_downloading_content_meta(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_application_downloading_content_meta\n");
        client::get_application_manager_interface().list_application_downloading_content_meta(unk_1, unk_2, out_buf)
    }

    fn get_application_view(&mut self, in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_view\n");

        let app_ids = in_app_ids.get_slice::<ApplicationId>();
        let mut real_app_ids = app_ids.to_vec();
        real_app_ids.retain(|app_id| !hb::is_extra_application(*app_id));

        let mut out_real_views: Vec<ApplicationView> = vec![unsafe { core::mem::zeroed() }; real_app_ids.len()];
        client::get_application_manager_interface().get_application_view(sf::InMapAliasBuffer::from_array(&real_app_ids), sf::OutMapAliasBuffer::from_array(&out_real_views))?;

        let out_views_arr = out_views.get_mut_slice::<ApplicationView>();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for app_id in app_ids {
            if hb::is_extra_application(*app_id) {
                diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> Gen view of hb application {:?}\n", *app_id);
                out_views_arr[i] = hb::gen_application_view(*app_id);
            }
            else {
                diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> View of real application: {:?}\n", out_real_views[j]);
                out_views_arr[i] = out_real_views[j];
                j += 1;
            }
            i += 1;
        }

        Ok(())
    }

    fn get_application_download_task_status(&mut self, app_id: ApplicationId) -> Result<u8> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_download_task_status\n");
        client::get_application_manager_interface().get_application_download_task_status(app_id)
    }

    fn get_application_view_download_error_context(&mut self, app_id: ApplicationId, out_err_ctx_buf: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_view_download_error_context\n");
        client::get_application_manager_interface().get_application_view_download_error_context(app_id, out_err_ctx_buf)
    }

    fn get_application_view_with_promotion_info(&mut self, in_app_ids: sf::InMapAliasBuffer, out_data: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_view_with_promotion_info\n");

        let app_ids = in_app_ids.get_slice::<ApplicationId>();
        let mut real_app_ids = app_ids.to_vec();
        real_app_ids.retain(|app_id| !hb::is_extra_application(*app_id));

        let mut out_real_views: Vec<ApplicationViewWithPromotionInfo> = vec![unsafe { core::mem::zeroed() }; real_app_ids.len()];
        client::get_application_manager_interface().get_application_view_with_promotion_info(sf::InMapAliasBuffer::from_array(&real_app_ids), sf::OutMapAliasBuffer::from_array(&out_real_views))?;

        let out_views_arr = out_data.get_mut_slice::<ApplicationViewWithPromotionInfo>();
        let mut i: usize = 0;
        let mut j: usize = 0;
        for app_id in app_ids {
            if hb::is_extra_application(*app_id) {
                out_views_arr[i] = hb::gen_application_view_with_promotion_info(*app_id);
            }
            else {
                out_views_arr[i] = out_real_views[j];
                j += 1;
            }
            i += 1;
        }

        Ok(())
    }

    fn is_patch_auto_deletable_application(&mut self, app_id: ApplicationId) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_patch_auto_deletable_application\n");
        client::get_application_manager_interface().is_patch_auto_deletable_application(app_id)
    }

    fn is_notification_setup_completed(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_notification_setup_completed\n");
        client::get_application_manager_interface().is_notification_setup_completed()
    }

    fn get_last_notification_info_count(&mut self) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_last_notification_info_count\n");
        client::get_application_manager_interface().get_last_notification_info_count()
    }

    fn list_last_notification_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_last_notification_info\n");
        client::get_application_manager_interface().list_last_notification_info(out_buf)
    }

    fn list_notification_task(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_notification_task\n");
        client::get_application_manager_interface().list_notification_task(out_buf)
    }

    fn is_active_account(&mut self, unk: u32) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_active_account\n");
        client::get_application_manager_interface().is_active_account(unk)
    }

    fn request_download_application_prepurchased_rights(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_application_prepurchased_rights\n");
        client::get_application_manager_interface().request_download_application_prepurchased_rights(app_id)
    }

    fn get_application_ticket_info(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_ticket_info\n");
        client::get_application_manager_interface().get_application_ticket_info()
    }

    fn request_download_application_prepurchased_rights_for_account(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_download_application_prepurchased_rights_for_account\n");
        client::get_application_manager_interface().request_download_application_prepurchased_rights_for_account()
    }

    fn get_system_delivery_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_system_delivery_info\n");
        client::get_application_manager_interface().get_system_delivery_info(out_buf)
    }

    fn select_latest_system_delivery_info(&mut self, system_info_buf: sf::InMapAliasBuffer, system_infos_buf: sf::InMapAliasBuffer, app_infos_buf: sf::InMapAliasBuffer) -> Result<i32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> select_latest_system_delivery_info\n");
        client::get_application_manager_interface().select_latest_system_delivery_info(system_info_buf, system_infos_buf, app_infos_buf)
    }

    fn verify_delivery_protocol_version(&mut self, system_info_buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> verify_delivery_protocol_version\n");
        client::get_application_manager_interface().verify_delivery_protocol_version(system_info_buf)
    }

    fn get_application_delivery_info(&mut self, bitmask: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_delivery_info\n");
        client::get_application_manager_interface().get_application_delivery_info(bitmask, app_id, out_buf)
    }

    fn has_all_contents_to_deliver(&mut self, array_buf: sf::InMapAliasBuffer) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> has_all_contents_to_deliver\n");
        client::get_application_manager_interface().has_all_contents_to_deliver(array_buf)
    }

    fn compare_application_delivery_info(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<i32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> compare_application_delivery_info\n");
        client::get_application_manager_interface().compare_application_delivery_info(buf_1, buf_2)
    }

    fn can_deliver_application(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> can_deliver_application\n");
        client::get_application_manager_interface().can_deliver_application(buf_1, buf_2)
    }

    fn list_content_meta_key_to_deliver_application(&mut self, unk: i32, in_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_content_meta_key_to_deliver_application\n");
        client::get_application_manager_interface().list_content_meta_key_to_deliver_application(unk, in_buf, out_buf)
    }

    fn needs_system_update_to_deliver_application(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> needs_system_update_to_deliver_application\n");
        client::get_application_manager_interface().needs_system_update_to_deliver_application(buf_1, buf_2)
    }

    fn estimate_required_size(&mut self, meta_key_buf: sf::InMapAliasBuffer) -> Result<usize> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> estimate_required_size\n");
        client::get_application_manager_interface().estimate_required_size(meta_key_buf)
    }

    fn request_receive_application(&mut self, storage_id: StorageId, port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_receive_application\n");
        client::get_application_manager_interface().request_receive_application(storage_id, port, ipv4_addr, app_id, meta_keys_buf)
    }

    fn commit_receive_application(&mut self, app_id: ApplicationId) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> commit_receive_application\n");
        client::get_application_manager_interface().commit_receive_application(app_id)
    }

    fn get_receive_application_progress(&mut self, app_id: ApplicationId) -> Result<ReceiveApplicationProgress> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_receive_application_progress\n");
        client::get_application_manager_interface().get_receive_application_progress(app_id)
    }

    fn request_send_application(&mut self, port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_send_application\n");
        client::get_application_manager_interface().request_send_application(port, ipv4_addr, app_id, meta_keys_buf)
    }

    fn get_send_application_progress(&mut self, app_id: ApplicationId) -> Result<SendApplicationProgress> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_send_application_progress\n");
        client::get_application_manager_interface().get_send_application_progress(app_id)
    }

    fn compare_system_delivery_info(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<i32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> compare_system_delivery_info\n");
        client::get_application_manager_interface().compare_system_delivery_info(buf_1, buf_2)
    }

    fn list_not_committed_content_meta(&mut self, unk: i32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_not_committed_content_meta\n");
        client::get_application_manager_interface().list_not_committed_content_meta(unk, app_id, out_buf)
    }

    fn recover_download_task(&mut self, unk: u64, array: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> recover_download_task\n");
        client::get_application_manager_interface().recover_download_task(unk, array)
    }

    fn get_application_delivery_info_hash(&mut self, array: sf::InMapAliasBuffer) -> Result<[u8; 0x20]> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_delivery_info_hash\n");
        client::get_application_manager_interface().get_application_delivery_info_hash(array)
    }

    fn get_application_rights_on_client(&mut self, flags: u32, app_id: ApplicationId, uid: Uid, mut out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_rights_on_client [flags: {}, app_id: {:?}, uid: {:?}]\n", flags, app_id, uid);

        if hb::is_extra_application(app_id) {
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> Generating fake ApplicationRightsOnClient...\n");
            let roc = ApplicationRightsOnClient {
                app_id: app_id,
                uid: uid,
                flags_1: 1,
                flags_2: 0,
                unk: [0, 0, 1, 0, 0, 0]
            };
            out_buf.set_as(roc);
            Ok(1)
        }
        else {
            let out_buf_c = out_buf.clone();
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> getting real ApplicationRightsOnClient...\n");
            let count = client::get_application_manager_interface().get_application_rights_on_client(flags, app_id, uid, out_buf)?;
            if count > 0 {
                let roc_arr = out_buf_c.get_slice::<ApplicationRightsOnClient>();
                for i in 0..count as usize {
                    diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> got real ApplicationRightsOnClient: {:?}\n", roc_arr[i]);
                }
            }
            Ok(count)
        }
    }

    fn invalidate_rights_id_cache(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> invalidate_rights_id_cache\n");
        client::get_application_manager_interface().invalidate_rights_id_cache()
    }

    fn get_application_terminate_result(&mut self, app_id: ApplicationId) -> Result<ResultCode> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_terminate_result\n");
        client::get_application_manager_interface().get_application_terminate_result(app_id)
    }

    fn get_raw_application_terminate_result(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_raw_application_terminate_result\n");
        client::get_application_manager_interface().get_raw_application_terminate_result()
    }

    fn create_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> create_rights_environment\n");
        client::get_application_manager_interface().create_rights_environment()
    }

    fn destroy_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> destroy_rights_environment\n");
        client::get_application_manager_interface().destroy_rights_environment()
    }

    fn activate_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> activate_rights_environment\n");
        client::get_application_manager_interface().activate_rights_environment()
    }

    fn deactivate_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> deactivate_rights_environment\n");
        client::get_application_manager_interface().deactivate_rights_environment()
    }

    fn force_activate_rights_context_for_exit(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> force_activate_rights_context_for_exit\n");
        client::get_application_manager_interface().force_activate_rights_context_for_exit()
    }

    fn update_rights_environment_status(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> update_rights_environment_status\n");
        client::get_application_manager_interface().update_rights_environment_status()
    }

    fn create_rights_environment_for_micro_application_preomia(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> create_rights_environment_for_micro_application_preomia\n");
        client::get_application_manager_interface().create_rights_environment_for_micro_application_preomia()
    }

    fn add_target_application_to_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> add_target_application_to_rights_environment\n");
        client::get_application_manager_interface().add_target_application_to_rights_environment()
    }

    fn set_users_to_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> set_users_to_rights_environment\n");
        client::get_application_manager_interface().set_users_to_rights_environment()
    }

    fn get_rights_environment_status(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_rights_environment_status\n");
        client::get_application_manager_interface().get_rights_environment_status()
    }

    fn get_rights_environment_status_changed_event(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_rights_environment_status_changed_event\n");
        client::get_application_manager_interface().get_rights_environment_status_changed_event()
    }

    fn request_extend_expiration_in_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_extend_expiration_in_rights_environment\n");
        client::get_application_manager_interface().request_extend_expiration_in_rights_environment()
    }

    fn get_result_of_extend_expiration_in_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_result_of_extend_expiration_in_rights_environment\n");
        client::get_application_manager_interface().get_result_of_extend_expiration_in_rights_environment()
    }

    fn set_active_rights_context_using_state_to_rights_environment(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> set_active_rights_context_using_state_to_rights_environment\n");
        client::get_application_manager_interface().set_active_rights_context_using_state_to_rights_environment()
    }

    fn get_rights_environment_handle_for_application(&mut self, unk: u64) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_rights_environment_handle_for_application\n");
        client::get_application_manager_interface().get_rights_environment_handle_for_application(unk)
    }

    fn get_rights_environment_count_for_debug(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_rights_environment_count_for_debug\n");
        client::get_application_manager_interface().get_rights_environment_count_for_debug()
    }

    fn get_game_card_application_copy_identifier(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_game_card_application_copy_identifier\n");
        client::get_application_manager_interface().get_game_card_application_copy_identifier()
    }

    fn get_installed_application_copy_identifier(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_installed_application_copy_identifier\n");
        client::get_application_manager_interface().get_installed_application_copy_identifier()
    }

    fn request_report_active_elicence(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_report_active_elicence\n");
        client::get_application_manager_interface().request_report_active_elicence()
    }

    fn list_event_log(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_event_log\n");
        client::get_application_manager_interface().list_event_log()
    }

    fn perform_auto_update_by_application_id(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> perform_auto_update_by_application_id\n");
        client::get_application_manager_interface().perform_auto_update_by_application_id()
    }

    fn request_no_download_rights_error_resolution(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_no_download_rights_error_resolution\n");
        client::get_application_manager_interface().request_no_download_rights_error_resolution(app_id)
    }

    fn request_resolve_no_download_rights_error(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_resolve_no_download_rights_error\n");
        client::get_application_manager_interface().request_resolve_no_download_rights_error(app_id)
    }

    fn get_application_download_task_info(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_download_task_info\n");
        client::get_application_manager_interface().get_application_download_task_info()
    }

    fn prioritize_application_background_task(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> prioritize_application_background_task\n");
        client::get_application_manager_interface().prioritize_application_background_task()
    }

    fn prefer_storage_efficient_update(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> prefer_storage_efficient_update\n");
        client::get_application_manager_interface().prefer_storage_efficient_update()
    }

    fn request_storage_efficient_update_preferible(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> request_storage_efficient_update_preferible\n");
        client::get_application_manager_interface().request_storage_efficient_update_preferible()
    }

    fn get_promotion_info(&mut self, app_id_buf: sf::InMapAliasBuffer, uid_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_promotion_info\n");
        client::get_application_manager_interface().get_promotion_info(app_id_buf, uid_buf, out_buf)
    }

    fn count_promotion_info(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> count_promotion_info\n");
        client::get_application_manager_interface().count_promotion_info()
    }

    fn list_promotion_info(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> list_promotion_info\n");
        client::get_application_manager_interface().list_promotion_info()
    }

    fn import_promotion_json_for_debug(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> import_promotion_json_for_debug\n");
        client::get_application_manager_interface().import_promotion_json_for_debug(buf)
    }

    fn clear_promotion_info_for_debug(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> clear_promotion_info_for_debug\n");
        client::get_application_manager_interface().clear_promotion_info_for_debug()
    }

    fn confirm_available_time(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> confirm_available_time\n");
        client::get_application_manager_interface().confirm_available_time()
    }

    fn create_application_resource(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> create_application_resource\n");
        client::get_application_manager_interface().create_application_resource()
    }

    fn get_application_resource(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_resource\n");
        client::get_application_manager_interface().get_application_resource()
    }

    fn launch_micro_application_preomia(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> launch_micro_application_preomia\n");
        client::get_application_manager_interface().launch_micro_application_preomia()
    }

    fn clear_task_of_async_task_manager(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> clear_task_of_async_task_manager\n");
        client::get_application_manager_interface().clear_task_of_async_task_manager()
    }

    fn cleanup_all_placeholder_and_fragments_if_no_task(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> cleanup_all_placeholder_and_fragments_if_no_task\n");
        client::get_application_manager_interface().cleanup_all_placeholder_and_fragments_if_no_task()
    }

    fn ensure_application_certificate(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> ensure_application_certificate\n");
        client::get_application_manager_interface().ensure_application_certificate()
    }

    fn create_application_instance(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> create_application_instance\n");
        client::get_application_manager_interface().create_application_instance()
    }

    fn update_qualification_for_debug(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> update_qualification_for_debug\n");
        client::get_application_manager_interface().update_qualification_for_debug()
    }

    fn is_qualification_transition_supported(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_qualification_transition_supported\n");
        client::get_application_manager_interface().is_qualification_transition_supported()
    }

    fn is_qualification_transition_supported_by_process_id(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_qualification_transition_supported_by_process_id\n");
        client::get_application_manager_interface().is_qualification_transition_supported_by_process_id()
    }

    fn get_rights_user_changed_event(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_rights_user_changed_event\n");
        client::get_application_manager_interface().get_rights_user_changed_event()
    }

    fn get_application_id_of_preomia(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_id_of_preomia\n");
        client::get_application_manager_interface().get_application_id_of_preomia()
    }

    fn register_device_lock_key(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> register_device_lock_key\n");
        client::get_application_manager_interface().register_device_lock_key(buf)
    }

    fn unregister_device_lock_key(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> unregister_device_lock_key\n");
        client::get_application_manager_interface().unregister_device_lock_key()
    }

    fn verify_device_lock_key(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> verify_device_lock_key\n");
        client::get_application_manager_interface().verify_device_lock_key(buf)
    }

    fn hide_application_icon(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> hide_application_icon\n");
        client::get_application_manager_interface().hide_application_icon()
    }

    fn show_application_icon(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> show_application_icon\n");
        client::get_application_manager_interface().show_application_icon()
    }

    fn hide_application_title(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> hide_application_title\n");
        client::get_application_manager_interface().hide_application_title()
    }

    fn show_application_title(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> show_application_title\n");
        client::get_application_manager_interface().show_application_title()
    }

    fn enable_game_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_game_card\n");
        client::get_application_manager_interface().enable_game_card()
    }

    fn disable_game_card(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> disable_game_card\n");
        client::get_application_manager_interface().disable_game_card()
    }

    fn enable_local_content_share(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> enable_local_content_share\n");
        client::get_application_manager_interface().enable_local_content_share()
    }

    fn disable_local_content_share(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> disable_local_content_share\n");
        client::get_application_manager_interface().disable_local_content_share()
    }

    fn is_application_icon_hidden(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_application_icon_hidden\n");
        client::get_application_manager_interface().is_application_icon_hidden()
    }

    fn is_application_title_hidden(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_application_title_hidden\n");
        client::get_application_manager_interface().is_application_title_hidden()
    }

    fn is_game_card_enabled(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_game_card_enabled\n");
        client::get_application_manager_interface().is_game_card_enabled()
    }

    fn is_local_content_share_enabled(&mut self) -> Result<bool> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> is_local_content_share_enabled\n");
        client::get_application_manager_interface().is_local_content_share_enabled()
    }

    fn get_application_certificate(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "mitm:IApplicationManagerInterface -> get_application_certificate\n");
        client::get_application_manager_interface().get_application_certificate()
    }
}

pub struct ServiceGetterInterface<const K: GetterServiceKind> {
    session: sf::Session
}

impl<const K: GetterServiceKind> sf::IObject for ServiceGetterInterface<K> {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec! [
            ipc_cmif_interface_make_command_meta!(get_read_only_application_control_data_interface: 7989),
            ipc_cmif_interface_make_command_meta!(get_application_manager_interface: 7996)
        ]
    }
}

impl<const K: GetterServiceKind> server::IMitmServerObject for ServiceGetterInterface<K> {
    fn new(info: sm::MitmProcessInfo) -> Self {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Opening NS ({:?}) mitm from process {:?}\n", K, ProgramId(info.program_id));
        Self { session: sf::Session::new() }
    }
}

impl<const K: GetterServiceKind> IServiceGetterInterface for ServiceGetterInterface<K> {
    fn get_read_only_application_control_data_interface(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Opening RO control data intf!\n");

        Ok(Shared::new(ReadOnlyApplicationControlDataInterface::new()))
    }

    fn get_application_manager_interface(&mut self) -> Result<Shared<dyn sf::IObject>> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Opening appman intf!\n");

        Ok(Shared::new(ApplicationManagerInterface::new()))
    }
}

impl<const K: GetterServiceKind> server::IMitmService for ServiceGetterInterface<K> {
    fn get_name() -> &'static str {
        get_getter_service_name::<K>()
    }

    fn should_mitm(_info: sm::MitmProcessInfo) -> bool {
        true
    }
}