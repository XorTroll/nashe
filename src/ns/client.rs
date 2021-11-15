use super::*;
use nx::ipc::sf;
use nx::service;

pub struct ReadOnlyApplicationControlDataInterface {
    session: sf::Session
}

impl sf::IObject for ReadOnlyApplicationControlDataInterface {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for ReadOnlyApplicationControlDataInterface {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IReadOnlyApplicationControlDataInterface for ReadOnlyApplicationControlDataInterface {
    fn get_application_control_data(&mut self, source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 0] (source, app_id, out_buf) => (size: u32))
    }
    
    fn get_application_desired_language(&mut self, lang_bitmask: u8) -> Result<u8> {
        ipc_client_send_request_command!([self.session.object_info; 1] (lang_bitmask) => (lang_idx: u8))
    }

    fn convert_application_language_to_language_code(&mut self, app_lang: u8) -> Result<CString<0x8>> {
        ipc_client_send_request_command!([self.session.object_info; 2] (app_lang) => (lang_code: CString<0x8>))
    }

    fn convert_language_code_to_application_language(&mut self, lang_code: CString<0x8>) -> Result<u8> {
        ipc_client_send_request_command!([self.session.object_info; 3] (lang_code) => (app_lang: u8))
    }

    fn select_application_desired_language(&mut self) -> Result<()> {
        panic!("Unsupported select_application_desired_language")
        // ipc_client_send_request_command!([self.session.object_info; 4] () => ())
    }
}

pub struct AsyncValue {
    session: sf::Session
}

impl sf::IObject for AsyncValue {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for AsyncValue {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IAsyncValue for AsyncValue {
    fn get_size(&mut self) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => (size: usize))
    }

    fn get(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1] (out_buf) => ())
    }

    fn cancel(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] () => ())
    }

    fn get_error_context(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3] (out_buf) => ())
    }
}

pub struct RequestServerStopper {
    session: sf::Session
}

impl sf::IObject for RequestServerStopper {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for RequestServerStopper {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IRequestServerStopper for RequestServerStopper {
}

pub struct GameCardStopper {
    session: sf::Session
}

impl sf::IObject for GameCardStopper {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for GameCardStopper {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IGameCardStopper for GameCardStopper {
}

pub struct AsyncResult {
    session: sf::Session
}

impl sf::IObject for AsyncResult {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for AsyncResult {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IAsyncResult for AsyncResult {
    fn get(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => ())
    }

    fn cancel(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => ())
    }

    fn get_error_context(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] (out_buf) => ())
    }
}

pub struct ProgressAsyncResult {
    session: sf::Session
}

impl sf::IObject for ProgressAsyncResult {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for ProgressAsyncResult {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IAsyncResult for ProgressAsyncResult {
    fn get(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => ())
    }

    fn cancel(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => ())
    }

    fn get_error_context(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 4] (out_buf) => ())
    }
}

impl IProgressAsyncResult for ProgressAsyncResult {
    fn get_progress(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] (out_buf) => ())
    }

    fn get_detail_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3] () => ())
    }
}

pub struct AsyncValueAndProgress {
    session: sf::Session
}

impl sf::IObject for AsyncValueAndProgress {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for AsyncValueAndProgress {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IAsyncValue for AsyncValueAndProgress {
    fn get_size(&mut self) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => (size: usize))
    }

    fn get(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1] (out_buf) => ())
    }

    fn cancel(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] () => ())
    }

    fn get_error_context(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3] (out_buf) => ())
    }
}

impl IAsyncValueAndProgress for AsyncValueAndProgress {
    fn get_progress(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 4] () => ())
    }
}

pub struct ProgressMonitorForDeleteUserSaveDataAll {
    session: sf::Session
}

impl sf::IObject for ProgressMonitorForDeleteUserSaveDataAll {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for ProgressMonitorForDeleteUserSaveDataAll {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IProgressMonitorForDeleteUserSaveDataAll for ProgressMonitorForDeleteUserSaveDataAll {
    fn get_system_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => (event: sf::CopyHandle))
    }

    fn is_finished(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => (finished: bool))
    }

    fn get_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2] () => ())
    }

    fn get_progress(&mut self) -> Result<ProgressForDeleteUserSaveDataAll> {
        ipc_client_send_request_command!([self.session.object_info; 10] () => (progress: ProgressForDeleteUserSaveDataAll))
    }
}

pub struct ApplicationResource {
    session: sf::Session
}

impl sf::IObject for ApplicationResource {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for ApplicationResource {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IApplicationResource for ApplicationResource {
    fn attach(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 0] () => ())
    }

