// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

use assert_cmd::Command;
use image::DynamicImage;
use predicates::prelude::predicate;

fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests");
    command
}

#[test]
fn generate_completion_conflicts_with_subcommands() {
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("encode")
        .assert()
        .failure()
        .code(2);
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("decode")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn long_version() {
    command()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/long-version.md"
        )));
}

#[test]
fn after_long_help() {
    command()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/after-long-help.md"
        )));
}

#[test]
fn basic_encode() {
    let output = command().arg("encode").arg("QR code").output().unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn validate_aliases_for_encode_command() {
    command().arg("enc").arg("-V").assert().success();
    command().arg("e").arg("-V").assert().success();
}

#[test]
fn encode_data_from_file() {
    let output = command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_error_correction_level() {
    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("l")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/level/low.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("q")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/level/quartile.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/level/high.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_margin() {
    let output = command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/margin/8.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_png() {
    let output = command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg() {
    command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::str::contains("<svg"));
}

#[test]
fn encode_to_terminal() {
    command()
        .arg("encode")
        .arg("-t")
        .arg("terminal")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::str::starts_with('\u{2588}'));
}

#[test]
fn encode_as_micro_qr_code() {
    let output = command()
        .arg("encode")
        .arg("-v")
        .arg("3")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/variant/micro.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_fg_color() {
    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/fg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_bg_color() {
    let output = command()
        .arg("encode")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/bg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_color() {
    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/rgb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_hex_fg_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_bg_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_color_with_alpha() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a7f")
            .arg("--background")
            .arg("#7788997f")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a7f")
            .arg("--background")
            .arg("7788997f")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_short_hex_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("#111")
            .arg("--background")
            .arg("#eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("111")
            .arg("--background")
            .arg("eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_short_hex_color_with_alpha() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("#1118")
            .arg("--background")
            .arg("#eee8")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba_short.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("1118")
            .arg("--background")
            .arg("eee8")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba_short.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgb_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165 42 42)")
            .arg("--background")
            .arg("rgb(119 136 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165, 42, 42)")
            .arg("--background")
            .arg("rgb(119, 136, 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgb_color_with_alpha() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165 42 42 / 49.8%)")
            .arg("--background")
            .arg("rgb(119 136 153 / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165, 42, 42, 49.8%)")
            .arg("--background")
            .arg("rgb(119, 136, 153, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgba_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgba(165 42 42 / 49.8%)")
            .arg("--background")
            .arg("rgba(119 136 153 / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgba(165, 42, 42, 49.8%)")
            .arg("--background")
            .arg("rgba(119, 136, 153, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsl_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248 39% 39.2%)")
            .arg("--background")
            .arg("hsl(0 0% 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248, 39%, 39.2%)")
            .arg("--background")
            .arg("hsl(0, 0%, 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsl_color_with_alpha() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248 39% 39.2% / 49.8%)")
            .arg("--background")
            .arg("hsl(0 0% 66.3% / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248, 39%, 39.2%, 49.8%)")
            .arg("--background")
            .arg("hsl(0, 0%, 66.3%, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsla_color() {
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsla(248 39% 39.2% / 49.8%)")
            .arg("--background")
            .arg("hsla(0 0% 66.3% / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsla(248, 39%, 39.2%, 49.8%)")
            .arg("--background")
            .arg("hsla(0, 0%, 66.3%, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hwb_color() {
    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(50.6 0% 0%)")
        .arg("--background")
        .arg("hwb(0 66.3% 33.7%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/hwb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_hwb_color_with_alpha() {
    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(50.6 0% 0% / 49.8%)")
        .arg("--background")
        .arg("hwb(0 66.3% 33.7% / 49.8%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout).unwrap(),
        image::open("tests/data/colored/hwba.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_verbose() {
    command()
        .arg("encode")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn long_version_for_encode_command() {
    command()
        .arg("encode")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_encode_command() {
    command()
        .arg("encode")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/encode-after-long-help.md"
        )));
}

#[test]
fn validate_the_options_dependencies_for_encode_command() {
    command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
    command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn basic_decode() {
    command()
        .arg("decode")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[test]
fn validate_aliases_for_decode_command() {
    command().arg("dec").arg("-V").assert().success();
    command().arg("d").arg("-V").assert().success();
}

#[test]
fn decode_from_bmp() {
    command()
        .arg("decode")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("bmp")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("bmp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_dds() {
    command()
        .arg("decode")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("dds")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("dds")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_farbfeld() {
    command()
        .arg("decode")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("farbfeld")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("farbfeld")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_gif() {
    command()
        .arg("decode")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("gif")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("gif")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_hdr() {
    command()
        .arg("decode")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("hdr")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("hdr")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_bmp_cur() {
    command()
        .arg("decode")
        .arg("data/decode/bmp.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/bmp.cur")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_png_cur() {
    command()
        .arg("decode")
        .arg("data/decode/png.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/png.cur")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_bmp_ico() {
    command()
        .arg("decode")
        .arg("data/decode/bmp.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/bmp.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_png_ico() {
    command()
        .arg("decode")
        .arg("data/decode/png.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/png.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_jpeg() {
    command()
        .arg("decode")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("jpeg")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("jpeg")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_open_exr() {
    command()
        .arg("decode")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("openexr")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("openexr")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_png() {
    command()
        .arg("decode")
        .arg("-t")
        .arg("png")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("png")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_ascii_pbm() {
    command()
        .arg("decode")
        .arg("data/decode/ascii.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_ascii_pgm() {
    command()
        .arg("decode")
        .arg("data/decode/ascii.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_ascii_ppm() {
    command()
        .arg("decode")
        .arg("data/decode/ascii.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_binary_pbm() {
    command()
        .arg("decode")
        .arg("data/decode/binary.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_binary_pgm() {
    command()
        .arg("decode")
        .arg("data/decode/binary.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_binary_ppm() {
    command()
        .arg("decode")
        .arg("data/decode/binary.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_qoi() {
    command()
        .arg("decode")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("qoi")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("qoi")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svg() {
    command()
        .arg("decode")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/basic/basic.png")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svgz() {
    command()
        .arg("decode")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/basic/basic.png")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_tga() {
    command()
        .arg("decode")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("tga")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("tga")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_tiff() {
    command()
        .arg("decode")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("tiff")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("tiff")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_lossy_web_p() {
    command()
        .arg("decode")
        .arg("data/decode/lossy.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/lossy.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_lossless_web_p() {
    command()
        .arg("decode")
        .arg("data/decode/lossless.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/lossless.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_with_verbose() {
    command()
        .arg("decode")
        .arg("--verbose")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn decode_with_metadata() {
    command()
        .arg("decode")
        .arg("--metadata")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn validate_the_options_dependencies_for_decode_command() {
    command()
        .arg("decode")
        .arg("--verbose")
        .arg("--metadata")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn long_version_for_decode_command() {
    command()
        .arg("decode")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_decode_command() {
    command()
        .arg("decode")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "../src/assets/decode-after-long-help.md"
        )));
}
