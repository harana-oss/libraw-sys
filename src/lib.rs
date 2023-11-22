#![allow(non_camel_case_types,non_snake_case)]

extern crate libc;

use libc::{c_void,c_char,c_uchar,c_schar, c_short,c_ushort,c_int,c_uint,c_float,c_double,c_long,size_t,time_t};

#[repr(C)]
pub struct libraw_area_t {
    pub t: c_ushort,
    pub l: c_ushort,
    pub b: c_ushort,
    pub r: c_ushort
}

#[repr(C)]
pub struct libraw_data_t {
    pub image: *mut [c_ushort; 4],
    pub sizes: libraw_image_sizes_t,
    pub idata: libraw_iparams_t,
    #[cfg(have_lensinfo)]
    pub lens: libraw_lensinfo_t,
    pub makernotes: libraw_makernotes_t,
    pub shootinginfo: libraw_shootinginfo_t,
    pub params: libraw_output_params_t,
    pub rawparams: libraw_raw_unpack_params_t,
    pub progress_flags: c_uint,
    pub process_warnings: c_uint,
    pub color: libraw_colordata_t,
    pub other: libraw_imgother_t,
    pub thumbnail: libraw_thumbnail_t,
    pub thumbs_list: libraw_thumbnail_list_t,
    pub rawdata: libraw_rawdata_t,
    pub parent_class: *mut c_void,
}

#[repr(C)]
pub struct libraw_shootinginfo_t {
    pub DriveMode: c_short,
    pub FocusMode: c_short,
    pub MeteringMode: c_short,
    pub AFPoint: c_short,
    pub ExposureMode: c_short,
    pub ExposureProgram: c_short,
    pub ImageStabilization: c_short,
    pub BodySerial: [c_char; 64],
    pub InternalBodySerial: [c_char; 64]
}

#[repr(C)]
pub struct libraw_makernotes_t {
    #[cfg(have_canon_makernotes)]
    pub canon: libraw_canon_makernotes_t,

    #[cfg(have_fuji_info)]
    pub fuji: libraw_fuji_info_t,

    #[cfg(have_hasselblad_makernotes)]
    pub hassleblad: libraw_hasselblad_makernotes_t,

    #[cfg(have_kodak_makernotes)]
    pub kodak: libraw_kodak_makernotes_t,

    #[cfg(have_nikon_makernotes)]
    pub nikon: libraw_nikon_makernotes_t,

    #[cfg(have_olympus_makernotes)]
    pub olympus: libraw_olympus_makernotes_t,

    #[cfg(have_p1_makernotes)]
    pub phaseone: libraw_p1_makernotes_t,

    #[cfg(have_panasonic_makernotes)]
    pub panasonic: libraw_panasonic_makernotes_t,

    #[cfg(have_pentax_makernotes)]
    pub pentax: libraw_pentax_makernotes_t,

    #[cfg(have_ricoh_makernotes)]
    pub ricoh: libraw_ricoh_makernotes_t,

    #[cfg(have_samsung_makernotes)]
    pub samsung: libraw_samsung_makernotes_t,

    #[cfg(have_sony_info)]
    pub sony: libraw_sony_info_t,

    pub common: libraw_metadata_common_t
}

#[repr(C)]
pub struct libraw_raw_inset_crop_t {
    pub cleft: c_ushort,
    pub ctop: c_ushort,
    pub cwidth: c_ushort,
    pub cheight: c_ushort,
}

#[repr(C)]
pub struct libraw_image_sizes_t {
    pub raw_height: c_ushort,
    pub raw_width: c_ushort,
    pub height: c_ushort,
    pub width: c_ushort,
    pub top_margin: c_ushort,
    pub left_margin: c_ushort,
    pub iheight: c_ushort,
    pub iwidth: c_ushort,
    pub raw_pitch: c_uint,
    pub pixel_aspect: c_double,
    pub flip: c_int,
    pub mask: [[c_int; 4]; 8],
    pub raw_aspect: c_ushort,
    pub raw_inset_crops: [libraw_raw_inset_crop_t; 2]
}


#[repr(C)]
pub struct libraw_iparams_t {
    pub guard: [c_char; 4],
    pub make: [c_char; 64],
    pub model: [c_char; 64],
    #[cfg(have_iparams_software)]
    pub software: [c_char; 64],
    pub normalized_make: [c_char; 64],
    pub normalized_model: [c_char; 64],
    pub maker_index: c_uint,
    pub raw_count: c_uint,
    pub dng_version: c_uint,
    pub is_foveon: c_uint,
    pub colors: c_int,
    pub filters: c_uint,
    #[cfg(have_iparams_xtrans)]
    pub xtrans: [[c_char; 6]; 6],
    #[cfg(have_iparams_xtrans_abs)]
    pub xtrans_abs: [[c_char; 6]; 6],
    pub cdesc: [c_char; 5],
    #[cfg(have_iparams_xmplen)]
    pub xmplen: c_uint,
    #[cfg(have_iparams_xmpdata)]
    pub xmpdata: *mut c_char,
}

#[cfg(have_lensinfo)]
#[repr(C)]
pub struct libraw_lensinfo_t {
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
    pub EXIF_MaxAp: c_float,
    pub LensMake: [c_char; 128],
    pub Lens: [c_char; 128],
    pub LensSerial: [c_char; 128],
    pub InternalLensSerial: [c_char; 128],
    pub FocalLengthIn35mmFormat: c_ushort,
    pub nikon: libraw_nikonlens_t,
    pub dng: libraw_dnglens_t,
    pub makernotes: libraw_makernotes_lens_t,
}

#[cfg(have_nikonlens)]
#[repr(C)]
pub struct libraw_nikonlens_t {
    pub EffectiveMaxAp: c_float,
    pub LensIDNumber: c_uchar,
    pub LensFStops: c_uchar,
    pub MCUVersion: c_uchar,
    pub LensType: c_uchar,
}