    fn boost_system_memory_resource_limit(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => ())
    }
}

pub struct ApplicationManagerInterface {
    session: sf::Session
}

impl sf::IObject for ApplicationManagerInterface {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec!()
    }
}

impl service::IClientObject for ApplicationManagerInterface {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl IApplicationManagerInterface for ApplicationManagerInterface {
    fn list_application_record(&mut self, entry_offset: u32, out_record_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 0] (entry_offset, out_record_buf) => (count: u32))
    }

    fn generate_application_record_count(&mut self) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 1] () => (unk_count: u64))
    }

    fn get_application_record_update_system_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 2] () => (event: sf::CopyHandle))
    }

    fn get_application_view_deprecated(&mut self, in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3] (in_app_ids, out_views) => ())
    }

    fn delete_application_entity(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 4] (app_id) => ())
    }

    fn delete_application_completely(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 5] (app_id) => ())
    }

    fn is_any_application_entity_redundant(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 6] () => (redundant: bool))
    }

    fn delete_redundant_application_entity(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 7] () => ())
    }

    fn is_application_entity_movable(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 8] (storage_id, app_id) => (movable: bool))
    }

    fn move_application_entity(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 9] (storage_id, app_id) => ())
    }

    fn calculate_application_occupied_size(&mut self, app_id: ApplicationId) -> Result<ApplicationOccupiedSize> {
        ipc_client_send_request_command!([self.session.object_info; 11] (app_id) => (size: ApplicationOccupiedSize))
    }

    fn push_application_record(&mut self, last_modified_event: u8, app_id: ApplicationId, record_buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 16] (last_modified_event, app_id, record_buf) => ())
    }

    fn list_application_record_content_meta(&mut self, offset: u64, app_id: ApplicationId, out_meta_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 17] (offset, app_id, out_meta_buf) => (count: u32))
    }

    fn launch_application_old(&mut self, app_id: ApplicationId) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 19] (app_id) => (process_id: u64))
    }

    fn get_application_content_path(&mut self, meta_type: ContentMetaType, app_id: ApplicationId, out_path: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 21] (meta_type, app_id, out_path) => ())
    }

    fn terminate_application(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 22] (app_id) => ())
    }

    fn resolve_application_content_path(&mut self, meta_type: ContentMetaType, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 23] (meta_type, app_id) => ())
    }

    fn begin_install_application(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 26] (storage_id, app_id) => ())
    }

    fn delete_application_record(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 27] (app_id) => ())
    }

    fn request_application_update_info(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 30] (app_id) => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn request_update_application(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 31] (app_id) => ())
    }

    fn cancel_application_download(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 32] (app_id) => ())
    }

    fn resume_application_download(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 33] (app_id) => ())
    }

    fn update_version_list(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 35] (buf) => ())
    }

    fn push_launch_version(&mut self, version: u32, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 36] (version, app_id) => ())
    }

    fn list_required_version(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 37] (out_buf) => (count: u32))
    }

    fn check_application_launch_version(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 38] (app_id) => ())
    }

    fn check_application_launch_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 39] (app_id) => ())
    }

    fn get_application_logo_data(&mut self, app_id: ApplicationId, buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 40] (app_id, buf, out_buf) => (unk: u64))
    }

    fn calculate_application_download_required_size(&mut self, app_id: ApplicationId) -> Result<(u64, u64)> {
        ipc_client_send_request_command!([self.session.object_info; 41] (app_id) => (unk_1: u64, unk_2: u64))
    }

    fn cleanup_sd_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 42] () => ())
    }

    fn check_sd_card_mount_status(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 43] () => ())
    }

    fn get_sd_card_mount_status_changed_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 44] () => (event: sf::CopyHandle))
    }

    fn get_game_card_attachment_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 45] () => (event: sf::CopyHandle))
    }

    fn get_game_card_attachment_info(&mut self) -> Result<(u64, u64)> {
        ipc_client_send_request_command!([self.session.object_info; 46] () => (unk_1: u64, unk_2: u64))
    }

    fn get_total_space_size(&mut self, storage_id: StorageId) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 47] (storage_id) => (size: usize))
    }

    fn get_free_space_size(&mut self, storage_id: StorageId) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 48] (storage_id) => (size: usize))
    }

    fn get_sd_card_removed_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 49] () => (event: sf::CopyHandle))
    }

    fn get_game_card_update_detection_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 52] () => (event: sf::CopyHandle))
    }

    fn disable_application_auto_delete(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 53] (app_id) => ())
    }

    fn enable_application_auto_delete(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 54] (app_id) => ())
    }

    fn get_application_desired_language(&mut self, lang_bitmask: u8) -> Result<u8> {
        ipc_client_send_request_command!([self.session.object_info; 55] (lang_bitmask) => (lang_idx: u8))
    }

    fn set_application_terminate_result(&mut self, rc: ResultCode, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 56] (rc, app_id) => ())
    }

    fn clear_application_terminate_result(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 57] (app_id) => ())
    }

    fn get_last_sd_card_mount_unexpected_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 58] () => ())
    }

    fn convert_application_language_to_language_code(&mut self, app_lang: u8) -> Result<CString<0x8>> {
        ipc_client_send_request_command!([self.session.object_info; 59] (app_lang) => (lang_code: CString<0x8>))
    }

    fn convert_language_code_to_application_language(&mut self, lang_code: CString<0x8>) -> Result<u8> {
        ipc_client_send_request_command!([self.session.object_info; 60] (lang_code) => (app_lang: u8))
    }

    fn get_background_download_stress_task_info(&mut self) -> Result<(u64, u64)> {
        ipc_client_send_request_command!([self.session.object_info; 61] () => (unk_1: u64, unk_2: u64))
    }

    fn get_game_card_stopper(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 62] () => (stopper: Shared<GameCardStopper>))
    }

    fn is_system_program_installed(&mut self, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 63] (app_id) => (installed: bool))
    }

    fn start_apply_delta_task(&mut self, unk_app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 64] (unk_app_id) => ())
    }

    fn get_request_server_stopper(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 65] () => (stopper: Shared<RequestServerStopper>))
    }

    fn get_background_apply_delta_stress_task_info(&mut self) -> Result<(u64, u64)> {
        ipc_client_send_request_command!([self.session.object_info; 66] () => (unk_1: u64, unk_2: u64))
    }

    fn cancel_application_apply_delta(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 67] (app_id) => ())
    }

    fn resume_application_apply_delta(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 68] (app_id) => ())
    }

    fn calculate_application_apply_delta_required_size(&mut self, storage_id: StorageId, app_id: ApplicationId) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 69] (storage_id, app_id) => (size: usize))
    }

    fn resume_all(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 70] () => ())
    }

    fn get_storage_size(&mut self, storage_id: StorageId) -> Result<(usize, usize)> {
        ipc_client_send_request_command!([self.session.object_info; 71] (storage_id) => (total_size: usize, free_size: usize))
    }

    fn request_download_application(&mut self, unk_storage_id: StorageId, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 80] (unk_storage_id, app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn request_download_add_on_content(&mut self, unk_storage_id: StorageId, app_id: ApplicationId, buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 81] (unk_storage_id, app_id, buf) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn download_application(&mut self, unk_storage_id: StorageId, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 82] (unk_storage_id, app_id) => ())
    }

    fn check_application_resume_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 83] (app_id) => ())
    }

    fn get_dynamic_commit_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 84] () => (event: sf::CopyHandle))
    }

    fn request_update_application_2(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 85] (app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn enable_application_crash_report(&mut self, unk_enable: bool) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 86] (unk_enable) => ())
    }

    fn is_application_crash_report_enabled(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 87] () => (enabled: bool))
    }

    fn boost_system_memory_resource_limit(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 90] (app_id) => ())
    }

    fn deprecated_launch_application(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 91] () => ())
    }

    fn get_running_application_program_id(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 92] () => ())
    }

    fn get_main_application_program_index(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 93] () => ())
    }

    fn launch_application(&mut self, program_idx: u8, info: ApplicationLaunchInfo) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 94] (program_idx, info) => (process_id: u64))
    }

    fn get_application_launch_info(&mut self, app_id: ApplicationId) -> Result<ApplicationLaunchInfo> {
        ipc_client_send_request_command!([self.session.object_info; 95] (app_id) => (info: ApplicationLaunchInfo))
    }

    fn acquire_application_launch_info(&mut self, app_id: ApplicationId) -> Result<ApplicationLaunchInfo> {
        ipc_client_send_request_command!([self.session.object_info; 96] (app_id) => (info: ApplicationLaunchInfo))
    }

    fn get_main_application_program_index_by_application_launch_info(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 97] () => ())
    }

    fn enable_application_all_thread_dump_on_crash(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 98] () => ())
    }

    fn launch_dev_menu(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 99] () => ())
    }

    fn reset_to_factory_settings(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 100] () => ())
    }

    fn reset_to_factory_settings_without_user_save_data(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 101] () => ())
    }

    fn reset_to_factory_settings_for_refurbishment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 102] () => ())
    }

    fn reset_to_factory_settings_with_platform_region(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 103] () => ())
    }

    fn reset_to_factory_settings_with_platform_region_authentication(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 104] () => ())
    }

    fn request_reset_to_factory_settings_securely(&mut self, unk: u64, unk_2: sf::CopyHandle) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 105] (unk, unk_2) => (event: sf::CopyHandle, val: Shared<AsyncValueAndProgress>))
    }

    fn request_reset_to_factory_settings_with_platform_region_authentication_securely(&mut self, unk: u64, unk_2: u64, unk_3: sf::CopyHandle) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 106] (unk, unk_2, unk_3) => (event: sf::CopyHandle, val: Shared<AsyncValueAndProgress>))
    }

    fn calculate_user_save_data_statistics(&mut self, unk_1: u64, unk_2: u64) -> Result<(u64, u64)> {
        ipc_client_send_request_command!([self.session.object_info; 200] (unk_1, unk_2) => (unk_3: u64, unk_4: u64))
    }

    fn delete_user_save_data_all(&mut self, uid: Uid) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 201] (uid) => (monitor: Shared<ProgressMonitorForDeleteUserSaveDataAll>))
    }

    fn delete_user_system_save_data(&mut self, uid: Uid, id: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 210] (uid, id) => ())
    }

    fn delete_save_data(&mut self, space_id: SaveDataSpaceId, id: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 211] (space_id, id) => ())
    }

    fn unregister_network_service_account(&mut self, uid: Uid) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 220] (uid) => ())
    }

    fn unregister_network_service_account_with_user_save_data_deletion(&mut self, space_id: SaveDataSpaceId, id: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 221] (space_id, id) => ())
    }

    fn get_application_shell_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 300] () => (event: sf::CopyHandle))
    }

    fn pop_application_shell_event_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 301] (out_buf) => (unk: u32))
    }

    fn launch_library_applet(&mut self, program_id: ProgramId) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 302] (program_id) => (process_id: u64))
    }

    fn terminate_library_applet(&mut self, program_id: ProgramId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 303] (program_id) => ())
    }

    fn launch_system_applet(&mut self) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 304] () => (process_id: u64))
    }

    fn terminate_system_applet(&mut self, program_id: ProgramId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 305] (program_id) => ())
    }

    fn launch_overlay_applet(&mut self) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 306] () => (process_id: u64))
    }

    fn terminate_overlay_applet(&mut self, program_id: ProgramId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 307] (program_id) => ())
    }

    fn get_application_control_data(&mut self, source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 400] (source, app_id, out_buf) => (size: u32))
    }

    fn invalidate_all_application_control_cache(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 401] () => ())
    }

    fn request_download_application_control_data(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 402] (app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn get_max_application_control_cache_count(&mut self) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 403] () => (count: u32))
    }

    fn invalidate_application_control_cache(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 404] (app_id) => ())
    }

    fn list_application_control_cache_entry_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 405] (out_buf) => (unk: u32))
    }

    fn get_application_control_property(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 406] () => ())
    }

    fn list_application_title(&mut self, app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 407] (app_id_buf, source, tmem_handle, tmem_size) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>))
    }

    fn list_application_icon(&mut self, app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 408] (app_id_buf, source, tmem_handle, tmem_size) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>))
    }

    fn request_check_game_card_registration(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 502] (app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn request_game_card_registration_gold_point(&mut self, uid: Uid, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 503] (uid, app_id) => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn request_register_game_card(&mut self, unk: u32, uid: Uid, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 504] (unk, uid, app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn get_game_card_mount_failure_event(&mut self) -> Result<sf::CopyHandle> {
        ipc_client_send_request_command!([self.session.object_info; 505] () => (event: sf::CopyHandle))
    }

    fn is_game_card_inserted(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 506] () => (inserted: bool))
    }

    fn ensure_game_card_access(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 507] () => ())
    }

    fn get_last_game_card_mount_failure_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 508] () => ())
    }

    fn list_application_id_on_game_card(&mut self, out_app_id_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 509] (out_app_id_buf) => (count: u32))
    }

    fn get_game_card_platform_region(&mut self) -> Result<GameCardCompatibilityType> {
        ipc_client_send_request_command!([self.session.object_info; 510] () => (platform: GameCardCompatibilityType))
    }

    fn count_application_content_meta(&mut self, app_id: ApplicationId) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 600] (app_id) => (count: u32))
    }

    fn list_application_content_meta_status(&mut self, index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 601] (index, app_id, out_buf) => (count: u32))
    }

    fn list_available_add_on_content(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 602] (unk_1, unk_2, out_buf) => (unk_count: u64))
    }

    fn get_owned_application_content_meta_status(&mut self, unk_1: u64, unk_2: u64) -> Result<ApplicationContentMetaStatus> {
        ipc_client_send_request_command!([self.session.object_info; 603] (unk_1, unk_2) => (status: ApplicationContentMetaStatus))
    }

    fn register_contents_external_key(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 604] (unk_1, unk_2) => ())
    }

    fn list_application_content_meta_status_with_rights_check(&mut self, index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 605] (index, app_id, out_buf) => (count: u32))
    }

    fn get_content_meta_storage(&mut self, unk_1: u64, unk_2: u64) -> Result<StorageId> {
        ipc_client_send_request_command!([self.session.object_info; 606] (unk_1, unk_2) => (storage_id: StorageId))
    }

    fn list_available_add_on_content_new(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 607] (unk_1, unk_2, out_buf) => (unk_count: u64))
    }

    fn list_availability_assured_add_on_content(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 608] () => ())
    }

    fn push_download_task_list(&mut self, in_buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 700] (in_buf) => ())
    }

    fn clear_task_status_list(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 701] () => ())
    }

    fn request_download_task_list(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 702] () => ())
    }

    fn request_ensure_download_task(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 703] () => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn list_download_task_status(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 704] (out_buf) => (count: u32))
    }

    fn request_download_task_list_data(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 705] () => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn request_version_list(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 800] () => ())
    }

    fn list_version_list(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 801] (out_buf) => (count: u32))
    }

    fn request_version_list_data(&mut self) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 802] () => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn get_application_record(&mut self, app_id: ApplicationId) -> Result<ApplicationRecord> {
        ipc_client_send_request_command!([self.session.object_info; 900] (app_id) => (record: ApplicationRecord))
    }

    fn get_application_record_property(&mut self, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 901] (app_id, out_buf) => ())
    }

    fn enable_application_auto_update(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 902] (app_id) => ())
    }

    fn disable_application_auto_update(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 903] (app_id) => ())
    }

    fn touch_application(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 904] (app_id) => ())
    }

    fn request_application_update(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 905] (unk_1, unk_2) => ())
    }

    fn is_application_update_requested(&mut self, app_id: ApplicationId) -> Result<(bool, u32)> {
        ipc_client_send_request_command!([self.session.object_info; 906] (app_id) => (requested: bool, unk: u32))
    }

    fn withdraw_application_update_request(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 907] (app_id) => ())
    }

    fn list_application_record_installed_content_meta(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 908] (unk_1, unk_2, out_buf) => (count: u32))
    }

    fn withdraw_cleanup_add_on_contents_with_no_rights_recommendation(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 909] (app_id) => ())
    }

    fn has_application_record(&mut self, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 910] (app_id) => (has: bool))
    }

    fn set_pre_installed_application(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 911] () => ())
    }

    fn clear_pre_installed_application_flag(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 912] () => ())
    }

    fn list_all_application_record(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 913] () => ())
    }

    fn hide_application_record(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 914] () => ())
    }

    fn show_application_record(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 915] () => ())
    }

    fn is_application_auto_delete_disabled(&mut self, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 916] (app_id) => (disabled: bool))
    }

    fn request_verify_application_deprecated(&mut self, app_id: ApplicationId, tmem_handle: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 1000] (app_id, tmem_handle, tmem_size) => (event: sf::CopyHandle, rc: Shared<dyn sf::IObject>))
    }

    fn corrupt_application_for_debug(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1001] (unk_1, unk_2) => ())
    }

    fn request_verify_add_on_contents_rights(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 1002] (app_id) => (event: sf::CopyHandle, rc: Shared<ProgressAsyncResult>))
    }

    fn request_verify_application(&mut self, unk: u32, app_id: ApplicationId, tmem: sf::CopyHandle, tmem_size: usize) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 1003] (unk, app_id, tmem, tmem_size) => (event: sf::CopyHandle, rc: Shared<ProgressAsyncResult>))
    }

    fn corrupt_content_for_debug(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1004] () => ())
    }

    fn needs_update_vulnerability(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1200] () => (needs: bool))
    }

    fn is_any_application_entity_installed(&mut self, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1300] (app_id) => (installed: bool))
    }

    fn delete_application_content_entities(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1301] (unk_1, unk_2) => ())
    }

    fn cleanup_unrecorded_application_entity(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1302] (app_id) => ())
    }

    fn cleanup_add_on_contents_with_no_rights(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1303] (app_id) => ())
    }

    fn delete_application_content_entity(&mut self, unk_1: u64, unk_2: u64) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1304] (unk_1, unk_2) => ())
    }

    fn delete_application_completely_for_debug(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1308] () => ())
    }

    fn cleanup_unavailable_add_on_contents(&mut self, app_id: ApplicationId, uid: Uid) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1309] (app_id, uid) => ())
    }

    fn request_move_application_entity(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1310] () => ())
    }

    fn estimate_size_to_move(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1311] () => ())
    }

    fn has_movable_entity(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1312] () => ())
    }

    fn cleanup_orphan_contents(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1313] () => ())
    }

    fn check_precondition_satisfied_to_move(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1314] () => ())
    }

    fn prepare_shutdown(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1400] () => ())
    }

    fn format_sd_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1500] () => ())
    }

    fn needs_system_update_to_format_sd_card(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1501] () => (needs: bool))
    }

    fn get_last_sd_card_format_unexpected_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1502] () => ())
    }

    fn insert_sd_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1504] () => ())
    }

    fn remove_sd_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1505] () => ())
    }

    fn get_sd_card_startup_status(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1506] () => ())
    }

    fn get_system_seed_for_pseudo_device_id(&mut self) -> Result<[u8; 0x20]> {
        ipc_client_send_request_command!([self.session.object_info; 1600] () => (seed: [u8; 0x20]))
    }

    fn reset_system_seed_for_pseudo_device_id(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1601] () => ())
    }

    fn list_application_downloading_content_meta(&mut self, unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 1700] (unk_1, unk_2, out_buf) => (count: u32))
    }

    fn get_application_view(&mut self, in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1701] (in_app_ids, out_views) => ())
    }

    fn get_application_download_task_status(&mut self, app_id: ApplicationId) -> Result<u8> {
        ipc_client_send_request_command!([self.session.object_info; 1702] (app_id) => (status: u8))
    }

    fn get_application_view_download_error_context(&mut self, app_id: ApplicationId, out_err_ctx_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1703] (app_id, out_err_ctx_buf) => ())
    }

    fn get_application_view_with_promotion_info(&mut self, in_app_ids: sf::InMapAliasBuffer, out_data: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1704] (in_app_ids, out_data) => ())
    }

    fn is_patch_auto_deletable_application(&mut self, app_id: ApplicationId) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1705] (app_id) => (deletable: bool))
    }

    fn is_notification_setup_completed(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1800] () => (completed: bool))
    }

    fn get_last_notification_info_count(&mut self) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 1801] () => (unk: u64))
    }

    fn list_last_notification_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 1802] (out_buf) => (count: u32))
    }

    fn list_notification_task(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 1803] (out_buf) => (count: u32))
    }

    fn is_active_account(&mut self, unk: u32) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 1900] (unk) => (active: bool))
    }

    fn request_download_application_prepurchased_rights(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 1901] (app_id) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn get_application_ticket_info(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1902] () => ())
    }

    fn request_download_application_prepurchased_rights_for_account(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 1903] () => ())
    }

    fn get_system_delivery_info(&mut self, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2000] (out_buf) => ())
    }

    fn select_latest_system_delivery_info(&mut self, system_info_buf: sf::InMapAliasBuffer, system_infos_buf: sf::InMapAliasBuffer, app_infos_buf: sf::InMapAliasBuffer) -> Result<i32> {
        ipc_client_send_request_command!([self.session.object_info; 2001] (system_info_buf, system_infos_buf, app_infos_buf) => (index: i32))
    }

    fn verify_delivery_protocol_version(&mut self, system_info_buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2002] (system_info_buf) => ())
    }

    fn get_application_delivery_info(&mut self, bitmask: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 2003] (bitmask, app_id, out_buf) => (count: u32))
    }

    fn has_all_contents_to_deliver(&mut self, array_buf: sf::InMapAliasBuffer) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 2004] (array_buf) => (has: bool))
    }

    fn compare_application_delivery_info(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<i32> {
        ipc_client_send_request_command!([self.session.object_info; 2005] (buf_1, buf_2) => (cmp: i32))
    }

    fn can_deliver_application(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 2006] (buf_1, buf_2) => (can: bool))
    }

    fn list_content_meta_key_to_deliver_application(&mut self, unk: i32, in_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 2007] (unk, in_buf, out_buf) => (count: u32))
    }

    fn needs_system_update_to_deliver_application(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 2008] (buf_1, buf_2) => (needs: bool))
    }

    fn estimate_required_size(&mut self, meta_key_buf: sf::InMapAliasBuffer) -> Result<usize> {
        ipc_client_send_request_command!([self.session.object_info; 2009] (meta_key_buf) => (size: usize))
    }

    fn request_receive_application(&mut self, storage_id: StorageId, port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 2010] (storage_id, port, ipv4_addr, app_id, meta_keys_buf) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn commit_receive_application(&mut self, app_id: ApplicationId) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2011] (app_id) => ())
    }

    fn get_receive_application_progress(&mut self, app_id: ApplicationId) -> Result<ReceiveApplicationProgress> {
        ipc_client_send_request_command!([self.session.object_info; 2012] (app_id) => (progress: ReceiveApplicationProgress))
    }

    fn request_send_application(&mut self, port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 2013] (port, ipv4_addr, app_id, meta_keys_buf) => (event: sf::CopyHandle, rc: Shared<AsyncResult>))
    }

    fn get_send_application_progress(&mut self, app_id: ApplicationId) -> Result<SendApplicationProgress> {
        ipc_client_send_request_command!([self.session.object_info; 2014] (app_id) => (progress: SendApplicationProgress))
    }

    fn compare_system_delivery_info(&mut self, buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) -> Result<i32> {
        ipc_client_send_request_command!([self.session.object_info; 2015] (buf_1, buf_2) => (cmp: i32))
    }

    fn list_not_committed_content_meta(&mut self, unk: i32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 2016] (unk, app_id, out_buf) => (count: u32))
    }

    fn recover_download_task(&mut self, unk: u64, array: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2017] (unk, array) => ())
    }

    fn get_application_delivery_info_hash(&mut self, array: sf::InMapAliasBuffer) -> Result<[u8; 0x20]> {
        ipc_client_send_request_command!([self.session.object_info; 2018] (array) => (hash: [u8; 0x20]))
    }

    fn get_application_rights_on_client(&mut self, flags: u32, app_id: ApplicationId, uid: Uid, out_buf: sf::OutMapAliasBuffer) -> Result<u32> {
        ipc_client_send_request_command!([self.session.object_info; 2050] (flags, app_id, uid, out_buf) => (count: u32))
    }

    fn invalidate_rights_id_cache(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2051] () => ())
    }

    fn get_application_terminate_result(&mut self, app_id: ApplicationId) -> Result<ResultCode> {
        ipc_client_send_request_command!([self.session.object_info; 2100] (app_id) => (rc: ResultCode))
    }

    fn get_raw_application_terminate_result(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2101] () => ())
    }

    fn create_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2150] () => ())
    }

    fn destroy_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2151] () => ())
    }

    fn activate_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2152] () => ())
    }

    fn deactivate_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2153] () => ())
    }

    fn force_activate_rights_context_for_exit(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2154] () => ())
    }

    fn update_rights_environment_status(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2155] () => ())
    }

    fn create_rights_environment_for_micro_application_preomia(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2156] () => ())
    }

    fn add_target_application_to_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2160] () => ())
    }

    fn set_users_to_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2161] () => ())
    }

    fn get_rights_environment_status(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2170] () => ())
    }

    fn get_rights_environment_status_changed_event(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2171] () => ())
    }

    fn request_extend_expiration_in_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2180] () => ())
    }

    fn get_result_of_extend_expiration_in_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2181] () => ())
    }

    fn set_active_rights_context_using_state_to_rights_environment(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2182] () => ())
    }

    fn get_rights_environment_handle_for_application(&mut self, unk: u64) -> Result<u64> {
        ipc_client_send_request_command!([self.session.object_info; 2190] (unk) => (unk_2: u64))
    }

    fn get_rights_environment_count_for_debug(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2199] () => ())
    }

    fn get_game_card_application_copy_identifier(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2200] () => ())
    }

    fn get_installed_application_copy_identifier(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2201] () => ())
    }

    fn request_report_active_elicence(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2250] () => ())
    }

    fn list_event_log(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2300] () => ())
    }

    fn perform_auto_update_by_application_id(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2350] () => ())
    }

    fn request_no_download_rights_error_resolution(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 2351] (app_id) => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn request_resolve_no_download_rights_error(&mut self, app_id: ApplicationId) -> Result<(sf::CopyHandle, Shared<dyn sf::IObject>)> {
        ipc_client_send_request_command!([self.session.object_info; 2352] (app_id) => (event: sf::CopyHandle, val: Shared<AsyncValue>))
    }

    fn get_application_download_task_info(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2353] () => ())
    }

    fn prioritize_application_background_task(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2354] () => ())
    }

    fn prefer_storage_efficient_update(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2355] () => ())
    }

    fn request_storage_efficient_update_preferible(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2356] () => ())
    }

    fn get_promotion_info(&mut self, app_id_buf: sf::InMapAliasBuffer, uid_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2400] (app_id_buf, uid_buf, out_buf) => ())
    }

    fn count_promotion_info(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2401] () => ())
    }

    fn list_promotion_info(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2402] () => ())
    }

    fn import_promotion_json_for_debug(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2403] (buf) => ())
    }

    fn clear_promotion_info_for_debug(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2404] () => ())
    }

    fn confirm_available_time(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2500] () => ())
    }

    fn create_application_resource(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 2510] () => (res: Shared<ApplicationResource>))
    }

    fn get_application_resource(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 2511] () => (res: Shared<ApplicationResource>))
    }

    fn launch_micro_application_preomia(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2513] () => ())
    }

    fn clear_task_of_async_task_manager(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2514] () => ())
    }

    fn cleanup_all_placeholder_and_fragments_if_no_task(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2515] () => ())
    }

    fn ensure_application_certificate(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2516] () => ())
    }

    fn create_application_instance(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2517] () => ())
    }

    fn update_qualification_for_debug(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2518] () => ())
    }

    fn is_qualification_transition_supported(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2519] () => ())
    }

    fn is_qualification_transition_supported_by_process_id(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2520] () => ())
    }

    fn get_rights_user_changed_event(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2521] () => ())
    }

    fn get_application_id_of_preomia(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 2800] () => ())
    }

    fn register_device_lock_key(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3000] (buf) => ())
    }

    fn unregister_device_lock_key(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3001] () => ())
    }

    fn verify_device_lock_key(&mut self, buf: sf::InMapAliasBuffer) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3002] (buf) => ())
    }

    fn hide_application_icon(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3003] () => ())
    }

    fn show_application_icon(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3004] () => ())
    }

    fn hide_application_title(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3005] () => ())
    }

    fn show_application_title(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3006] () => ())
    }

    fn enable_game_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3007] () => ())
    }

    fn disable_game_card(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3008] () => ())
    }

    fn enable_local_content_share(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3009] () => ())
    }

    fn disable_local_content_share(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 3010] () => ())
    }

    fn is_application_icon_hidden(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 3011] () => (hidden: bool))
    }

    fn is_application_title_hidden(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 3012] () => (hidden: bool))
    }

    fn is_game_card_enabled(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 3013] () => (enabled: bool))
    }

    fn is_local_content_share_enabled(&mut self) -> Result<bool> {
        ipc_client_send_request_command!([self.session.object_info; 3014] () => (enabled: bool))
    }

    fn get_application_certificate(&mut self) -> Result<()> {
        ipc_client_send_request_command!([self.session.object_info; 9999] () => ())
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

impl<const K: GetterServiceKind> service::IClientObject for ServiceGetterInterface<K> {
    fn new(session: sf::Session) -> Self {
        Self { session }
    }
}

impl<const K: GetterServiceKind> IServiceGetterInterface for ServiceGetterInterface<K> {
    fn get_read_only_application_control_data_interface(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 7989] () => (intf: Shared<ReadOnlyApplicationControlDataInterface>))
    }

    fn get_application_manager_interface(&mut self) -> Result<Shared<dyn sf::IObject>> {
        ipc_client_send_request_command!([self.session.object_info; 7996] () => (intf: Shared<ApplicationManagerInterface>))
    }
}

