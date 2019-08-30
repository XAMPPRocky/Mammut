# 0.13
- Added `media` endpoint and `MediaBuilder` to enable media uploads. By @klausi
- Changed `StatusBuilder`'s ID type to be `String`.

# 0.12
 - Added the `StatusesRequest` struct.
 - `Mastodon::statuses` now takes an id and a `StatusesRequest`
 - Documentation should be properly formatted.
 - Added the Page iterator.
 - Updated reqwest to 0.9
 - Fixed various codegen bugs.

# 0.11
- Added more examples to `examples` directory.
- Fixed `follow` and `unfollow` routes.
- Updated `moved` field to be `Box<Account>`.

# 0.10

- Added the ability to handle paged entities like favourites and such.(Only favourites in prerelease)
- Added optional `source` and `moved` fields to `Account`.
- Added `Source` struct to match with the `Account.source` field.
- Added `CredientialsBuilder` struct for updating profile using
  `verify_credientials`.
- Attachment now handles being sent an empty object, which is converted
  to `None`.
- Added ombed data fields to `Card`.
- Added `version` and `urls` fields to `Instance`.
- Added `id`, `muting_notifications`, and `domain_blocking` to `Relationship`.
- Added `emojis`, `language`, and `pinned` fields to `Status`
- Added `Emoji` struct.
- Added `List` and `Mention` structs(matching routes not added yet).
- Added example that prints your profile.
- Updated dependencies