#[cfg(have_dnglens)]
#[repr(C)]
pub struct libraw_dnglens_t {
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
}

#[cfg(have_makernotes_lens)]
#[repr(C)]
pub struct libraw_makernotes_lens_t {
    pub LensID: libc::c_ulonglong,
    pub Lens: [c_char; 128],
    pub LensFormat: c_ushort,
    pub LensMount: c_ushort,
    pub CamID: libc::c_ulonglong,
    pub CameraFormat: c_ushort,
    pub CameraMount: c_ushort,
    pub body: [c_char; 64],
    pub FocalType: c_short,
    pub LensFeatures_pre: [c_char; 16],
    pub LensFeatures_suf: [c_char; 16],
    pub MinFocal: c_float,
    pub MaxFocal: c_float,
    pub MaxAp4MinFocal: c_float,
    pub MaxAp4MaxFocal: c_float,
    pub MinAp4MinFocal: c_float,
    pub MinAp4MaxFocal: c_float,
    pub MaxAp: c_float,
    pub MinAp: c_float,
    pub CurFocal: c_float,
    pub CurAp: c_float,
    pub MaxAp4CurFocal: c_float,
    pub MinAp4CurFocal: c_float,
    pub MinFocusDistance: c_float,
    pub FocalRangeIndex: c_float,
    pub LensFStops: c_float,
    pub TeleconverterID: libc::c_ulonglong,
    pub Teleconverter: [c_char; 128],
    pub AdapterID: libc::c_ulonglong,
    pub Adapter: [c_char; 128],
    pub AttachmentID: libc::c_ulonglong,
    pub Attachment: [c_char; 128],
    pub CanonFocalUnits: c_short,
    pub FocalLengthIn35mmFormat: c_float,
}

#[repr(C)]
pub struct libraw_raw_unpack_params_t {
    pub use_rawspeed: c_int,
    pub use_dngsdk: c_int,
    pub options: c_uint,
    pub shot_select: c_uint,
    pub specials: c_uint,
    pub max_raw_memory_mb: c_uint,
    pub sony_arw2_posterization_thr: c_int,
    pub coolscan_nef_gamma: c_float,
    pub p4shot_order: [c_char; 5],
    // char **custom_camera_strings;
}

#[repr(C)]
pub struct libraw_output_params_t {
    pub greybox: [c_uint; 4],
    pub cropbox: [c_uint; 4],
    pub aber: [c_double; 4],
    pub gamm: [c_double; 6],
    pub user_mul: [c_float; 4],
    pub bright: c_float,
    pub threshold: c_float,
    pub half_size: c_int,
    pub four_color_rgb: c_int,
    pub highlight: c_int,
    pub use_auto_wb: c_int,
    pub use_camera_wb: c_int,
    pub use_camera_matrix: c_int,
    pub output_color: c_int,
    pub output_profile: *mut c_char,
    pub camera_profile: *mut c_char,
    pub bad_pixels: *mut c_char,
    pub dark_frame: *mut c_char,
    pub output_bps: c_int,
    pub output_tiff: c_int,
    pub output_flags: c_int,
    pub user_flip: c_int,
    pub user_qual: c_int,
    pub user_black: c_int,
    pub user_cblack: [c_int; 4],
    pub user_sat: c_int,
    pub med_passes: c_int,
    pub auto_bright_thr: c_float,
    pub adjust_maximum_thr: c_float,
    pub no_auto_bright: c_int,
    pub use_fuji_rotate: c_int,
    pub green_matching: c_int,
    pub dcb_iterations: c_int,
    pub dcb_enhance_fl: c_int,
    pub fbdd_noiserd: c_int,
    pub exp_correc: c_int,
    pub exp_shift: c_float,
    pub exp_preser: c_float,

    #[cfg(have_output_params_no_auto_scale)]
    pub no_auto_scale: c_int,
    #[cfg(have_output_params_no_interpolation)]
    pub no_interpolation: c_int,
}

#[repr(C)]
pub struct libraw_colordata_t {
    pub curve: [c_ushort; 0x10000],
    pub cblack: [c_uint; 4104],
    pub black: c_uint,
    pub data_maximum: c_uint,
    pub maximum: c_uint,
    pub linear_max: [c_long; 4],
    pub fmaximum: c_float,
    pub fnorm: c_float,
    pub white: [[c_ushort; 8]; 8],
    pub cam_mul: [c_float; 4],
    pub pre_mul: [c_float; 4],
    pub cmatrix: [[c_float; 4]; 3],
    pub ccm: [[c_float; 4]; 3],
    pub rgb_cam: [[c_float; 4]; 3],
    pub cam_xyz: [[c_float; 3]; 4],
    pub phase_one_data: ph1_t,
    pub flash_used: c_float,
    pub canon_ev: c_float,
    pub model2: [c_char; 64],
    pub UniqueCameraModel: [c_char; 64],
    pub LocalizedCameraModel: [c_char; 64],
    pub ImageUniqueID: [c_char; 64],
    pub RawDataUniqueID: [c_char; 17],
    pub OriginalRawFileName: [c_char; 64],
    pub profile: *mut c_void,
    pub profile_length: c_uint,

    #[cfg(have_colordata_black_stat)]
    pub black_stat: [c_uint; 8],
    #[cfg(have_dng_color)]
    pub dng_color: [libraw_dng_color_t; 2],

    #[cfg(have_dng_color)]
    pub dng_levels: libraw_dng_levels_t,
    pub WB_Coeffs: [[c_float; 4]; 256],
    pub WBCT_Coeffs: [[c_float; 5]; 64],
    pub as_shot_wb_applied: c_uint,
    pub P1_color: [libraw_P1_color_t; 2],
    pub raw_bps: c_uint,
    pub ExifColorSpace: c_int
}

#[cfg(have_ph1_black_off)]
#[repr(C)]
pub struct ph1_t {
    pub format: c_int,
    pub key_off: c_int,
    pub t_black: c_int,
    pub black_off: c_int,
    pub split_col: c_int,
    pub tag_21a: c_int,
    pub tag_210: c_float,
}

