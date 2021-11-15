use nx::mem::Shared;
use nx::result::*;
use nx::ipc::sf;
use nx::ipc::server;
use nx::ipc::sf::sm;
use nx::diag::log;
use nx::util::CString;
use core::fmt::{Display, Debug, Formatter, Result as FmtResult};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationTitle {
    pub name: CString<0x200>,
    pub author: CString<0x100>
}

#[derive(Copy, Clone, PartialEq, Eq, Debug /*, Default */)]
#[repr(C)]
pub struct ApplicationControlProperty {
    pub titles: [ApplicationTitle; 16],
    pub todo_other_stuff: [u8; 0x1000]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ApplicationControlSource {
    CacheOnly,
    Storage,
    StorageOnly
}

// Note: ncm type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(C)]
pub struct ProgramId(pub u64);

impl Display for ProgramId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#018X}", self.0)
    }
}

impl Debug for ProgramId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#018X}", self.0)
    }
}

// Note: ncm type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[repr(C)]
pub struct ApplicationId(pub u64);

impl Display for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#018X}", self.0)
    }
}

impl Debug for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#018X}", self.0)
    }
}

// Note: ncm type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum StorageId {
    #[default]
    None = 0,
    Host = 1,
    GameCard = 2,
    BuiltInSystem = 3,
    BuiltInUser = 4,
    SdCard = 5,
    Any = 6
}

// Note: ncm type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum ContentMetaType {
    #[default]
    Unknown = 0x0,
    SystemProgram = 0x1,
    SystemData = 0x2,
    SystemUpdate = 0x3,
    BootImagePackage = 0x4,
    BootImagePackageSafe = 0x5,
    Application = 0x80,
    Patch = 0x81,
    AddOnContent = 0x82,
    Delta = 0x83
}

// Note: ncm type, not ns one

