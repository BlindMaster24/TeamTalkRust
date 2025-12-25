use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let mut builder = bindgen::Builder::default();

    if target_os == "windows" {
        builder = builder.header_contents(
            "wrapper.h",
            "
            typedef unsigned short WCHAR;
            typedef int INT32;
            typedef unsigned int UINT32;
            typedef long long INT64;
            typedef unsigned short UINT16;
            typedef void VOID;
            typedef void* HWND;
            typedef void* HDC;
            #define WIN32
            #include \"TeamTalk.h\"
        ",
        );
    } else {
        builder = builder.header("TeamTalk.h");
    }

    let bindings = builder
        .clang_arg("-DIN=")
        .clang_arg("-DOUT=")
        .dynamic_library_name("TeamTalk5")
        .wrap_unsafe_ops(true)
        .rustified_enum("AudioFileFormat")
        .rustified_enum("AudioPreprocessorType")
        .rustified_enum("BanType")
        .rustified_enum("BitmapFormat")
        .rustified_enum("ChannelType")
        .rustified_enum("ClientError")
        .rustified_enum("ClientEvent")
        .rustified_enum("ClientFlag")
        .rustified_enum("Codec")
        .rustified_enum("DesktopKeyState")
        .rustified_enum("DesktopProtocol")
        .rustified_enum("FileTransferStatus")
        .rustified_enum("FourCC")
        .rustified_enum("MediaFileStatus")
        .rustified_enum("MixerControl")
        .rustified_enum("ServerLogEvent")
        .rustified_enum("SoundDeviceFeature")
        .rustified_enum("SoundLevel")
        .rustified_enum("SoundSystem")
        .rustified_enum("StreamType")
        .rustified_enum("Subscription")
        .rustified_enum("TTKeyTranslate")
        .rustified_enum("TTType")
        .rustified_enum("TextMsgType")
        .rustified_enum("UserRight")
        .rustified_enum("UserState")
        .rustified_enum("UserType")
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