#[cfg(not(have_ph1_black_off))]
#[repr(C)]
pub struct ph1_t {
    pub format: c_int,
    pub key_off: c_int,
    pub tag_21a: c_int,
    pub t_black: c_int,
    pub split_col: c_int,
    pub black_col: c_int,
    pub split_row: c_int,
    pub black_row: c_int,
    pub tag_210: c_float,
}

#[cfg(have_dng_color)]
#[repr(C)]
pub struct libraw_dng_color_t {
    pub parsedfileds: c_uint,
    pub illuminant: c_ushort,
    pub calibration: [[c_float; 4]; 4],
    pub colormatrix: [[c_float; 3]; 4],
    pub forwardmatrix: [[c_float; 4]; 3],
}

#[cfg(have_dng_color)]
#[repr(C)]
pub struct libraw_dng_levels_t {
    pub parsedfields: c_uint,
    pub dng_cblack: [c_uint; 4104],
    pub dng_black: c_uint,
    pub dng_fcblack: [c_float; 4104],
    pub dng_fblack: c_float,
    pub dng_whitelevel: [c_uint; 4],
    pub default_crop: [c_ushort; 4],
    pub user_crop: [c_float; 4],
    pub preview_colorspace: c_uint,
    pub analogbalance: [c_float; 4],
    pub asshotneutral: [c_float; 4],
    pub baseline_exposure: c_float,
    pub LinearResponseLimit: c_float
}

#[repr(C)]
pub struct libraw_P1_color_t {
    pub romm_cam: [c_float; 9]
}

#[cfg(have_canon_makernotes)]
#[repr(C)]
pub struct libraw_canon_makernotes_t {
    pub ColorDataVer: c_int,
    pub ColorDataSubVer: c_int,
    pub SpecularWhiteLevel: c_int,
    pub NormalWhiteLevel: c_int,
    pub ChannelBlackLevel: [c_int; 4],
    pub AverageBlackLevel: c_int,
    pub multishot: [c_uint; 4],
    pub MeteringMode: c_short,
    pub SpotMeteringMode: c_short,
    pub FlashMeteringMode: c_uchar,
    pub FlashExposureLock: c_short,
    pub ExposureMode: c_short,
    pub AESetting: c_short,
    pub ImageStabilization: c_short,
    pub FlashMode: c_short,
    pub FlashActivity: c_short,
    pub FlashBits: c_short,
    pub ManualFlashOutput: c_short,
    pub FlashOutput: c_short,
    pub FlashGuideNumber: c_short,
    pub ContinuousDrive: c_short,
    pub SensorWidth: c_short,
    pub SensorHeight: c_short,
    pub AFMicroAdjMode: c_int,
    pub AFMicroAdjValue: c_float,
    pub MakernotesFlip: c_short,
    pub RecordMode: c_short,
    pub SRAWQuality: c_short,
    pub wbi: c_uint,
    pub RF_lensID: c_short,
    pub AutoLightingOptimizer: c_int,
    pub HighlightTonePriority: c_int,
    pub Quality: c_short,
    pub CanonLog: c_int,
    pub DefaultCropAbsolute: libraw_area_t,
    pub RecommendedImageArea: libraw_area_t,
    pub LeftOpticalBlack: libraw_area_t,
    pub UpperOpticalBlack: libraw_area_t,
    pub ActiveArea: libraw_area_t,
    pub ISOgain: [c_short; 2]
}

#[cfg(have_hasselblad_makernotes)]
#[repr(C)]
pub struct libraw_hasselblad_makernotes_t {
    pub BaseISO: c_int,
    pub Gain: c_double,
    pub Sensor: [c_char; 8],
    pub SensorUnit: [c_char; 64],
    pub HostBody: [c_char; 64],
    pub SensorCode: c_int,
    pub SensorSubCode: c_int,
    pub CoatingCode: c_int,
    pub uncropped: c_int,
    pub CaptureSequenceInitiator: [c_char; 32],
    pub SensorUnitConnector: [c_char; 64],
    pub format: c_int,
    pub nIFD_CM: [c_int; 2],
    pub RecommendedCrop: [c_int; 2],
    pub mnColorMatrix: [[c_double; 3]; 4]
}

#[cfg(have_fuji_info)]
#[repr(C)]
pub struct libraw_fuji_info_t {
    pub ExpoMidPointShift: c_float,
    pub DynamicRange: c_ushort,
    pub FilmMode: c_ushort,
    pub DynamicRangeSetting: c_ushort,
    pub DevelopmentDynamicRange: c_ushort,
    pub AutoDynamicRange: c_ushort,
    pub DRangePriority: c_ushort,
    pub DRangePriorityAuto: c_ushort,
    pub DRangePriorityFixed: c_ushort,
    pub BrightnessCompensation: c_float,
    pub FocusMode: c_ushort,
    pub AFMode: c_ushort,
    pub FocusPixel: [c_ushort; 2],
    pub PrioritySettings: c_ushort,
    pub FocusSettings: c_uint,
    pub AF_C_Settings: c_uint,
    pub FocusWarning: c_ushort,
    pub ImageStabilization: [c_ushort; 3],
    pub FlashMode: c_ushort,
    pub WB_Preset: c_ushort,
    pub ShutterType: c_ushort,
    pub ExrMode: c_ushort,
    pub Macro: c_ushort,
    pub Rating: c_uint,
    pub CropMode: c_ushort,
    pub SerialSignature: [c_char; 0x0c + 1],
    pub SensorID: [c_char; 4 + 1],
    pub RAFVersion: [c_char; 4 + 1],
    pub RAFDataGeneration: c_int,
    pub RAFDataVersion: c_ushort,
    pub isTSNERDTS: c_int,
    pub DriveMode: c_short,
    pub BlackLevel: [c_short; 9],
    pub RAFData_ImageSizeTable: [c_uint; 32],
    pub AutoBracketing: c_int,
    pub SequenceNumber: c_int,
    pub SeriesLength: c_int,
    pub PixelShiftOffset: [c_float; 2],
    pub ImageCount: c_int
}

