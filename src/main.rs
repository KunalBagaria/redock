use std::io;
use std::env;
use std::time::Duration;
use settimeout::set_timeout;
use futures::executor::block_on;
use std::process::{ Command, Stdio };

mod filesystem;
pub use crate::filesystem::functions;

struct IconParameters {
    pub width: u32,
    pub scale: u8,
}

impl IconParameters {
    fn get_icon_name(&self) -> String {
        let icon_name: String = if self.scale == 1 {
            format!("icon_{}x{}.png", &self.width, &self.width)
        } else {
            format!("icon_{}x{}@2x.png", &self.width / 2, &self.width / 2)
        };
        icon_name
    }
}

fn png_collect() -> String {
    println!("Drag your png here");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Not a valid path of a png!");
    path = path.replace("\\", "");
    path
}

fn app_collect() -> String {
    println!("Drag your app here");
    let mut app_path_string = String::new();
    io::stdin().read_line(&mut app_path_string).expect("Not a valid path of a application!");
    app_path_string = format!("{}", app_path_string.trim().replace("'", "").replace("\\", ""));
    app_path_string
}

async fn main_async() {

    let args: Vec<_> = env::args().collect();
    let mut png_path: String = String::new();
    let mut app_icon_path: String = String::new();

    if args.len() == 3 {
        // println!("{:?}", args);
        png_path = args[1].clone();
        app_icon_path = args[2].clone();
    } else if args.len() == 2 {
        println!(r#"Usage:
redock <png_path> <app_path>
"#);
        return
    } else {
        png_path = png_collect();
        app_icon_path = app_collect();
    }

    println!("Icon Path: {}", png_path);
    println!("Path: {}", app_icon_path);

    let mut iterations: u8 = 0;

    let folder = functions::mkdir();
    drop(folder);

    let python_script = r#"import Cocoa
import sys

Cocoa.NSWorkspace.sharedWorkspace().setIcon_forFile_options_(
  Cocoa.NSImage.alloc().initWithContentsOfFile_(sys.argv[1]),
  sys.argv[2],
  0
) or sys.exit("Unable to set file icon")
    "#;

    let list_of_icons: Vec<IconParameters> = vec![
        IconParameters { width: 16, scale: 1 },
        IconParameters { width: 16, scale: 2 },
        IconParameters { width: 32, scale: 1 },
        IconParameters { width: 32, scale: 2 },
        IconParameters { width: 64, scale: 1 },
        IconParameters { width: 64, scale: 2 },
        IconParameters { width: 128, scale: 1 },
        IconParameters { width: 128, scale: 2 },
        IconParameters { width: 256, scale: 1 },
        IconParameters { width: 256, scale: 2 },
        IconParameters { width: 512, scale: 1 },
        IconParameters { width: 512, scale: 2 },
        IconParameters { width: 1024, scale: 1 },
        IconParameters { width: 1024, scale: 2 }
    ];

    for icon in list_of_icons {
        let icon_path: &str = &(format!("icons/{}", icon.get_icon_name()))[..];
        let icon_width: &str = &(format!("{}", icon.width))[..];
        let png_path_str: &str = &(format!("{}", png_path.trim().replace("'", "")))[..];
        
        let echo_child = Command::new("sips")
            .args(["-z", icon_width, icon_width, png_path_str, "--out", icon_path])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start echo process");
        drop(echo_child);

        println!("Converted {}", icon_path);
        
        iterations += 1;

        if iterations == 14 {

            println!("Waiting...");
            
            set_timeout(Duration::from_secs(3)).await;

            let renamed = functions::rename();
            drop(renamed);

            let echo_child = Command::new("iconutil")
                .args(["-c", "icns", "./icons.iconset", "-o", "./icons.icns"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child);

            println!("Converting to icns...");
            set_timeout(Duration::from_secs(3)).await;

            println!("Trying to move icon");

            let echo_child_mv = Command::new("python")
                .args(["-c", python_script, "./icons.icns", &app_icon_path[..]])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child_mv);

            println!("Clearing cache and cleaning up");
            set_timeout(Duration::from_secs(3)).await;

            let echo_child_rm = Command::new("bash")
                .args(["-c", "rm /var/folders/*/*/*/com.apple.dock.iconcache; killall Dock"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child_rm);

            set_timeout(Duration::from_secs(3)).await;

            let removed_folder = functions::remove();
            drop(removed_folder);

            let deleted_icon = functions::del();
            drop(deleted_icon);

            println!("Process Complete");
        }

    }
}

fn main() {
    block_on(main_async());
}