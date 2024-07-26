use super::prelude::*;

pub struct Mingw;

impl PlatformDetails for Mingw {
    fn gn_args(&self, _config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        builder
            .arg("is_msvc", no())
            .arg("is_trivial_abi", no())

            .arg("cc", quote("clang"))
            .arg("cxx", quote("clang++"))
            .arg("clang_win", quote("boop"))
            .arg("clang_win_version", quote("boop2"))
            .arg("target_os", quote("win"))
            
            .arg("win_vc", quote("/tmp/void"))
            .arg("win_sdk_version", quote("dummy"))
            .arg("win_toolchain_version", quote("dummy"))
            
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes())
            .arg("skia_enable_fontmgr_win", no())
            .arg("skia_enable_svg", no())
            .arg("skia_use_freetype", yes())
            ;
    }

    fn bindgen_args(&self, _target: &cargo::Target, builder: &mut BindgenArgsBuilder) {
        builder.arg("-pthread");
    }

    fn link_libraries(&self, features: &Features) -> Vec<String> {
        let mut libs = vec!["usp10", "ole32", "user32", "gdi32", "fontsub"];
        
        if features.gl {
            libs.push("opengl32");
        }
        
        if features.d3d {
            libs.extend(["d3d12", "dxgi", "d3dcompiler"]);
        }

        libs.iter().map(|l| l.to_string()).collect()
    }
    
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }
}
