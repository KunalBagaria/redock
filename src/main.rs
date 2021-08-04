use std::io;
use std::fs;
use std::path::Path;
use std::time::Duration;
use settimeout::set_timeout;
use futures::executor::block_on;
use std::process::{ Command, Stdio };

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

fn mkdir() {
    let echo_child = Command::new("mkdir")
        .arg("icons")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");
    println!("Folder created");
    drop(echo_child);
}

fn rename() {
    let echo_child = Command::new("mv")
        .args(["./icons", "./icons.iconset"])
        .spawn()
        .expect("Failed to start echo process");
    println!("Trying to rename icons folder");
    drop(echo_child);
}

fn png_collect() -> String {
    println!("Drag your png here");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Not a valid path of a png!");
    path
}

fn app_collect() -> String {
    println!("Drag your app here");
    let mut app_path_string = String::new();
    io::stdin().read_line(&mut app_path_string).expect("Not a valid path of a application!");
    app_path_string = format!("{}/Contents/Resources/", app_path_string.trim().replace("'", ""));
    let app_path = Path::new(&app_path_string);
    let paths = fs::read_dir(app_path).unwrap();
    let mut to_replace_icon: String = String::new();
    for path in paths {
        let path_string = path.unwrap().path().display().to_string();
        if path_string.contains("icns") {
            to_replace_icon = path_string;
        }
    }
    to_replace_icon
}

async fn main_async() {
    let png_path: String = png_collect();
    println!("Icon Path: {}", png_path);
    let mut iterations: u8 = 0;
    mkdir();

    let app_icon_path = app_collect();
    println!("Path: {}", app_icon_path);

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
            rename();

            let echo_child = Command::new("iconutil")
                .args(["-c", "icns", "./icons.iconset", "-o", "./icons.icns"])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child);

            println!("Converting to icns...");
            set_timeout(Duration::from_secs(3)).await;

            println!("Trying to move icon");
            let echo_child_mv = Command::new("mv")
                .args(["./icons.icns", &app_icon_path[..]])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start echo process");
            drop(echo_child_mv);

            println!("Icon moved");
            
            println!("If you like my work, consider buying me a coffee!");
            println!("BTC: bc1q9sk8gxjvhdneqlh4m80mjw3s5uvzfmph3fqq02");
            println!("ETH: 0x5c1FbEa600C5483562A28aabC9E707bBCEe6F98c");
            println!("DOGE: DTvRUW2zBZyZwkQ48BPYKaorDV7KPTMTQS")

        }

    }
}

fn main() {
    block_on(main_async());
}