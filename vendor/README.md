# vendor/

External donor source trees. Each subdirectory is a separate git repo with
upstream + private remotes. NOT tracked in FocalPoint's git history (see
`.gitignore`) — these are reference sources we consult while adapting specific
files into `apps/` and `crates/` with MIT attribution per `THIRD_PARTY_LICENSES.md`.

## foqos/

- Upstream: https://github.com/awaseem/foqos (MIT © 2024 Ali Waseem)
- Private mirror: https://github.com/KooshaPari/foqos-private
- Role: iOS donor — NFC/QR/Screen Time patterns (`Utils/NFCScannerUtil.swift`,
  `Utils/NFCWriter.swift`, `Utils/PhysicalReader.swift`,
  `Utils/AppBlockerUtil.swift`, `Utils/FamilyActivityUtil.swift`,
  `Utils/StrategyManager.swift`, `Utils/DeviceActivityCenterUtil.swift`,
  `Components/Strategy/QRCodeScanner.swift`, session aggregators).
- NOT adopted: Apple entitlements, Xcode project file, bundle IDs, widget
  Live Activity bundles. FocalPoint authors its own.

## Pulling updates

```bash
cd vendor/foqos
git fetch upstream
git merge upstream/main
git push private
```
