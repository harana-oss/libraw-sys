extern crate cc;
extern crate conan2;
extern crate home;
extern crate pkg_config;
extern crate walkdir;

use conan2::ConanInstall;
use std::fs::{self,File};
use std::path::Path;

use std::io::prelude::*;
use walkdir::WalkDir;

fn main() {

    ConanInstall::new().build("missing").run().parse().emit();

    let conan_dir = home::home_dir().unwrap().join(".conan2");
    let build_paths = WalkDir::new(conan_dir.to_str().unwrap()).max_depth(10).into_iter()
        .filter_map(|e| e.ok())
        .filter(|p| p.path().to_str().unwrap().ends_with("include/libraw"))
        .collect::<Vec<_>>();

    let libraw_path = build_paths.first().unwrap().path();

    // libraw_iparams_t members
    if check_member(libraw_path, "libraw_iparams_t", "software") {
        println!("cargo:rustc-cfg=have_iparams_software");
    }

    if check_member(libraw_path, "libraw_iparams_t", "xtrans") {
        println!("cargo:rustc-cfg=have_iparams_xtrans");
    }

    if check_member(libraw_path, "libraw_iparams_t", "xtrans_abs") {
        println!("cargo:rustc-cfg=have_iparams_xtrans_abs");
    }

    if check_member(libraw_path, "libraw_iparams_t", "xmplen") {
        println!("cargo:rustc-cfg=have_iparams_xmplen");
    }

    if check_member(libraw_path, "libraw_iparams_t", "xmpdata") {
        println!("cargo:rustc-cfg=have_iparams_xmpdata");
    }

    // libraw_output_params_t members

    if check_member(libraw_path, "libraw_output_params_t", "sony_arw2_hack") {
        println!("cargo:rustc-cfg=have_output_params_sony_arw2_hack");
    }

    if check_member(libraw_path, "libraw_output_params_t", "afd_noise_att") {
        println!("cargo:rustc-cfg=have_output_params_afd_noise_att");
    }

    if check_member(libraw_path, "libraw_output_params_t", "afd_noise_thres") {
        println!("cargo:rustc-cfg=have_output_params_afd_noise_thres");
    }

    if check_member(libraw_path, "libraw_output_params_t", "afd_luminance_passes") {
        println!("cargo:rustc-cfg=have_output_params_afd_luminance_passes");
    }

    if check_member(libraw_path, "libraw_output_params_t", "afd_chrominance_method") {
        println!("cargo:rustc-cfg=have_output_params_afd_chrominance_method");
    }

    if check_member(libraw_path, "libraw_output_params_t", "afd_luminance_only") {
        println!("cargo:rustc-cfg=have_output_params_afd_luminance_only");
    }

    if check_member(libraw_path, "libraw_output_params_t", "no_auto_scale") {
        println!("cargo:rustc-cfg=have_output_params_no_auto_scale");
    }

    if check_member(libraw_path, "libraw_output_params_t", "no_interpolation") {
        println!("cargo:rustc-cfg=have_output_params_no_interpolation");
    }

    if check_member(libraw_path, "libraw_output_params_t", "sraw_ycc") {
        println!("cargo:rustc-cfg=have_output_params_sraw_ycc");
    }

    if check_member(libraw_path, "libraw_output_params_t", "force_foveon_x3f") {
        println!("cargo:rustc-cfg=have_output_params_force_foveon_x3f");
    }

    if check_member(libraw_path, "libraw_output_params_t", "x3f_flags") {
        println!("cargo:rustc-cfg=have_output_params_x3f_flags");
    }

    if check_member(libraw_path, "libraw_output_params_t", "sony_arw2_options") {
        println!("cargo:rustc-cfg=have_output_params_sony_arw2_options");
    }

    if check_member(libraw_path, "libraw_output_params_t", "sony_arw2_posterization_thr") {
        println!("cargo:rustc-cfg=have_output_params_sony_arw2_posterization_thr");
    }

    if check_member(libraw_path, "libraw_output_params_t", "coolscan_nef_gamma") {
        println!("cargo:rustc-cfg=have_output_params_coolscan_nef_gamma");
    }

    // libraw_colordata_t members

    if check_member(libraw_path, "libraw_colordata_t", "black_stat") {
        println!("cargo:rustc-cfg=have_colordata_black_stat");
    }

    if check_member(libraw_path, "libraw_colordata_t", "baseline_exposure") {
        println!("cargo:rustc-cfg=have_colordata_baseline_exposure");
    }

    if check_member(libraw_path, "libraw_colordata_t", "OlympusSensorCalibration") {
        println!("cargo:rustc-cfg=have_colordata_olympus_sensor_calibration");
    }

    if check_member(libraw_path, "libraw_colordata_t", "FujiExpoMidPointShift") {
        println!("cargo:rustc-cfg=have_colordata_fuji_expo_mid_point_shift");
    }

    if check_member(libraw_path, "libraw_colordata_t", "digitalBack_color") {
        println!("cargo:rustc-cfg=have_colordata_digital_back_color");
    }

    // libraw_rawdata_t members

    if check_member(libraw_path, "libraw_rawdata_t", "ph1_rblack") {
        println!("cargo:rustc-cfg=have_rawdata_ph1_rblack");
    }

    // ph1_t layout

    if check_member(libraw_path, "struct ph1_t", "black_off") {
        println!("cargo:rustc-cfg=have_ph1_black_off");
    }

    // structs

    let mut defines = Vec::<&str>::with_capacity(16);

    if check_type(libraw_path, "libraw_lensinfo_t") {
        println!("cargo:rustc-cfg=have_lensinfo");
        defines.push("HAVE_LENSINFO");
    }

    if check_type(libraw_path, "libraw_nikonlens_t") {
        println!("cargo:rustc-cfg=have_nikonlens");
        defines.push("HAVE_NIKONLENS");
    }

    if check_type(libraw_path, "libraw_dnglens_t") {
        println!("cargo:rustc-cfg=have_dnglens");
        defines.push("HAVE_DNGLENS");
    }

    if check_type(libraw_path, "libraw_makernotes_lens_t") {
        println!("cargo:rustc-cfg=have_makernotes_lens");
        defines.push("HAVE_MAKERNOTES_LENS");
    }

    if check_type(libraw_path, "libraw_ph1_t") {
        println!("cargo:rustc-cfg=have_ph1");
        defines.push("HAVE_PH1");
    }

    if check_type(libraw_path, "libraw_dng_color_t") {
        println!("cargo:rustc-cfg=have_dng_color");
        defines.push("HAVE_DNG_COLOR");
    }

    if check_type(libraw_path, "libraw_canon_makernotes_t") {
        println!("cargo:rustc-cfg=have_canon_makernotes");
        defines.push("HAVE_CANON_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_fuji_info_t") {
        println!("cargo:rustc-cfg=have_fuji_info");
        defines.push("HAVE_FUJI_INFO");
    }

    if check_type(libraw_path, "libraw_hasselblad_makernotes_t") {
        println!("cargo:rustc-cfg=have_hasselblad_makernotes");
        defines.push("HAVE_HASSELBLAD_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_kodak_makernotes_t") {
        println!("cargo:rustc-cfg=have_kodak_makernotes");
        defines.push("HAVE_KODAK_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_nikon_makernotes_t") {
        println!("cargo:rustc-cfg=have_nikon_makernotes");
        defines.push("HAVE_NIKON_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_olympus_makernotes_t") {
        println!("cargo:rustc-cfg=have_olympus_makernotes");
        defines.push("HAVE_OLYMPUS_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_p1_makernotes_t") {
        println!("cargo:rustc-cfg=have_p1_makernotes");
        defines.push("HAVE_P1_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_panasonic_makernotes_t") {
        println!("cargo:rustc-cfg=have_panasonic_makernotes");
        defines.push("HAVE_PANASONIC_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_pentax_makernotes_t") {
        println!("cargo:rustc-cfg=have_pentax_makernotes");
        defines.push("HAVE_PENTAX_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_ricoh_makernotes_t") {
        println!("cargo:rustc-cfg=have_ricoh_makernotes");
        defines.push("HAVE_RICOH_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_samsung_makernotes_t") {
        println!("cargo:rustc-cfg=have_samsung_makernotes");
        defines.push("HAVE_SAMSUNG_MAKERNOTES");
    }

    if check_type(libraw_path, "libraw_sony_info_t") {
        println!("cargo:rustc-cfg=have_sony_info");
        defines.push("HAVE_SONY_INFO");
    }

    if check_type(libraw_path, "libraw_gps_info_t") {
        println!("cargo:rustc-cfg=have_gps_info");
        defines.push("HAVE_GPS_INFO");
    }

    // compile test helper functions
    let mut cc = cc::Build::new();
    cc.include(libraw_path);

    for define in defines {
        cc.define(define, None);
    }

    cc.file("tests/sizes.c").compile("libsizes.a");
    }

    fn check_type(libraw_path: &Path, type_name: &str) -> bool {
      let test_file_path = format!("target/build/check_{}.c", type_name);
      let test_file_name = Path::new(&test_file_path);

      fs::create_dir_all("target/build").unwrap();

      {
          let mut test_file = File::create(test_file_name).unwrap();

          writeln!(&mut test_file, "#include <libraw.h>").unwrap();
          writeln!(&mut test_file, "int main() {{").unwrap();
          writeln!(&mut test_file, "  if (sizeof({})) {{", type_name).unwrap();
          writeln!(&mut test_file, "    return 0;").unwrap();
          writeln!(&mut test_file, "  }}").unwrap();
          writeln!(&mut test_file, "  return 0;").unwrap();
          writeln!(&mut test_file, "}}").unwrap();
      }

      let mut cc = cc::Build::new();
      cc.include(libraw_path);

      let mut command = cc.get_compiler().to_command();

      let output = command.arg("-c").arg(test_file_name).
          arg("-o").arg("target/build/conftest.o").
          output().unwrap();

      output.status.success()
}

fn check_member(libraw_path: &Path, type_name: &str, member_name: &str) -> bool {
    let test_file_path = format!("target/build/check_{}_{}.c", type_name, member_name);
    let test_file_name = Path::new(&test_file_path);

    fs::create_dir_all("target/build").unwrap();

    {
        let mut test_file = File::create(test_file_name).unwrap();
        writeln!(&mut test_file, "#include <libraw.h>").unwrap();
        writeln!(&mut test_file, "int main() {{").unwrap();
        writeln!(&mut test_file, "  static {} aggr;", type_name).unwrap();
        writeln!(&mut test_file, "  if (aggr.{}) {{", member_name).unwrap();
        writeln!(&mut test_file, "    return 0;").unwrap();
        writeln!(&mut test_file, "  }}").unwrap();
        writeln!(&mut test_file, "  return 0;").unwrap();
        writeln!(&mut test_file, "}}").unwrap();
    }

    let mut cc = cc::Build::new();
    cc.include(libraw_path);

    let mut command = cc.get_compiler().to_command();

    let output = command.arg("-c").arg(test_file_name).
        arg("-o").arg("target/build/conftest.o").
        output().unwrap();

    output.status.success()
}
