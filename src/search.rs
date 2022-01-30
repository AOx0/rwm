use crate::utils::*;
use rwm_locals::Filtrable;
use rwm_installer::Installer;
use crate::args::{Local, Steam};

pub fn search_locally(
    i: Installer, args: Local
) {
    let d_type = rwm_locals::DisplayType::from(args.large);
    let mods = GameMods::from(i.rim_install.unwrap())
        .with_display( d_type);

    let filtered = mods.filter_by(args.to_filter_obj(), &args.string);

    if !filtered.is_empty() {
        filtered.display();
    } else {
        println!("No results found")
    }
}

pub async fn search_steam(args: Steam) {
    let mods = SteamMods::search(&args.r#mod)
        .await
        .with_display(rwm_locals::DisplayType::from(args.large));

    mods.display();
}