impl<const K: GetterServiceKind> service::IService for ServiceGetterInterface<K> {
    fn get_name() -> &'static str {
        get_getter_service_name::<K>()
    }

    fn as_domain() -> bool {
        false
    }

    fn post_initialize(&mut self) -> Result<()> {
        Ok(())
    }
}

// Global implementation

static mut G_SRV: Shared<ServiceGetterInterface<{ GetterServiceKind::AM2 }>> = Shared::empty();

static mut G_RO_INTF: Shared<ReadOnlyApplicationControlDataInterface> = Shared::empty();
static mut G_APP_MAN_INTF: Shared<ApplicationManagerInterface> = Shared::empty();

pub fn initialize() -> Result<()> {
    unsafe {
        if G_SRV.is_null() {
            G_SRV = service::new_service_object()?;

            G_RO_INTF = G_SRV.get_read_only_application_control_data_interface()?.to();
            G_APP_MAN_INTF = G_SRV.get_application_manager_interface()?.to();
        }
    }

    Ok(())
}

pub fn finalize() {
    unsafe {
        if G_SRV.is_valid() {
            G_SRV.reset();
        }
    }
}

#[inline]
pub fn get_read_only_application_control_data_interface() -> &'static mut Shared<ReadOnlyApplicationControlDataInterface> {
    unsafe {
        &mut G_RO_INTF
    }
}

#[inline]
pub fn get_application_manager_interface() -> &'static mut Shared<ApplicationManagerInterface> {
    unsafe {
        &mut G_APP_MAN_INTF
    }
}