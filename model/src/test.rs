//! Scaffolding for unit tests.

pub mod image_hash {
    //! Static hashes for use in tests.

    use crate::util::image_hash::ImageHash;

    pub const AVATAR_INPUT: &str = "5f95e03c3523a2de6f3136f636fba136";
    pub const AVATAR: ImageHash = new(AVATAR_INPUT);
    pub const BANNER_INPUT: &str = "341680f28074624d4fc9836416c2d519";
    pub const BANNER: ImageHash = new(BANNER_INPUT);
    pub const COVER_INPUT: &str = "ad38c1dce9f7250202b7d8b8c6bcc3c4";
    pub const COVER: ImageHash = new(COVER_INPUT);
    pub const ICON_INPUT: &str = "c273213790e64f8230f7ea035817cbbf";
    pub const ICON: ImageHash = new(ICON_INPUT);
    pub const SPLASH_INPUT: &str = "ce291c6ce4db99fc2e3a69bc20c6e899";
    pub const SPLASH: ImageHash = new(SPLASH_INPUT);

    const fn new(value: &str) -> ImageHash {
        if let Ok(image_hash) = ImageHash::parse(value.as_bytes()) {
            image_hash
        } else {
            panic!("not valid static image hash");
        }
    }
}
