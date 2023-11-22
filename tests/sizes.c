#include <stddef.h>
#include <libraw.h>

#define EXPORT_SIZE(x) size_t size_of_##x () { return sizeof(x); }

EXPORT_SIZE(libraw_data_t)
EXPORT_SIZE(libraw_image_sizes_t)
EXPORT_SIZE(libraw_iparams_t)

#ifdef HAVE_LENSINFO
EXPORT_SIZE(libraw_lensinfo_t)
#endif

#ifdef HAVE_NIKONLENS
EXPORT_SIZE(libraw_nikonlens_t)
#endif

#ifdef HAVE_DNGLENS
EXPORT_SIZE(libraw_dnglens_t)
#endif

#ifdef HAVE_MAKERNOTES_LENS
EXPORT_SIZE(libraw_makernotes_lens_t)
#endif

EXPORT_SIZE(libraw_output_params_t)
EXPORT_SIZE(libraw_colordata_t)

#ifdef HAVE_PH1
typedef struct ph1_t ph1_t;
EXPORT_SIZE(ph1_t)
#endif

#ifdef HAVE_DNG_COLOR
EXPORT_SIZE(libraw_dng_color_t)
#endif

#ifdef HAVE_CANON_MAKERNOTES
EXPORT_SIZE(libraw_canon_makernotes_t)
#endif

#ifdef HAVE_FUJI_INFO
EXPORT_SIZE(libraw_fuji_info_t)
#endif

#ifdef HAVE_HASSELBLAD_MAKERNOTES
EXPORT_SIZE(libraw_hasselblad_makernotes_t)
#endif

#ifdef HAVE_KODAK_MAKERNOTES
EXPORT_SIZE(libraw_kodak_makernotes_t)
#endif

#ifdef HAVE_NIKON_MAKERNOTES
EXPORT_SIZE(libraw_nikon_makernotes_t)
#endif

#ifdef HAVE_OLYMPUS_MAKERNOTES
EXPORT_SIZE(libraw_olympus_makernotes_t)
#endif

#ifdef HAVE_P1_MAKERNOTES
EXPORT_SIZE(libraw_p1_makernotes_t)
#endif

#ifdef HAVE_PANASONIC_MAKERNOTES
EXPORT_SIZE(libraw_panasonic_makernotes_t)
#endif

#ifdef HAVE_PENTAX_MAKERNOTES
EXPORT_SIZE(libraw_pentax_makernotes_t)
#endif

#ifdef HAVE_RICOH_MAKERNOTES
EXPORT_SIZE(libraw_ricoh_makernotes_t)
#endif

#ifdef HAVE_SAMSUNG_MAKERNOTES
EXPORT_SIZE(libraw_samsung_makernotes_t)
#endif

#ifdef HAVE_SONY_INFO
EXPORT_SIZE(libraw_sony_info_t)
#endif

EXPORT_SIZE(libraw_imgother_t)

#ifdef HAVE_GPS_INFO
EXPORT_SIZE(libraw_gps_info_t)
#endif

EXPORT_SIZE(libraw_thumbnail_t)
EXPORT_SIZE(libraw_rawdata_t)
EXPORT_SIZE(libraw_internal_output_params_t)
EXPORT_SIZE(libraw_processed_image_t)
EXPORT_SIZE(libraw_decoder_info_t)