#[cfg(have_kodak_makernotes)]
#[repr(C)]
pub struct libraw_kodak_makernotes_t {
    pub BlackLevelTop: c_ushort,
    pub BlackLevelBottom: c_ushort,
    pub offset_left: c_short,
    pub offset_top: c_short,
    pub clipBlack: c_ushort,
    pub clipWhite: c_ushort,
    pub romm_camDaylight: [[c_float; 3]; 3],
    pub romm_camTungsten: [[c_float; 3]; 3],
    pub romm_camFluorescent: [[c_float; 3]; 3],
    pub romm_camFlash: [[c_float; 3]; 3],
    pub romm_camCustom: [[c_float; 3]; 3],
    pub romm_camAuto: [[c_float; 3]; 3],
    pub  val018percent: c_ushort,
    pub val100percent: c_ushort,
    pub val170percent: c_ushort,
    pub MakerNoteKodak8a: c_short,
    pub ISOCalibrationGain: c_float,
    pub AnalogISO: c_float
}

#[cfg(have_nikon_makernotes)]
#[repr(C)]
pub struct libraw_sensor_highspeed_crop_t {
    pub cleft: c_ushort,
    pub ctop: c_ushort,
    pub cwidth: c_ushort,
    pub cheight: c_ushort
}

#[cfg(have_nikon_makernotes)]
#[repr(C)]
pub struct libraw_nikon_makernotes_t {
    pub ExposureBracketValue: c_double,
    pub ActiveDLighting: c_ushort,
    pub ShootingMode: c_ushort,
    pub ImageStabilization: [c_uchar; 7],
    pub VibrationReduction: c_uchar,
    pub VRMode: c_uchar,
    pub FlashSetting: [c_char; 13],
    pub FlashType: [c_char; 20],
    pub FlashExposureCompensation: [c_uchar; 4],
    pub ExternalFlashExposureComp: [c_uchar; 4],
    pub FlashExposureBracketValue: [c_uchar; 4],
    pub FlashMode: c_uchar,
    pub FlashExposureCompensation2: c_schar,
    pub FlashExposureCompensation3: c_schar,
    pub FlashExposureCompensation4: c_schar,
    pub FlashSource: c_uchar,
    pub FlashFirmware: [c_uchar; 2],
    pub ExternalFlashFlags: c_uchar,
    pub FlashControlCommanderMode: c_uchar,
    pub FlashOutputAndCompensation: c_uchar,
    pub FlashFocalLength: c_uchar,
    pub FlashGNDistance: c_uchar,
    pub FlashGroupControlMode: [c_uchar; 4],
    pub FlashGroupOutputAndCompensation: [c_uchar; 4],
    pub FlashColorFilter: c_uchar,
    pub NEFCompression: c_ushort,
    pub ExposureMode: c_int,
    pub ExposureProgram: c_int,
    pub nMEshots: c_int,
    pub MEgainOn: c_int,
    pub ME_WB: [c_double; 4],
    pub AFFineTune: c_uchar,
    pub AFFineTuneIndex: c_uchar,
    pub AFFineTuneAdj: c_schar,
    pub LensDataVersion: c_uint,
    pub FlashInfoVersion: c_uint,
    pub ColorBalanceVersion: c_uint,
    pub key: c_uchar,
    pub NEFBitDepth: [c_ushort; 4],
    pub HighSpeedCropFormat: c_ushort,
    pub SensorHighSpeedCrop: libraw_sensor_highspeed_crop_t,
    pub SensorWidth: c_ushort,
    pub SensorHeight: c_ushort,
    pub Active_D_Lighting: c_ushort,
    pub ShotInfoVersion: c_uint,
    pub MakernotesFlip: c_short,
    pub RollAngle: c_double,
    pub PitchAngle: c_double,
    pub YawAngle: c_double,
}

#[cfg(have_olympus_makernotes)]
#[repr(C)]
pub struct libraw_olympus_makernotes_t {
    pub CameraType2: [c_char; 6],
    pub ValidBits: c_ushort,
    pub SensorCalibration: [c_int; 2],
    pub DriveMode: [c_ushort; 5],
    pub ColorSpace: c_ushort,
    pub FocusMode: [c_ushort; 2],
    pub AutoFocus: c_ushort,
    pub AFPoint: c_ushort,
    pub AFAreas: [c_int; 64],
    pub AFPointSelected: [c_double; 5],
    pub AFResult: c_ushort,
    pub AFFineTune: c_uchar,
    pub AFFineTuneAdj: [c_short; 3],
    pub SpecialMode: [c_uint; 3],
    pub ZoomStepCount: c_ushort,
    pub FocusStepCount: c_ushort,
    pub FocusStepInfinity: c_ushort,
    pub FocusStepNear: c_ushort,
    pub FocusDistance: c_double,
    pub AspectFrame: [c_ushort; 4],
    pub StackedImage: [c_uint; 2],
    pub isLiveND: c_uchar,
    pub LiveNDfactor: c_uint,
    pub Panorama_mode: c_ushort,
    pub Panorama_frameNum: c_ushort,
}

#[cfg(have_p1_makernotes)]
#[repr(C)]
pub struct libraw_p1_makernotes_t {
    pub Software: [c_char; 64],
    pub SystemType: [c_char; 64],
    pub FirmwareString: [c_char; 256],
    pub SystemModel: [c_char; 64]
}


#[cfg(have_panasonic_makernotes)]
#[repr(C)]
pub struct libraw_panasonic_makernotes_t {
    pub Compression: c_ushort,
    pub BlackLevelDim: c_ushort,
    pub BlackLevel: [c_float; 8],
    pub Multishot: c_uint,
    pub gamma: c_float,
    pub HighISOMultiplier: [c_int; 3],
    pub FocusStepNear: c_short,
    pub FocusStepCount: c_short,
    pub ZoomPosition: c_uint,
    pub LensManufacturer: c_uint
}

