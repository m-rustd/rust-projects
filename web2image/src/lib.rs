use std::{time::Instant, thread};

use anyhow::{Ok, Result};
use headless_chrome::{
  protocol::cdp::{Page::CaptureScreenshotFormatOption, Target::CreateTarget},
  Browser, LaunchOptionsBuilder,
};
use image::{GenericImageView, load_from_memory, DynamicImage, imageops::overlay, ImageFormat, Luma};
use qrcode::QrCode;

fn url2image(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let options = LaunchOptionsBuilder::default()
        .window_size(Some((1280, 1600)))
        .args(
            [
                std::ffi::OsStr::new("--headless=chrome"),
                std::ffi::OsStr::new("--hide-scrollbars"),
                std::ffi::OsStr::new("--lang=en_US"),
                // Default args but with extensions enabled
                std::ffi::OsStr::new("--disable-background-networking"),
                std::ffi::OsStr::new("--enable-features=NetworkService,NetworkServiceInProcess"),
                std::ffi::OsStr::new("--disable-background-timer-throttling"),
                std::ffi::OsStr::new("--disable-backgrounding-occluded-windows"),
                std::ffi::OsStr::new("--disable-breakpad"),
                std::ffi::OsStr::new("--disable-client-side-phishing-detection"),
                // std::ffi::OsStr::new("--disable-component-extensions-with-background-pages"),
                std::ffi::OsStr::new("--disable-default-apps"),
                std::ffi::OsStr::new("--disable-dev-shm-usage"),
                //    std::ffi::OsStr::new( "--disable-extensions"),
                // BlinkGenPropertyTrees disabled due to crbug.com/937609
                std::ffi::OsStr::new("--disable-features=TranslateUI,BlinkGenPropertyTrees"),
                std::ffi::OsStr::new("--disable-hang-monitor"),
                std::ffi::OsStr::new("--disable-ipc-flooding-protection"),
                // std::ffi::OsStr::new("--disable-popup-blocking"),
                std::ffi::OsStr::new("--disable-prompt-on-repost"),
                std::ffi::OsStr::new("--disable-renderer-backgrounding"),
                std::ffi::OsStr::new("--disable-sync"),
                std::ffi::OsStr::new("--force-color-profile=srgb"),
                std::ffi::OsStr::new("--metrics-recording-only"),
                std::ffi::OsStr::new("--no-first-run"),
                std::ffi::OsStr::new("--enable-automation"),
                std::ffi::OsStr::new("--password-store=basic"),
                std::ffi::OsStr::new("--use-mock-keychain"),
            ]
            .to_vec(),
        )
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    // tab.set_bounds(headless_chrome::types::Bounds::Normal {
    //     left: Some(0),
    //     top: Some(0),
    //     width: Some(1280.0),
    //     height: Some(1600.0),
    // })?;
    
    let viewport = tab
        .navigate_to(url)?
        .wait_until_navigated()?
        .find_element("body")?
        .get_box_model()?
        .margin_viewport();

    dbg!(&viewport);

    // this is a hack for headless chrome as it cannot handle the case that
    // viewport is bigger than window size. I guess pupeeteer have a solution
    // there but for this quick-and-dirty live coding, let's just open a new
    // tab and set its width/height.
    // let tab = browser
    //     .new_tab_with_options(
    //       CreateTarget {
    //           url: url.into(),
    //           width: Some(viewport.width as u32),
    //           height: Some(viewport.height as u32),
    //           browser_context_id: None,
    //           enable_begin_frame_control: None,
    //           new_window: Some(false),
    //           background: None,
    //       }
    //     )?;
    // tab.wait_until_navigated()?;

    let data = tab
        .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(75), None, true)?;
    println!("time spent on url2image: {}ms", start.elapsed().as_millis());

    tab.close(true).expect("TODO: panic message");

    Ok(load_from_memory(&data)?)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let start = Instant::now();
    let code = QrCode::new(url.as_bytes())?;
    // Render the bits into an image.
    let buf = code.render::<Luma<u8>>().build();
    println!(
        "time spent on gen_qrcode: {}ms",
        start.elapsed().as_millis()
    );
    Ok(DynamicImage::ImageLuma8(buf))
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let start = Instant::now();
    let x = bottom.width() - top.width() - 10;
    let y = bottom.height() - top.height() - 10;
    overlay(bottom, top, x as u32, y as u32);
    println!(
        "time spent on do_overlay: {}ms",
        start.elapsed().as_millis()
    );
}

pub fn web2image(url: &str, output: &str, format: ImageFormat) -> Result<()> {
    let url = url.to_owned();
    let url1 = url.clone();
    let bottom_handle = thread::spawn(move || url2image(&url).unwrap());
    let qrcode_handle = thread::spawn(move || gen_qrcode(&url1).unwrap());
    let mut bottom = bottom_handle.join().unwrap();
    let qrcode = qrcode_handle.join().unwrap();

    do_overlay(&mut bottom, &qrcode);

    let start = Instant::now();
    bottom.save_with_format(output, format)?;
    println!("time spent on web2image: {}ms", start.elapsed().as_millis());

    Ok(())
}