pub type ContentPath = CString<0x301>;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationRecord {
    pub app_id: ApplicationId,
    pub unk_type: u8,
    pub unk_2: u8,
    pub unk_3: [u8; 0x6],
    pub unk_4: u8,
    pub unk_reserved: [u8; 0x7]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationView {
    pub app_id: ApplicationId,
    pub unk_1: u32,
    pub unk_flags: u32,
    pub unk_2: [u8; 0x10],
    pub unk_3: u32,
    pub unk_4: u16,
    pub unk_reserved: [u8; 0x2],
    pub unk_6: [u8; 0x8],
    pub unk_7: [u8; 0x10],
    pub unk_8: u32,
    pub unk_9: u8,
    pub unk_reserved_2: [u8; 0xB]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationViewDeprecated {
    pub app_id: ApplicationId,
    pub unk_1: u32,
    pub unk_flags: u32,
    pub unk_2: [u8; 0x10],
    pub unk_3: u32,
    pub unk_4: u16,
    pub unk_reserved: [u8; 0x2],
    pub unk_7: [u8; 0x10],
    pub unk_8: u32,
    pub unk_9: u8,
    pub unk_reserved_3: [u8; 0x3]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ApplicationOccupiedSize {
    pub unk_data: [u8; 0x80]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ApplicationLaunchInfo {
    pub app_id: ApplicationId,
    pub version: u32,
    pub pm_launch_flags: u32 /* */,
    pub app_storage_id: StorageId,
    pub update_storage_id: StorageId,
    pub other_data: [u8; 0x2E]
}

// Note: account type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct Uid {
    pub hi: u64,
    pub lo: u64
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ProgressForDeleteUserSaveDataAll {
    pub data: [u8; 0x28]
}

// Note: fs type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SaveDataSpaceId {
    System = 0,
    User = 1,
    SdSystem = 2,
    Temporary = 3,
    SdUser = 4,
    ProperSystem = 100,
    SafeMode = 101
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationContentMetaStatus {
    pub meta_type: ContentMetaType,
    pub storage: StorageId,
    pub unk: u8, /**/
    pub pad: u8,
    pub version: u32,
    pub app_id: ApplicationId
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct DownloadTaskStatus {
    pub data: [u8; 0x20]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct VersionListData {
    pub data: [u8; 0x20 /**/]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct PromotionInfo {
    pub todo_data: [u8; 0x20 /**/]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationViewWithPromotionInfo {
    pub view: ApplicationView,
    pub info: PromotionInfo
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct SystemDeliveryInfo {
    pub system_delivery_protocol_version: u32,
    pub app_delivery_protocol_version: u32,
    pub has_exfat: bool,
    pub pad: [u8; 0x3],
    pub system_update_meta_version: u32,
    pub system_update_meta_id: u64,
    pub fw_variation_id: u8,
    pub updatable_fw_group_id: u8,
    pub platform_region: u8,
    pub unused: [u8; 0xC5],
    pub hmac_sha256_above: [u8; 0x20]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ApplicationDeliveryInfo {
    pub unk: [u8; 0x8],
    pub app_id: ApplicationId,
    pub app_version: u32,
    pub unk_2: [u8; 0x4],
    pub required_system_version: u32,
    pub unk_3: [u8; 0x4],
    pub unk_4: [u8; 0xC0],
    pub hmac_sha256_above: [u8; 0x20]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ReceiveApplicationProgress {
    pub data: [u8; 0x10]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct SendApplicationProgress {
    pub data: [u8; 0x10]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct ApplicationRightsOnClient {
    pub app_id: ApplicationId,
    pub uid: Uid,
    pub flags_1: u8,
    pub flags_2: u8,
    pub unk: [u8; 0x6]
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[repr(C)]
pub struct NoDownloadRightsErrorResolution {
}

// Note: fs type, not ns one

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum GameCardCompatibilityType {
    Global = 0,
    China = 1
}

pub trait IServiceGetterInterface {
    ipc_cmif_interface_define_command!(get_read_only_application_control_data_interface: () => (intf: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_application_manager_interface: () => (intf: Shared<dyn sf::IObject>));
}

pub trait IReadOnlyApplicationControlDataInterface {
    ipc_cmif_interface_define_command!(get_application_control_data: (source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (size: u32));
    ipc_cmif_interface_define_command!(get_application_desired_language: (lang_bitmask: u8) => (lang_idx: u8));
    ipc_cmif_interface_define_command!(convert_application_language_to_language_code: (app_lang: u8) => (lang_code: CString<0x8>));
    ipc_cmif_interface_define_command!(convert_language_code_to_application_language: (lang_code: CString<0x8>) => (app_lang: u8));
    ipc_cmif_interface_define_command!(select_application_desired_language: () => ());
}

pub trait IAsyncValue {
    ipc_cmif_interface_define_command!(get_size: () => (size: usize));
    ipc_cmif_interface_define_command!(get: (out_buf: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(cancel: () => ());
    ipc_cmif_interface_define_command!(get_error_context: (out_buf: sf::OutMapAliasBuffer) => ());
}

pub trait IRequestServerStopper {
}

pub trait IGameCardStopper {
}

pub trait IAsyncResult {
    ipc_cmif_interface_define_command!(get: () => ());
    ipc_cmif_interface_define_command!(cancel: () => ());
    ipc_cmif_interface_define_command!(get_error_context: (out_buf: sf::OutMapAliasBuffer) => ());
}

pub trait IProgressAsyncResult: IAsyncResult {
    /**/ ipc_cmif_interface_define_command!(get_progress: (out_buf: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(get_detail_result: () => ());
}

pub trait IAsyncValueAndProgress: IAsyncValue {
    /**/ ipc_cmif_interface_define_command!(get_progress: () => ());
}

pub trait IProgressMonitorForDeleteUserSaveDataAll {
    ipc_cmif_interface_define_command!(get_system_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(is_finished: () => (finished: bool));
    ipc_cmif_interface_define_command!(get_result: () => ());
    ipc_cmif_interface_define_command!(get_progress: () => (progress: ProgressForDeleteUserSaveDataAll));
}

pub trait IApplicationResource {
    /**/ ipc_cmif_interface_define_command!(attach: () => ());
    /**/ ipc_cmif_interface_define_command!(boost_system_memory_resource_limit: () => ());
}

pub trait IApplicationManagerInterface {
    ipc_cmif_interface_define_command!(list_application_record: (entry_offset: u32, out_record_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(generate_application_record_count: () => (unk_count: u64));
    ipc_cmif_interface_define_command!(get_application_record_update_system_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(get_application_view_deprecated: (in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(delete_application_entity: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(delete_application_completely: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(is_any_application_entity_redundant: () => (redundant: bool));
    ipc_cmif_interface_define_command!(delete_redundant_application_entity: () => ());
    ipc_cmif_interface_define_command!(is_application_entity_movable: (storage_id: StorageId, app_id: ApplicationId) => (movable: bool));
    ipc_cmif_interface_define_command!(move_application_entity: (storage_id: StorageId, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(calculate_application_occupied_size: (app_id: ApplicationId) => (size: ApplicationOccupiedSize));
    ipc_cmif_interface_define_command!(push_application_record: (last_modified_event: u8, app_id: ApplicationId, record_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(list_application_record_content_meta: (offset: u64, app_id: ApplicationId, out_meta_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(launch_application_old: (app_id: ApplicationId) => (process_id: u64));
    ipc_cmif_interface_define_command!(get_application_content_path: (meta_type: ContentMetaType, app_id: ApplicationId, out_path: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(terminate_application: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(resolve_application_content_path: (meta_type: ContentMetaType, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(begin_install_application: (storage_id: StorageId, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(delete_application_record: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(request_application_update_info: (app_id: ApplicationId) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(request_update_application: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(cancel_application_download: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(resume_application_download: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(update_version_list: (buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(push_launch_version: (version: u32, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(list_required_version: (out_buf: sf::OutMapAliasBuffer) => (unk_count: u32));
    ipc_cmif_interface_define_command!(check_application_launch_version: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(check_application_launch_rights: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(get_application_logo_data: (app_id: ApplicationId, buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) => (unk: u64));
    /**/ ipc_cmif_interface_define_command!(calculate_application_download_required_size: (app_id: ApplicationId) => (unk_1: u64, unk_2: u64));
    ipc_cmif_interface_define_command!(cleanup_sd_card: () => ());
    ipc_cmif_interface_define_command!(check_sd_card_mount_status: () => ());
    ipc_cmif_interface_define_command!(get_sd_card_mount_status_changed_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(get_game_card_attachment_event: () => (event: sf::CopyHandle));
    /**/ ipc_cmif_interface_define_command!(get_game_card_attachment_info: () => (unk_1: u64, unk_2: u64));
    ipc_cmif_interface_define_command!(get_total_space_size: (storage_id: StorageId) => (size: usize));
    ipc_cmif_interface_define_command!(get_free_space_size: (storage_id: StorageId) => (size: usize));
    ipc_cmif_interface_define_command!(get_sd_card_removed_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(get_game_card_update_detection_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(disable_application_auto_delete: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(enable_application_auto_delete: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(get_application_desired_language: (lang_bitmask: u8) => (lang_idx: u8));
    ipc_cmif_interface_define_command!(set_application_terminate_result: (rc: ResultCode, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(clear_application_terminate_result: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(get_last_sd_card_mount_unexpected_result: () => ());
    ipc_cmif_interface_define_command!(convert_application_language_to_language_code: (app_lang: u8) => (lang_code: CString<0x8>));
    ipc_cmif_interface_define_command!(convert_language_code_to_application_language: (lang_code: CString<0x8>) => (app_lang: u8));
    ipc_cmif_interface_define_command!(get_background_download_stress_task_info: () => (unk_1: u64, unk_2: u64));
    ipc_cmif_interface_define_command!(get_game_card_stopper: () => (stopper: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(is_system_program_installed: (app_id: ApplicationId) => (installed: bool));
    /**/ ipc_cmif_interface_define_command!(start_apply_delta_task: (unk_app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(get_request_server_stopper: () => (stopper: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_background_apply_delta_stress_task_info: () => (unk_1: u64, unk_2: u64));
    ipc_cmif_interface_define_command!(cancel_application_apply_delta: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(resume_application_apply_delta: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(calculate_application_apply_delta_required_size: (storage_id: StorageId, app_id: ApplicationId) => (size: usize));
    ipc_cmif_interface_define_command!(resume_all: () => ());
    ipc_cmif_interface_define_command!(get_storage_size: (storage_id: StorageId) => (total_size: usize, free_size: usize));
    /**/ ipc_cmif_interface_define_command!(request_download_application: (unk_storage_id: StorageId, app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(request_download_add_on_content: (unk_storage_id: StorageId, app_id: ApplicationId, buf: sf::InMapAliasBuffer) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(download_application: (unk_storage_id: StorageId, app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(check_application_resume_rights: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(get_dynamic_commit_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(request_update_application_2: (app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(enable_application_crash_report: (unk_enable: bool) => ());
    ipc_cmif_interface_define_command!(is_application_crash_report_enabled: () => (enabled: bool));
    /**/ ipc_cmif_interface_define_command!(boost_system_memory_resource_limit: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(deprecated_launch_application: () => ());
    /**/ ipc_cmif_interface_define_command!(get_running_application_program_id: () => ());
    /**/ ipc_cmif_interface_define_command!(get_main_application_program_index: () => ());
    ipc_cmif_interface_define_command!(launch_application: (program_idx: u8, info: ApplicationLaunchInfo) => (process_id: u64));
    ipc_cmif_interface_define_command!(get_application_launch_info: (app_id: ApplicationId) => (info: ApplicationLaunchInfo));
    ipc_cmif_interface_define_command!(acquire_application_launch_info: (app_id: ApplicationId) => (info: ApplicationLaunchInfo));
    /**/ ipc_cmif_interface_define_command!(get_main_application_program_index_by_application_launch_info: () => ());
    /**/ ipc_cmif_interface_define_command!(enable_application_all_thread_dump_on_crash: () => ());
    ipc_cmif_interface_define_command!(launch_dev_menu: () => ());
    ipc_cmif_interface_define_command!(reset_to_factory_settings: () => ());
    ipc_cmif_interface_define_command!(reset_to_factory_settings_without_user_save_data: () => ());
    ipc_cmif_interface_define_command!(reset_to_factory_settings_for_refurbishment: () => ());
    ipc_cmif_interface_define_command!(reset_to_factory_settings_with_platform_region: () => ());
    ipc_cmif_interface_define_command!(reset_to_factory_settings_with_platform_region_authentication: () => ());
    /**/ ipc_cmif_interface_define_command!(request_reset_to_factory_settings_securely: (unk: u64, unk_2: sf::CopyHandle) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(request_reset_to_factory_settings_with_platform_region_authentication_securely: (unk: u64, unk_2: u64, unk_3: sf::CopyHandle) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(calculate_user_save_data_statistics: (unk_1: u64, unk_2: u64) => (unk_3: u64, unk_4: u64));
    ipc_cmif_interface_define_command!(delete_user_save_data_all: (uid: Uid) => (monitor: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(delete_user_system_save_data: (uid: Uid, id: u64) => ());
    ipc_cmif_interface_define_command!(delete_save_data: (space_id: SaveDataSpaceId, id: u64) => ());
    ipc_cmif_interface_define_command!(unregister_network_service_account: (uid: Uid) => ());
    ipc_cmif_interface_define_command!(unregister_network_service_account_with_user_save_data_deletion: (space_id: SaveDataSpaceId, id: u64) => ());
    ipc_cmif_interface_define_command!(get_application_shell_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(pop_application_shell_event_info: (out_buf: sf::OutMapAliasBuffer) => (unk: u32));
    ipc_cmif_interface_define_command!(launch_library_applet: (program_id: ProgramId) => (process_id: u64));
    ipc_cmif_interface_define_command!(terminate_library_applet: (program_id: ProgramId) => ());
    ipc_cmif_interface_define_command!(launch_system_applet: () => (process_id: u64));
    ipc_cmif_interface_define_command!(terminate_system_applet: (program_id: ProgramId) => ());
    ipc_cmif_interface_define_command!(launch_overlay_applet: () => (process_id: u64));
    ipc_cmif_interface_define_command!(terminate_overlay_applet: (program_id: ProgramId) => ());
    ipc_cmif_interface_define_command!(get_application_control_data: (source: ApplicationControlSource, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (size: u32));
    ipc_cmif_interface_define_command!(invalidate_all_application_control_cache: () => ());
    ipc_cmif_interface_define_command!(request_download_application_control_data: (app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_max_application_control_cache_count: () => (count: u32));
    ipc_cmif_interface_define_command!(invalidate_application_control_cache: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(list_application_control_cache_entry_info: (out_buf: sf::OutMapAliasBuffer) => (unk: u32));
    /**/ ipc_cmif_interface_define_command!(get_application_control_property: () => ());
    ipc_cmif_interface_define_command!(list_application_title: (app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(list_application_icon: (app_id_buf: sf::InMapAliasBuffer, source: ApplicationControlSource, tmem_handle: sf::CopyHandle, tmem_size: usize) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_check_game_card_registration: (app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_game_card_registration_gold_point: (uid: Uid, app_id: ApplicationId) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_register_game_card: (unk: u32, uid: Uid, app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_game_card_mount_failure_event: () => (event: sf::CopyHandle));
    ipc_cmif_interface_define_command!(is_game_card_inserted: () => (inserted: bool));
    ipc_cmif_interface_define_command!(ensure_game_card_access: () => ());
    ipc_cmif_interface_define_command!(get_last_game_card_mount_failure_result: () => ());
    ipc_cmif_interface_define_command!(list_application_id_on_game_card: (out_app_id_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(get_game_card_platform_region: () => (platform: GameCardCompatibilityType));
    ipc_cmif_interface_define_command!(count_application_content_meta: (app_id: ApplicationId) => (count: u32));
    ipc_cmif_interface_define_command!(list_application_content_meta_status: (index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    /**/ ipc_cmif_interface_define_command!(list_available_add_on_content: (unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) => (unk_count: u64));
    /**/ ipc_cmif_interface_define_command!(get_owned_application_content_meta_status: (unk_1: u64, unk_2: u64) => (status: ApplicationContentMetaStatus));
    /**/ ipc_cmif_interface_define_command!(register_contents_external_key: (unk_1: u64, unk_2: u64) => ());
    ipc_cmif_interface_define_command!(list_application_content_meta_status_with_rights_check: (index: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    /**/ ipc_cmif_interface_define_command!(get_content_meta_storage: (unk_1: u64, unk_2: u64) => (storage: StorageId));
    /**/ ipc_cmif_interface_define_command!(list_available_add_on_content_new: (unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) => (unk_count: u64));
    /**/ ipc_cmif_interface_define_command!(list_availability_assured_add_on_content: () => ());
    ipc_cmif_interface_define_command!(push_download_task_list: (in_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(clear_task_status_list: () => ());
    ipc_cmif_interface_define_command!(request_download_task_list: () => ());
    ipc_cmif_interface_define_command!(request_ensure_download_task: () => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(list_download_task_status: (out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(request_download_task_list_data: () => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_version_list: () => ());
    ipc_cmif_interface_define_command!(list_version_list: (out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(request_version_list_data: () => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_application_record: (app_id: ApplicationId) => (record: ApplicationRecord));
    ipc_cmif_interface_define_command!(get_application_record_property: (app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(enable_application_auto_update: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(disable_application_auto_update: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(touch_application: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(request_application_update: (unk_1: u64, unk_2: u64) => ());
    ipc_cmif_interface_define_command!(is_application_update_requested: (app_id: ApplicationId) => (requested: bool, unk: u32));
    ipc_cmif_interface_define_command!(withdraw_application_update_request: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(list_application_record_installed_content_meta: (unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(withdraw_cleanup_add_on_contents_with_no_rights_recommendation: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(has_application_record: (app_id: ApplicationId) => (has: bool));
    /**/ ipc_cmif_interface_define_command!(set_pre_installed_application: () => ());
    /**/ ipc_cmif_interface_define_command!(clear_pre_installed_application_flag: () => ());
    /**/ ipc_cmif_interface_define_command!(list_all_application_record: () => ());
    /**/ ipc_cmif_interface_define_command!(hide_application_record: () => ());
    /**/ ipc_cmif_interface_define_command!(show_application_record: () => ());
    /**/ ipc_cmif_interface_define_command!(is_application_auto_delete_disabled: (app_id: ApplicationId) => (disabled: bool));
    ipc_cmif_interface_define_command!(request_verify_application_deprecated: (app_id: ApplicationId, tmem_handle: sf::CopyHandle, tmem_size: usize) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(corrupt_application_for_debug: (unk_1: u64, unk_2: u64) => ());
    ipc_cmif_interface_define_command!(request_verify_add_on_contents_rights: (app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_verify_application: (unk: u32, app_id: ApplicationId, tmem: sf::CopyHandle, tmem_size: usize) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(corrupt_content_for_debug: () => ());
    ipc_cmif_interface_define_command!(needs_update_vulnerability: () => (needs: bool));
    ipc_cmif_interface_define_command!(is_any_application_entity_installed: (app_id: ApplicationId) => (installed: bool));
    /**/ ipc_cmif_interface_define_command!(delete_application_content_entities: (unk_1: u64, unk_2: u64) => ());
    ipc_cmif_interface_define_command!(cleanup_unrecorded_application_entity: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(cleanup_add_on_contents_with_no_rights: (app_id: ApplicationId) => ());
    /**/ ipc_cmif_interface_define_command!(delete_application_content_entity: (unk_1: u64, unk_2: u64) => ());
    /**/ ipc_cmif_interface_define_command!(delete_application_completely_for_debug: () => ());
    ipc_cmif_interface_define_command!(cleanup_unavailable_add_on_contents: (app_id: ApplicationId, uid: Uid) => ());
    /**/ ipc_cmif_interface_define_command!(request_move_application_entity: () => ());
    /**/ ipc_cmif_interface_define_command!(estimate_size_to_move: () => ());
    /**/ ipc_cmif_interface_define_command!(has_movable_entity: () => ());
    /**/ ipc_cmif_interface_define_command!(cleanup_orphan_contents: () => ());
    /**/ ipc_cmif_interface_define_command!(check_precondition_satisfied_to_move: () => ());
    ipc_cmif_interface_define_command!(prepare_shutdown: () => ());
    ipc_cmif_interface_define_command!(format_sd_card: () => ());
    ipc_cmif_interface_define_command!(needs_system_update_to_format_sd_card: () => (needs: bool));
    ipc_cmif_interface_define_command!(get_last_sd_card_format_unexpected_result: () => ());
    ipc_cmif_interface_define_command!(insert_sd_card: () => ());
    ipc_cmif_interface_define_command!(remove_sd_card: () => ());
    /**/ ipc_cmif_interface_define_command!(get_sd_card_startup_status: () => ());
    ipc_cmif_interface_define_command!(get_system_seed_for_pseudo_device_id: () => (system_seed: [u8; 0x20]));
    ipc_cmif_interface_define_command!(reset_system_seed_for_pseudo_device_id: () => ());
    ipc_cmif_interface_define_command!(list_application_downloading_content_meta: (unk_1: u64, unk_2: u64, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(get_application_view: (in_app_ids: sf::InMapAliasBuffer, out_views: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(get_application_download_task_status: (app_id: ApplicationId) => (status: u8));
    ipc_cmif_interface_define_command!(get_application_view_download_error_context: (app_id: ApplicationId, out_err_ctx_buf: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(get_application_view_with_promotion_info: (in_app_ids: sf::InMapAliasBuffer, out_data: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(is_patch_auto_deletable_application: (app_id: ApplicationId) => (is: bool));
    ipc_cmif_interface_define_command!(is_notification_setup_completed: () => (completed: bool));
    /**/ ipc_cmif_interface_define_command!(get_last_notification_info_count: () => (unk: u64));
    ipc_cmif_interface_define_command!(list_last_notification_info: (out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(list_notification_task: (out_buf: sf::OutMapAliasBuffer) => (count: u32));
    /**/ ipc_cmif_interface_define_command!(is_active_account: (unk: u32) => (active: bool));
    ipc_cmif_interface_define_command!(request_download_application_prepurchased_rights: (app_id: ApplicationId) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(get_application_ticket_info: () => ());
    /**/ ipc_cmif_interface_define_command!(request_download_application_prepurchased_rights_for_account: () => ());
    ipc_cmif_interface_define_command!(get_system_delivery_info: (out_buf: sf::OutMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(select_latest_system_delivery_info: (system_info_buf: sf::InMapAliasBuffer, system_infos_buf: sf::InMapAliasBuffer, app_infos_buf: sf::InMapAliasBuffer) => (index: i32));
    ipc_cmif_interface_define_command!(verify_delivery_protocol_version: (system_info_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(get_application_delivery_info: (bitmask: u32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(has_all_contents_to_deliver: (array_buf: sf::InMapAliasBuffer) => (has: bool));
    ipc_cmif_interface_define_command!(compare_application_delivery_info: (buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) => (cmp: i32));
    ipc_cmif_interface_define_command!(can_deliver_application: (buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) => (can: bool));
    ipc_cmif_interface_define_command!(list_content_meta_key_to_deliver_application: (unk: i32, in_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    ipc_cmif_interface_define_command!(needs_system_update_to_deliver_application: (buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) => (needs: bool));
    ipc_cmif_interface_define_command!(estimate_required_size: (meta_key_buf: sf::InMapAliasBuffer) => (size: usize));
    ipc_cmif_interface_define_command!(request_receive_application: (storage_id: StorageId, port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(commit_receive_application: (app_id: ApplicationId) => ());
    ipc_cmif_interface_define_command!(get_receive_application_progress: (app_id: ApplicationId) => (progress: ReceiveApplicationProgress));
    ipc_cmif_interface_define_command!(request_send_application: (port: u16, ipv4_addr: u32, app_id: ApplicationId, meta_keys_buf: sf::InMapAliasBuffer) => (event: sf::CopyHandle, async_rc: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_send_application_progress: (app_id: ApplicationId) => (progress: SendApplicationProgress));
    ipc_cmif_interface_define_command!(compare_system_delivery_info: (buf_1: sf::InMapAliasBuffer, buf_2: sf::InMapAliasBuffer) => (cmp: i32));
    ipc_cmif_interface_define_command!(list_not_committed_content_meta: (unk: i32, app_id: ApplicationId, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    /**/ ipc_cmif_interface_define_command!(recover_download_task: (unk: u64, array: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(get_application_delivery_info_hash: (array: sf::InMapAliasBuffer) => (sha256_hash: [u8; 0x20]));
    ipc_cmif_interface_define_command!(get_application_rights_on_client: (flags: u32, app_id: ApplicationId, uid: Uid, out_buf: sf::OutMapAliasBuffer) => (count: u32));
    /**/ ipc_cmif_interface_define_command!(invalidate_rights_id_cache: () => ());
    ipc_cmif_interface_define_command!(get_application_terminate_result: (app_id: ApplicationId) => (rc: ResultCode));
    /**/ ipc_cmif_interface_define_command!(get_raw_application_terminate_result: () => ());
    /**/ ipc_cmif_interface_define_command!(create_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(destroy_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(activate_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(deactivate_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(force_activate_rights_context_for_exit: () => ());
    /**/ ipc_cmif_interface_define_command!(update_rights_environment_status: () => ());
    /**/ ipc_cmif_interface_define_command!(create_rights_environment_for_micro_application_preomia: () => ());
    /**/ ipc_cmif_interface_define_command!(add_target_application_to_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(set_users_to_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(get_rights_environment_status: () => ());
    /**/ ipc_cmif_interface_define_command!(get_rights_environment_status_changed_event: () => ());
    /**/ ipc_cmif_interface_define_command!(request_extend_expiration_in_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(get_result_of_extend_expiration_in_rights_environment: () => ());
    /**/ ipc_cmif_interface_define_command!(set_active_rights_context_using_state_to_rights_environment: () => ());
    ipc_cmif_interface_define_command!(get_rights_environment_handle_for_application: (/**/ unk: u64) => (unk_2: u64));
    /**/ ipc_cmif_interface_define_command!(get_rights_environment_count_for_debug: () => ());
    /**/ ipc_cmif_interface_define_command!(get_game_card_application_copy_identifier: () => ());
    /**/ ipc_cmif_interface_define_command!(get_installed_application_copy_identifier: () => ());
    /**/ ipc_cmif_interface_define_command!(request_report_active_elicence: () => ());
    /**/ ipc_cmif_interface_define_command!(list_event_log: () => ());
    /**/ ipc_cmif_interface_define_command!(perform_auto_update_by_application_id: () => ());
    ipc_cmif_interface_define_command!(request_no_download_rights_error_resolution: (app_id: ApplicationId) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(request_resolve_no_download_rights_error: (app_id: ApplicationId) => (event: sf::CopyHandle, val: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(get_application_download_task_info: () => ());
    /**/ ipc_cmif_interface_define_command!(prioritize_application_background_task: () => ());
    /**/ ipc_cmif_interface_define_command!(prefer_storage_efficient_update: () => ());
    /**/ ipc_cmif_interface_define_command!(request_storage_efficient_update_preferible: () => ());
    ipc_cmif_interface_define_command!(get_promotion_info: (app_id_buf: sf::InMapAliasBuffer, uid_buf: sf::InMapAliasBuffer, out_buf: sf::OutMapAliasBuffer) => ());
    /**/ ipc_cmif_interface_define_command!(count_promotion_info: () => ());
    /**/ ipc_cmif_interface_define_command!(list_promotion_info: () => ());
    ipc_cmif_interface_define_command!(import_promotion_json_for_debug: (buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(clear_promotion_info_for_debug: () => ());
    /**/ ipc_cmif_interface_define_command!(confirm_available_time: () => ());
    ipc_cmif_interface_define_command!(create_application_resource: () => (resource: Shared<dyn sf::IObject>));
    ipc_cmif_interface_define_command!(get_application_resource: () => (resource: Shared<dyn sf::IObject>));
    /**/ ipc_cmif_interface_define_command!(launch_micro_application_preomia: () => ());
    /**/ ipc_cmif_interface_define_command!(clear_task_of_async_task_manager: () => ());
    /**/ ipc_cmif_interface_define_command!(cleanup_all_placeholder_and_fragments_if_no_task: () => ());
    /**/ ipc_cmif_interface_define_command!(ensure_application_certificate: () => ());
    /**/ ipc_cmif_interface_define_command!(create_application_instance: () => ());
    /**/ ipc_cmif_interface_define_command!(update_qualification_for_debug: () => ());
    /**/ ipc_cmif_interface_define_command!(is_qualification_transition_supported: () => ());
    /**/ ipc_cmif_interface_define_command!(is_qualification_transition_supported_by_process_id: () => ());
    /**/ ipc_cmif_interface_define_command!(get_rights_user_changed_event: () => ());
    /**/ ipc_cmif_interface_define_command!(get_application_id_of_preomia: () => ());
    ipc_cmif_interface_define_command!(register_device_lock_key: (buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(unregister_device_lock_key: () => ());
    ipc_cmif_interface_define_command!(verify_device_lock_key: (buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(hide_application_icon: () => ());
    ipc_cmif_interface_define_command!(show_application_icon: () => ());
    ipc_cmif_interface_define_command!(hide_application_title: () => ());
    ipc_cmif_interface_define_command!(show_application_title: () => ());
    ipc_cmif_interface_define_command!(enable_game_card: () => ());
    ipc_cmif_interface_define_command!(disable_game_card: () => ());
    ipc_cmif_interface_define_command!(enable_local_content_share: () => ());
    ipc_cmif_interface_define_command!(disable_local_content_share: () => ());
    ipc_cmif_interface_define_command!(is_application_icon_hidden: () => (hidden: bool));
    ipc_cmif_interface_define_command!(is_application_title_hidden: () => (hidden: bool));
    ipc_cmif_interface_define_command!(is_game_card_enabled: () => (enabled: bool));
    ipc_cmif_interface_define_command!(is_local_content_share_enabled: () => (enabled: bool));
    /**/ ipc_cmif_interface_define_command!(get_application_certificate: () => ());
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GetterServiceKind {
    AM2,
    EC,
    RID,
    RT,
    WEB,
    RO
}

pub const fn get_getter_service_name<const K: GetterServiceKind>() -> &'static str {
    match K {
        GetterServiceKind::AM2 => nul!("ns:am2"),
        GetterServiceKind::EC => nul!("ns:ec"),
        GetterServiceKind::RID => nul!("ns:rid"),
        GetterServiceKind::RT => nul!("ns:rt"),
        GetterServiceKind::WEB => nul!("ns:web"),
        GetterServiceKind::RO => nul!("ns:ro")
    }
}

pub mod client;

pub mod mitm;