#[cfg(have_pentax_makernotes)]
#[repr(C)]
pub struct libraw_pentax_makernotes_t {
    pub DriveMode: [c_uchar; 4],
    pub FocusMode: [c_ushort; 2],
    pub AFPointSelected: [c_ushort; 2],
    pub AFPointSelected_Area: c_ushort,
    pub AFPointsInFocus_version: c_int,
    pub AFPointsInFocus: c_uint,
    pub FocusPosition: c_ushort,
    pub AFAdjustment: c_short,
    pub AFPointMode: c_uchar,
    pub MultiExposure: c_uchar,
    pub Quality: c_ushort
}

#[cfg(have_ricoh_makernotes)]
#[repr(C)]
pub struct libraw_ricoh_makernotes_t {
    pub AFStatus: c_ushort,
    pub AFAreaXPosition: [c_uint; 2],
    pub AFAreaYPosition: [c_uint; 2],
    pub AFAreaMode: c_short,
    pub SensorWidth: c_uint,
    pub SensorHeight: c_uint,
    pub CroppedImageWidth: c_uint,
    pub CroppedImageHeight: c_uint,
    pub WideAdapter: c_ushort,
    pub CropMode: c_ushort,
    pub NDFilter: c_ushort,
    pub AutoBracketing: c_ushort,
    pub MacroMode: c_ushort,
    pub FlashMode: c_ushort,
    pub FlashExposureComp: c_double,
    pub ManualFlashOutput: c_double
}

#[cfg(have_samsung_makernotes)]
#[repr(C)]
pub struct libraw_samsung_makernotes_t {
    pub ImageSizeFull: [c_uint; 4],
    pub ImageSizeCrop: [c_uint; 4],
    pub ColorSpace: [c_int; 2],
    pub key: [c_uint; 11],
    pub DigitalGain: c_double,
    pub DeviceType: c_int,
    pub LensFirmware: [c_char; 32]
}

#[cfg(have_sony_info)]
#[repr(C)]
pub struct libraw_sony_info_t {
    pub CameraType: c_ushort,
    pub Sony0x9400_version: c_uchar,
    pub Sony0x9400_ReleaseMode2: c_uchar,
    pub Sony0x9400_SequenceImageNumber: c_uint,
    pub Sony0x9400_SequenceLength1: c_uchar,
    pub Sony0x9400_SequenceFileNumber: c_uint,
    pub Sony0x9400_SequenceLength2: c_uchar,
    pub AFAreaModeSetting: u8,
    pub AFAreaMode: u16,
    pub FlexibleSpotPosition: [c_ushort; 2],
    pub AFPointSelected: u8,
    pub AFPointSelected_0x201e: u8,
    pub nAFPointsUsed: c_short,
    pub AFPointsUsed: [u8; 10],
    pub AFTracking: u8,
    pub AFType: u8,
    pub FocusLocation: [c_ushort; 4],
    pub FocusPosition: c_ushort,
    pub AFMicroAdjValue: i8,
    pub AFMicroAdjOn: i8,
    pub AFMicroAdjRegisteredLenses: c_uchar,
    pub VariableLowPassFilter: c_ushort,
    pub LongExposureNoiseReduction: c_uint,
    pub HighISONoiseReduction: c_ushort,
    pub HDR: [c_ushort; 2],
    pub group2010: c_ushort,
    pub group9050: c_ushort,
    pub real_iso_offset: c_ushort,
    pub MeteringMode_offset: c_ushort,
    pub ExposureProgram_offset: c_ushort,
    pub ReleaseMode2_offset: c_ushort,
    pub MinoltaCamID: c_uint,
    pub firmware: c_float,
    pub ImageCount3_offset: c_ushort,
    pub ImageCount3: c_uint,
    pub ElectronicFrontCurtainShutter: c_uint,
    pub MeteringMode2: c_ushort,
    pub SonyDateTime: [c_char; 20],
    pub ShotNumberSincePowerUp: c_uint,
    pub PixelShiftGroupPrefix: c_ushort,
    pub PixelShiftGroupID: c_uint,
    pub nShotsInPixelShiftGroup: c_char,
    pub numInPixelShiftGroup: c_char,
    pub prd_ImageHeight: c_ushort,
    pub prd_ImageWidth: c_ushort,
    pub prd_Total_bps: c_ushort,
    pub prd_Active_bps: c_ushort,
    pub prd_StorageMethod: c_ushort,
    pub prd_BayerPattern: c_ushort,
    pub SonyRawFileType: c_ushort,
    pub RAWFileType: c_ushort,
    pub RawSizeType: c_ushort,
    pub Quality: c_uint,
    pub FileFormat: c_ushort,
    pub MetaVersion: [c_char; 16]
}

#[repr(C)]
pub struct libraw_afinfo_item_t {
    pub AFInfoData_tag: c_uint,
    pub AFInfoData_order: c_short,
    pub AFInfoData_version: c_uint,
    pub AFInfoData_length: c_uint,
    pub AFInfoData: *mut c_uchar
}

#[repr(C)]
pub struct libraw_metadata_common_t {
    pub FlashEC: c_float,
    pub FlashGN: c_float,
    pub CameraTemperature: c_float,
    pub SensorTemperature: c_float,
    pub SensorTemperature2: c_float,
    pub LensTemperature: c_float,
    pub AmbientTemperature: c_float,
    pub BatteryTemperature: c_float,
    pub exifAmbientTemperature: c_float,
    pub exifHumidity: c_float,
    pub exifPressure: c_float,
    pub exifWaterDepth: c_float,
    pub exifAcceleration: c_float,
    pub exifCameraElevationAngle: c_float,
    pub real_ISO: c_float,
    pub exifExposureIndex: c_float,
    pub ColorSpace: c_ushort,
    pub firmware: [c_char; 128],
    pub ExposureCalibrationShift: c_float,
    pub afdata: [libraw_afinfo_item_t; 4],
    pub afcount: c_int
}

