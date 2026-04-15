use teamtalk::types::{BanType, BannedUser};
use teamtalk::utils::strings::to_string;

#[test]
fn banned_user_to_ffi_copies_fields() {
    let user = BannedUser::new(
        "1.2.3.4",
        "/root",
        "nick",
        "user",
        "now",
        BanType::CHANNEL | BanType::IP_ADDR | BanType::USERNAME,
        "admin",
    );
    let raw = user.to_ffi();
    assert_eq!(to_string(&raw.szIPAddress), "1.2.3.4");
    assert_eq!(to_string(&raw.szChannelPath), "/root");
    assert_eq!(to_string(&raw.szNickname), "nick");
    assert_eq!(to_string(&raw.szUsername), "user");
    assert_eq!(to_string(&raw.szOwner), "admin");
    assert_eq!(raw.uBanTypes, 7);
}
