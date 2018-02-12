# 0.10 (prerelease)

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