#[repr(C)]
pub struct libraw_imgother_t {
    pub iso_speed: c_float,
    pub shutter: c_float,
    pub aperture: c_float,
    pub focal_len: c_float,
    pub timestamp: time_t,
    pub shot_order: c_uint,
    pub gpsdata: [c_uint; 32],
    #[cfg(have_gps_info)]
    pub parsed_gps: libraw_gps_info_t,
    pub desc: [c_char; 512],
    pub artist: [c_char; 64],
    pub analogbalance: [c_float; 4]
}

#[cfg(have_gps_info)]
#[repr(C)]
pub struct libraw_gps_info_t {
    pub latitude: [c_float; 3],
    pub longtitude: [c_float; 3],
    pub gpstimestamp: [c_float; 3],
    pub altitude: c_float,
    pub altref: c_char,
    pub latref: c_char,
    pub longref: c_char,
    pub gpsstatus: c_char,
    pub gpsparsed: c_char,
}

#[repr(C)]
pub struct libraw_thumbnail_t {
    pub tformat: LibRaw_thumbnail_formats,
    pub twidth: c_ushort,
    pub theight: c_ushort,
    pub tlength: c_uint,
    pub tcolors: c_int,
    pub thumb: *mut c_char,
}

#[repr(C)]
pub struct libraw_thumbnail_list_t {
    pub thumbcount: c_int,
    pub thumblist: [libraw_thumbnail_item_t; 8]
}

#[repr(C)]
pub struct libraw_thumbnail_item_t {
    pub tformat: LibRaw_internal_thumbnail_formats,
    pub twidth: c_ushort,
    pub theight: c_ushort,
    pub tflip: c_ushort,
    pub tlength: c_uint,
    pub tmisc: c_uint,
    pub toffset: i64
}

#[repr(C)]
pub struct libraw_rawdata_t {
    pub raw_alloc: *mut c_void,
    pub raw_image: *mut c_ushort,
    pub color4_image: *mut [c_ushort; 4],
    pub color3_image: *mut [c_ushort; 3],
    pub float_image: *mut c_float,
    pub float3_image: *mut [c_float; 3],
    pub float4_image: *mut [c_float; 4],
    pub ph1_cblack: *mut [c_short; 2],
    #[cfg(have_rawdata_ph1_rblack)]
    pub ph1_rblack: *mut [c_short; 2],
    pub iparams: libraw_iparams_t,
    pub sizes: libraw_image_sizes_t,
    pub ioparams: libraw_internal_output_params_t,
    pub color: libraw_colordata_t,
}

#[repr(C)]
pub struct libraw_internal_output_params_t {
    pub mix_green: c_uint,
    pub raw_color: c_uint,
    pub zero_is_bad: c_uint,
    pub shrink: c_ushort,
    pub fuji_width: c_ushort,
}

#[repr(C)]
pub struct libraw_processed_image_t {
    pub image_type: LibRaw_image_formats,
    pub height: c_ushort,
    pub width: c_ushort,
    pub colors: c_ushort,
    pub bits: c_ushort,
    pub data_size: c_uint,
    pub data: [c_uchar; 1],
}

#[repr(C)]
pub struct libraw_decoder_info_t {
    pub decoder_name: *const c_char,
    pub decoder_flags: c_uint,
}

pub type LibRaw_constructor_flags = c_uint;
pub const LIBRAW_OPTIONS_NONE:               LibRaw_constructor_flags = 0;
pub const LIBRAW_OPIONS_NO_MEMERR_CALLBACK:  LibRaw_constructor_flags = 1;
pub const LIBRAW_OPIONS_NO_DATAERR_CALLBACK: LibRaw_constructor_flags = 1<<1;

pub type LibRaw_errors = c_int;
pub const LIBRAW_SUCCESS:                               LibRaw_errors = 0;
pub const LIBRAW_UNSPECIFIED_ERROR:                     LibRaw_errors = -1;
pub const LIBRAW_FILE_UNSUPPORTED:                      LibRaw_errors = -2;
pub const LIBRAW_REQUEST_FOR_NONEXISTENT_IMAGE:         LibRaw_errors = -3;
pub const LIBRAW_OUT_OF_ORDER_CALL:                     LibRaw_errors = -4;
pub const LIBRAW_NO_THUMBNAIL:                          LibRaw_errors = -5;
pub const LIBRAW_UNSUPPORTED_THUMBNAIL:                 LibRaw_errors = -6;
pub const LIBRAW_INPUT_CLOSED:                          LibRaw_errors = -7;
pub const LIBRAW_NOT_IMPLEMENTED:                       LibRaw_errors = -8;
pub const LIBRAW_REQUEST_FOR_NONEXISTENT_THUMBNAIL:     LibRaw_errors = -9;
pub const LIBRAW_INSUFFICIENT_MEMORY:                   LibRaw_errors = -100007;
pub const LIBRAW_DATA_ERROR:                            LibRaw_errors = -100008;
pub const LIBRAW_IO_ERROR:                              LibRaw_errors = -100009;
pub const LIBRAW_CANCELLED_BY_CALLBACK:                 LibRaw_errors = -100010;
pub const LIBRAW_BAD_CROP:                              LibRaw_errors = -100011;
pub const LIBRAW_TOO_BIG:                               LibRaw_errors = -100012;
pub const LIBRAW_MEMPOOL_OVERFLOW:                      LibRaw_errors = -100013;

