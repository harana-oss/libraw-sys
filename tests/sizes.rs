extern crate libraw_sys as libraw;

use std::mem;

#[link(name = "sizes")]
mod sizes {
    extern crate libc;

    use self::libc::size_t;

    extern "C" {
        pub fn size_of_libraw_data_t() -> size_t;
        pub fn size_of_libraw_image_sizes_t() -> size_t;
        pub fn size_of_libraw_iparams_t() -> size_t;

        #[cfg(have_lensinfo)]
        pub fn size_of_libraw_lensinfo_t() -> size_t;
        #[cfg(have_nikonlens)]
        pub fn size_of_libraw_nikonlens_t() -> size_t;
        #[cfg(have_dnglens)]
        pub fn size_of_libraw_dnglens_t() -> size_t;
        #[cfg(have_makernotes_lens)]
        pub fn size_of_libraw_makernotes_lens_t() -> size_t;

        pub fn size_of_libraw_output_params_t() -> size_t;
        pub fn size_of_libraw_colordata_t() -> size_t;

        #[cfg(have_ph1)]
        pub fn size_of_ph1_t() -> size_t;
        #[cfg(have_dng_color)]
        pub fn size_of_libraw_dng_color_t() -> size_t;

        #[cfg(have_canon_makernotes)]
        pub fn size_of_libraw_canon_makernotes_t() -> size_t;

        #[cfg(have_fuji_info)]
        pub fn size_of_libraw_fuji_info_t() -> size_t;

        #[cfg(have_hasselblad_makernotes)]
        pub fn size_of_libraw_hasselblad_makernotes_t() -> size_t;

        #[cfg(have_kodak_makernotes)]
        pub fn size_of_libraw_kodak_makernotes_t() -> size_t;

        #[cfg(have_nikon_makernotes)]
        pub fn size_of_libraw_nikon_makernotes_t() -> size_t;

        #[cfg(have_olympus_makernotes)]
        pub fn size_of_libraw_olympus_makernotes_t() -> size_t;

        #[cfg(have_p1_makernotes)]
        pub fn size_of_libraw_p1_makernotes_t() -> size_t;

        #[cfg(have_panasonic_makernotes)]
        pub fn size_of_libraw_panasonic_makernotes_t() -> size_t;

        #[cfg(have_pentax_makernotes)]
        pub fn size_of_libraw_pentax_makernotes_t() -> size_t;

        #[cfg(have_ricoh_makernotes)]
        pub fn size_of_libraw_ricoh_makernotes_t() -> size_t;

        #[cfg(have_samsung_makernotes)]
        pub fn size_of_libraw_samsung_makernotes_t() -> size_t;

        #[cfg(have_sony_info)]
        pub fn size_of_libraw_sony_info_t() -> size_t;

        pub fn size_of_libraw_imgother_t() -> size_t;

        #[cfg(have_gps_info)]
        pub fn size_of_libraw_gps_info_t() -> size_t;

        pub fn size_of_libraw_thumbnail_t() -> size_t;
        pub fn size_of_libraw_rawdata_t() -> size_t;
        pub fn size_of_libraw_internal_output_params_t() -> size_t;
        pub fn size_of_libraw_processed_image_t() -> size_t;
        pub fn size_of_libraw_decoder_info_t() -> size_t;
    }
}

#[test]
fn it_should_have_same_size_for_libraw_data_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_data_t() as usize, mem::size_of::<libraw::libraw_data_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_image_sizes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_image_sizes_t() as usize, mem::size_of::<libraw::libraw_image_sizes_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_output_params_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_output_params_t() as usize, mem::size_of::<libraw::libraw_output_params_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_iparams_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_iparams_t() as usize, mem::size_of::<libraw::libraw_iparams_t>());
    }
}

#[cfg(have_lensinfo)]
#[test]
fn it_should_have_same_size_for_libraw_lensinfo_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_lensinfo_t() as usize, mem::size_of::<libraw::libraw_lensinfo_t>());
    }
}

#[cfg(have_nikonlens)]
#[test]
fn it_should_have_same_size_for_libraw_nikonlens_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_nikonlens_t() as usize, mem::size_of::<libraw::libraw_nikonlens_t>());
    }
}

