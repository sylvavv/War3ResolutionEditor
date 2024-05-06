#![windows_subsystem = "windows"]

use winsafe::{co, gui, prelude::*, HKEY};

const TITLE: &'static str = "War3ResolutionEditor";
const KEY_PATH: &'static str = r"SOFTWARE\Blizzard Entertainment\Warcraft III\Video";

fn main() {
    window_main().run_main(None).unwrap();
}

fn window_main() -> gui::WindowMain {
    let window_options = gui::WindowMainOpts {
        title: TITLE.to_owned(),
        size: (210, 76),
        class_icon: gui::Icon::Id(1),
        ..Default::default()
    };

    let window = gui::WindowMain::new(window_options);

    let _label_width = gui::Label::new(
        &window,
        gui::LabelOpts {
            text: "宽度/Width".to_string(),
            position: (10, 10),
            ..Default::default()
        },
    );
    let _label_height = gui::Label::new(
        &window,
        gui::LabelOpts {
            text: "高度/Hight".to_string(),
            position: (10, 40),
            ..Default::default()
        },
    );

    let edit_width = gui::Edit::new(
        &window,
        gui::EditOpts {
            position: (90, 10),
            width: 40,
            height: 20,
            edit_style: co::ES::CENTER | co::ES::NUMBER,
            ..Default::default()
        },
    );

    let edit_height = gui::Edit::new(
        &window,
        gui::EditOpts {
            position: (90, 40),
            width: 40,
            height: 20,
            edit_style: co::ES::CENTER | co::ES::NUMBER,
            ..Default::default()
        },
    );

    let button_set_resolution = gui::Button::new(
        &window,
        gui::ButtonOpts {
            text: "修改\nChange".to_owned(),
            position: (140, 10),
            width: 60,
            height: 50,
            button_style: co::BS::CENTER | co::BS::VCENTER | co::BS::MULTILINE | co::BS::PUSHBUTTON,
            ..Default::default()
        },
    );

    let window1 = window.clone();
    button_set_resolution.on().bn_clicked(move || {
        let width;
        let height;

        if let Ok(ok) = edit_width.text().parse::<u32>() {
            width = ok;
        } else {
            return Ok(());
        }

        if let Ok(ok) = edit_height.text().parse::<u32>() {
            height = ok;
        } else {
            return Ok(());
        }

        let reg_current_user;
        if let Ok(reg) = HKEY::RegOpenCurrentUser(co::KEY::ALL_ACCESS) {
            reg_current_user = reg;
        } else {
            window1
                .hwnd()
                .MessageBox("无法打开注册表 CURRENT_USER", "", co::MB::OK)?;

            return Ok(());
        }

        reg_current_user.RegSetKeyValue(
            Some(KEY_PATH),
            Some("reswidth"),
            winsafe::RegistryValue::Dword(width),
        )?;

        reg_current_user.RegSetKeyValue(
            Some(KEY_PATH),
            Some("resheight"),
            winsafe::RegistryValue::Dword(height),
        )?;

        reg_current_user.RegSetKeyValue(
            Some(KEY_PATH),
            Some("cinematicwidth"),
            winsafe::RegistryValue::Dword(width),
        )?;

        reg_current_user.RegSetKeyValue(
            Some(KEY_PATH),
            Some("cinematicheight"),
            winsafe::RegistryValue::Dword(height),
        )?;

        Ok(())
    });

    window
}