pub type LibRaw_progress = c_int;
pub const LIBRAW_PROGRESS_START:              LibRaw_progress = 0;
pub const LIBRAW_PROGRESS_OPEN:               LibRaw_progress = 1<<0;
pub const LIBRAW_PROGRESS_IDENTIFY:           LibRaw_progress = 1<<1;
pub const LIBRAW_PROGRESS_SIZE_ADJUST:        LibRaw_progress = 1<<2;
pub const LIBRAW_PROGRESS_LOAD_RAW:           LibRaw_progress = 1<<3;
pub const LIBRAW_PROGRESS_RAW2_IMAGE:         LibRaw_progress = 1<<4;
pub const LIBRAW_PROGRESS_REMOVE_ZEROES:      LibRaw_progress = 1<<5;
pub const LIBRAW_PROGRESS_BAD_PIXELS:         LibRaw_progress = 1<<6;
pub const LIBRAW_PROGRESS_DARK_FRAME:         LibRaw_progress = 1<<7;
pub const LIBRAW_PROGRESS_FOVEON_INTERPOLATE: LibRaw_progress = 1<<8;
pub const LIBRAW_PROGRESS_SCALE_COLORS:       LibRaw_progress = 1<<9;
pub const LIBRAW_PROGRESS_PRE_INTERPOLATE:    LibRaw_progress = 1<<10;
pub const LIBRAW_PROGRESS_INTERPOLATE:        LibRaw_progress = 1<<11;
pub const LIBRAW_PROGRESS_MIX_GREEN:          LibRaw_progress = 1<<12;
pub const LIBRAW_PROGRESS_MEDIAN_FILTER:      LibRaw_progress = 1<<13;
pub const LIBRAW_PROGRESS_HIGHLIGHTS:         LibRaw_progress = 1<<14;
pub const LIBRAW_PROGRESS_FUJI_ROTATE:        LibRaw_progress = 1<<15;
pub const LIBRAW_PROGRESS_FLIP:               LibRaw_progress = 1<<16;
pub const LIBRAW_PROGRESS_APPLY_PROFILE:      LibRaw_progress = 1<<17;
pub const LIBRAW_PROGRESS_CONVERT_RGB:        LibRaw_progress = 1<<18;
pub const LIBRAW_PROGRESS_STRETCH:            LibRaw_progress = 1<<19;
pub const LIBRAW_PROGRESS_STAGE20:            LibRaw_progress = 1<<20;
pub const LIBRAW_PROGRESS_STAGE21:            LibRaw_progress = 1<<21;
pub const LIBRAW_PROGRESS_STAGE22:            LibRaw_progress = 1<<22;
pub const LIBRAW_PROGRESS_STAGE23:            LibRaw_progress = 1<<23;
pub const LIBRAW_PROGRESS_STAGE24:            LibRaw_progress = 1<<24;
pub const LIBRAW_PROGRESS_STAGE25:            LibRaw_progress = 1<<25;
pub const LIBRAW_PROGRESS_STAGE26:            LibRaw_progress = 1<<26;
pub const LIBRAW_PROGRESS_STAGE27:            LibRaw_progress = 1<<27;
pub const LIBRAW_PROGRESS_THUMB_LOAD:         LibRaw_progress = 1<<28;
pub const LIBRAW_PROGRESS_TRESERVED1:         LibRaw_progress = 1<<29;
pub const LIBRAW_PROGRESS_TRESERVED2:         LibRaw_progress = 1<<30;
pub const LIBRAW_PROGRESS_TRESERVED3:         LibRaw_progress = 1<<31;
pub const LIBRAW_PROGRESS_THUMB_MASK:         LibRaw_progress = 0x0fffffff;

pub type LibRaw_decoder_flags = c_uint;
pub const LIBRAW_DECODER_USEBAYER2:                 LibRaw_decoder_flags = 1<<3;
pub const LIBRAW_DECODER_HASCURVE:                  LibRaw_decoder_flags = 1<<4;
pub const LIBRAW_DECODER_SONYARW2:                  LibRaw_decoder_flags = 1<<5;
pub const LIBRAW_DECODER_TRYRAWSPEED:               LibRaw_decoder_flags = 1<<6;
pub const LIBRAW_DECODER_OWNALLOC:                  LibRaw_decoder_flags = 1<<7;
pub const LIBRAW_DECODER_FIXEDMAXC:                 LibRaw_decoder_flags = 1<<8;
pub const LIBRAW_DECODER_ADOBECOPYPIXEL:            LibRaw_decoder_flags = 1<<9;
pub const LIBRAW_DECODER_LEGACY_WITH_MARGINS:       LibRaw_decoder_flags = 1<<10;
pub const LIBRAW_DECODER_3CHANNE:                   LibRaw_decoder_flags = 1<<11;
pub const LIBRAW_DECODER_SINAR4SHOT:                LibRaw_decoder_flags = 1<<11;
pub const LIBRAW_DECODER_FLATDATA:                  LibRaw_decoder_flags = 1<<12;
pub const LIBRAW_DECODER_FLAT_BG2_SWAPPED:          LibRaw_decoder_flags = 1<<13;
pub const LIBRAW_DECODER_UNSUPPORTED_FORMAT:        LibRaw_decoder_flags = 1<<14;
pub const LIBRAW_DECODER_NOTSET:                    LibRaw_decoder_flags = 1<<15;
pub const LIBRAW_DECODER_TRYRAWSPEED3:              LibRaw_decoder_flags = 1<<16;

#[repr(C)]
enum LibRaw_internal_thumbnail_formats
{
    LIBRAW_INTERNAL_THUMBNAIL_UNKNOWN       = 0,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_THUMB   = 1,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_YCBCR   = 2,
    LIBRAW_INTERNAL_THUMBNAIL_KODAK_RGB     = 3,
    LIBRAW_INTERNAL_THUMBNAIL_JPEG          = 4,
    LIBRAW_INTERNAL_THUMBNAIL_LAYER,
    LIBRAW_INTERNAL_THUMBNAIL_ROLLEI,
    LIBRAW_INTERNAL_THUMBNAIL_PPM,
    LIBRAW_INTERNAL_THUMBNAIL_PPM16,
    LIBRAW_INTERNAL_THUMBNAIL_X3F,
}