#[cfg(have_dnglens)]
#[test]
fn it_should_have_same_size_for_libraw_dnglens_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_dnglens_t() as usize, mem::size_of::<libraw::libraw_dnglens_t>());
    }
}

#[cfg(have_makernotes_lens)]
#[test]
fn it_should_have_same_size_for_libraw_makernotes_lens_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_makernotes_lens_t() as usize, mem::size_of::<libraw::libraw_makernotes_lens_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_colordata_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_colordata_t() as usize, mem::size_of::<libraw::libraw_colordata_t>());
    }
}

#[cfg(have_ph1)]
#[test]
fn it_should_have_same_size_for_ph1_t() {
    unsafe {
        assert_eq!(sizes::size_of_ph1_t() as usize, mem::size_of::<libraw::ph1_t>());
    }
}

#[cfg(have_dng_color)]
#[test]
fn it_should_have_same_size_for_libraw_dng_color_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_dng_color_t() as usize, mem::size_of::<libraw::libraw_dng_color_t>());
    }
}

#[cfg(have_canon_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_canon_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_canon_makernotes_t() as usize, mem::size_of::<libraw::libraw_canon_makernotes_t>());
    }
}

#[cfg(have_fuji_info)]
#[test]
fn it_should_have_same_size_for_libraw_fuji_info_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_fuji_info_t() as usize, mem::size_of::<libraw::libraw_fuji_info_t>());
    }
}

#[cfg(have_hasselblad_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_hasselblad_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_hasselblad_makernotes_t() as usize, mem::size_of::<libraw::libraw_hasselblad_makernotes_t>());
    }
}

#[cfg(have_kodak_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_kodak_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_kodak_makernotes_t() as usize, mem::size_of::<libraw::libraw_kodak_makernotes_t>());
    }
}

#[cfg(have_nikon_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_nikon_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_nikon_makernotes_t() as usize, mem::size_of::<libraw::libraw_nikon_makernotes_t>());
    }
}

#[cfg(have_olympus_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_olympus_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_olympus_makernotes_t() as usize, mem::size_of::<libraw::libraw_olympus_makernotes_t>());
    }
}

#[cfg(have_p1_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_p1_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_p1_makernotes_t() as usize, mem::size_of::<libraw::libraw_p1_makernotes_t>());
    }
}

#[cfg(have_panasonic_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_panasonic_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_panasonic_makernotes_t() as usize, mem::size_of::<libraw::libraw_panasonic_makernotes_t>());
    }
}

#[cfg(have_pentax_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_pentax_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_pentax_makernotes_t() as usize, mem::size_of::<libraw::libraw_pentax_makernotes_t>());
    }
}

#[cfg(have_ricoh_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_ricoh_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_ricoh_makernotes_t() as usize, mem::size_of::<libraw::libraw_ricoh_makernotes_t>());
    }
}

#[cfg(have_samsung_makernotes)]
#[test]
fn it_should_have_same_size_for_libraw_samsung_makernotes_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_samsung_makernotes_t() as usize, mem::size_of::<libraw::libraw_samsung_makernotes_t>());
    }
}

#[cfg(have_sony_info)]
#[test]
fn it_should_have_same_size_for_libraw_sony_info_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_sony_info_t() as usize, mem::size_of::<libraw::libraw_sony_info_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_imgother_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_imgother_t() as usize, mem::size_of::<libraw::libraw_imgother_t>());
    }
}

#[cfg(have_gps_info)]
#[test]
fn it_should_have_same_size_for_libraw_gps_info_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_gps_info_t() as usize, mem::size_of::<libraw::libraw_gps_info_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_thumbnail_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_thumbnail_t() as usize, mem::size_of::<libraw::libraw_thumbnail_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_rawdata_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_rawdata_t() as usize, mem::size_of::<libraw::libraw_rawdata_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_internal_output_params_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_internal_output_params_t() as usize, mem::size_of::<libraw::libraw_internal_output_params_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_processed_image_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_processed_image_t() as usize, mem::size_of::<libraw::libraw_processed_image_t>());
    }
}

#[test]
fn it_should_have_same_size_for_libraw_decoder_info_t() {
    unsafe {
        assert_eq!(sizes::size_of_libraw_decoder_info_t() as usize, mem::size_of::<libraw::libraw_decoder_info_t>());
    }
}