#[repr(C)]
pub enum LibRaw_thumbnail_formats {
    LIBRAW_THUMBNAIL_UNKNOWN    = 0,
    LIBRAW_THUMBNAIL_JPEG       = 1,
    LIBRAW_THUMBNAIL_BITMAP     = 2,
    LIBRAW_THUMBNAIL_BITMAP16   = 3,
    LIBRAW_THUMBNAIL_LAYER      = 4,
    LIBRAW_THUMBNAIL_ROLLEI     = 5,
    LIBRAW_THUMBNAIL_H265       = 6
}

pub const LIBRAW_THUMBNAIL_UNKNOWN: LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_UNKNOWN;
pub const LIBRAW_THUMBNAIL_JPEG:    LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_JPEG;
pub const LIBRAW_THUMBNAIL_BITMAP:  LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_BITMAP;
pub const LIBRAW_THUMBNAIL_LAYER:   LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_LAYER;
pub const LIBRAW_THUMBNAIL_ROLLEI:  LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_ROLLEI;
pub const LIBRAW_THUMBNAIL_H265:    LibRaw_thumbnail_formats = LibRaw_thumbnail_formats::LIBRAW_THUMBNAIL_H265;

#[repr(C)]
pub enum LibRaw_image_formats {
    LIBRAW_IMAGE_JPEG   = 1,
    LIBRAW_IMAGE_BITMAP = 2,
}

pub const LIBRAW_IMAGE_JPEG:   LibRaw_image_formats = LibRaw_image_formats::LIBRAW_IMAGE_JPEG;
pub const LIBRAW_IMAGE_BITMAP: LibRaw_image_formats = LibRaw_image_formats::LIBRAW_IMAGE_BITMAP;

pub type memory_callback      = extern "C" fn (data: *mut c_void, file: *const c_char, func: *const c_char);
pub type exif_parser_callback = extern "C" fn (data: *mut c_void, tag: c_int, tag_type: c_int, len: c_int, ord: c_uint, ifp: *mut c_void);
pub type data_callback        = extern "C" fn (data: *mut c_void, file: *const c_char, offset: c_int);
pub type progress_callback    = extern "C" fn (data: *mut c_void, stage: LibRaw_progress, iteration: c_int, expected: c_int) -> c_int;

extern "C" {
    pub fn libraw_version() -> *const c_char;
    pub fn libraw_versionNumber() -> c_int;

    pub fn libraw_cameraList() -> *const *const c_char;
    pub fn libraw_cameraCount() -> c_int;

    pub fn libraw_strprogress(progress: LibRaw_progress) -> *const c_char;
    pub fn libraw_strerror(err: c_int) -> *const c_char;

    pub fn libraw_init(flags: c_uint) -> *mut libraw_data_t;
    pub fn libraw_close(lr: *mut libraw_data_t);

    pub fn libraw_unpack_function_name(lr: *mut libraw_data_t) -> *const c_char;
    pub fn libraw_subtract_black(lr: *mut libraw_data_t);

    pub fn libraw_open_file(lr: *mut libraw_data_t, file: *const c_char) -> c_int;
    pub fn libraw_open_file_ex(lr: *mut libraw_data_t, file: *const c_char, sz: i64) -> c_int;

    pub fn libraw_open_buffer(lr: *mut libraw_data_t, buffer: *mut c_void, size: size_t) -> c_int;
    pub fn libraw_unpack(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_unpack_thumb(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_recycle_datastream(lr: *mut libraw_data_t);
    pub fn libraw_recycle(lr: *mut libraw_data_t);

    pub fn libraw_set_exifparser_handler(lr: *mut libraw_data_t, cb: exif_parser_callback, data: *mut c_void);
    pub fn libraw_set_memerror_handler(lr: *mut libraw_data_t, cb: memory_callback, data: *mut c_void);
    pub fn libraw_set_dataerror_handler(lr: *mut libraw_data_t, cb: data_callback, data: *mut c_void);
    pub fn libraw_set_progress_handler(lr: *mut libraw_data_t, cb: progress_callback, data: *mut c_void);

    pub fn libraw_adjust_sizes_info_only(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_dcraw_ppm_tiff_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;
    pub fn libraw_dcraw_thumb_writer(lr: *mut libraw_data_t, filename: *const c_char) -> c_int;
    pub fn libraw_dcraw_process(lr: *mut libraw_data_t) -> c_int;

    pub fn libraw_dcraw_make_mem_image(lr: *mut libraw_data_t, errc: *mut c_int) -> *mut libraw_processed_image_t;
    pub fn libraw_dcraw_make_mem_thumb(lr: *mut libraw_data_t, errc: *mut c_int) -> *mut libraw_processed_image_t;
    pub fn libraw_dcraw_clear_mem(p: *mut libraw_processed_image_t);

    pub fn libraw_raw2image(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_free_image(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_decoder_info(lr: *mut libraw_data_t, d: *mut libraw_decoder_info_t) -> c_int;

    pub fn libraw_COLOR(lr: *mut libraw_data_t, row: c_int, col: c_int) -> c_int;

    pub fn libraw_set_demosaic(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_color(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_output_bps(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_gamma(lr: *mut libraw_data_t, index: c_int, value: c_float);
    pub fn libraw_set_no_auto_bright(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_bright(lr: *mut libraw_data_t, value: c_float);
    pub fn libraw_set_highlight(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_set_fbdd_noiserd(lr: *mut libraw_data_t, value: c_int);
    pub fn libraw_get_raw_height(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_raw_width(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iheight(lr: *mut libraw_data_t) -> c_int;
    pub fn libraw_get_iwidth(lr: *mut libraw_data_t) -> c_int;

    pub fn libraw_get_cam_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_pre_mul(lr: *mut libraw_data_t, index: c_int) -> c_float;
    pub fn libraw_get_rgb_cam(lr: *mut libraw_data_t, index1: c_int, index2: c_int) -> c_float;
    pub fn libraw_get_color_maximum(lr: *mut libraw_data_t) -> c_int;
}
