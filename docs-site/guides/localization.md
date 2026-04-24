# FocalPoint Localization Guide

**Last Updated:** 2026-04-23  
**Status:** Ready for translation partnership

---

## Overview

FocalPoint uses Apple's String Catalog format (`Localizable.xcstrings`) for managing user-visible text. This guide explains how translators and developers work with the extraction system and coordinate translations.

---

## For Translators

### File Location

```
apps/ios/FocalPoint/Sources/FocalPointApp/Resources/Localizable.xcstrings
```

This is a JSON-based text catalog that can be edited directly in Xcode 15+, localization tools, or any text editor.

### How to Translate

1. **Open in Xcode:**
   ```bash
   cd apps/ios/FocalPoint
   open FocalPointApp.xcodeproj
   # Select Localizable.xcstrings in Project Navigator
   # Xcode's Localization interface allows inline translation
   ```

2. **In Localization View:**
   - Left column: English strings (source)
   - Right column: Target language (translate here)
   - Comments: Context hints for ambiguous strings (e.g., "Button label: Mark as done")

3. **Export/Import for External Translation:**
   ```bash
   # Export base strings to XLIFF for CAT tools (Crowdin, Phrase, etc.)
   xcodebuild -exportLocalizations -localizationPath output.xliff -project FocalPointApp.xcodeproj
   
   # Import translated XLIFF back
   xcodebuild -importLocalizations -localizationPath translated.xliff -project FocalPointApp.xcodeproj
   ```

### Translation Units

Each translation unit includes:
- **Key:** Internal identifier (e.g., `"Active rule"`)
- **Source Value:** English text
- **Localization State:** `translated` or `needs_review`
- **Context:** Filename/context hint (inferred from code)

### Example Entry

```json
{
  "Add task" : {
    "extractionState" : "manual",
    "localizations" : {
      "en" : {
        "stringUnit" : {
          "state" : "translated",
          "value" : "Add task"
        }
      },
      "es" : {
        "stringUnit" : {
          "state" : "translated",
          "value" : "Añadir tarea"
        }
      },
      "fr" : {
        "stringUnit" : {
          "state" : "needs_review",
          "value" : "Ajouter une tâche"
        }
      }
    }
  }
}
```

---

## For Developers

### Adding New User-Visible Strings

All user-facing text **must** be localized. Use the pattern:

```swift
Text(String(localized: "My text", defaultValue: "My text"))
Label("Save task", systemImage: "checkmark")  // Label strings auto-localize
```

**Why `defaultValue`?**
- Provides fallback if translation key is missing
- Documents original English intent
- Helps translators understand context

### Common Patterns

#### 1. Simple Static Text
```swift
Text(String(localized: "Active rule", defaultValue: "Active rule"))
```

#### 2. Interpolated Values
```swift
let count = 5
Text(String(localized: "You have \(count) tasks", defaultValue: "You have \(count) tasks"))
```

#### 3. Form Sections & Fields
```swift
Section(String(localized: "Task Details", defaultValue: "Task Details")) {
    TextField(String(localized: "Title", defaultValue: "Title"), text: $title)
}
```

#### 4. Button Labels
```swift
Button(String(localized: "Save", defaultValue: "Save")) { save() }
```

#### 5. Accessibility Labels
```swift
Image(systemName: "bolt.fill")
    .accessibilityLabel(String(localized: "Priority", defaultValue: "Priority"))
```

#### 6. Coachy Dialogue
```swift
CoachyView(
    state: CoachyState(
        bubbleText: String(localized: "Ready for your first task?", defaultValue: "Ready for your first task?")
    )
)
```

### Extraction Workflow

1. **Developers add new localized strings** using `String(localized:)` or `Label()`
2. **Xcode auto-detects on build:**
   ```bash
   xcodebuild -scheme FocalPointApp build
   ```
3. **Strings appear in `Localizable.xcstrings`** with `extractionState: "new"`
4. **Commit to git** with source language only
5. **Translation partner updates target languages** in their Xcode build or CAT tool
6. **Pull translated changes** back into main repo

### Testing Translations

#### Pseudo-Language (QA)
```bash
# Set Xcode scheme → Run → Options → Application Language → "Pseudo-Language (en)"
# All English strings display with markers [‚00 ... 00‚] to detect layout issues
```

#### Specific Language
```bash
# Set Xcode scheme → Run → Options → Application Language → "Français (fr)"
# App renders with French strings; test for text overflow, RTL issues
```

#### Runtime Override
```swift
// In debug builds, force a language:
if #available(iOS 16, *) {
    Bundle.setLanguage("es")  // Not native API; use custom helper
}
```

### Custom String Lookup (If Needed)

By default, SwiftUI's `String(localized:)` reads from `Localizable.xcstrings`. For custom tables:

```swift
Text(String(localized: "My text", table: "CustomTable", defaultValue: "My text"))
```

---

## Language Support & Rollout

### Supported Languages (Planned)

- English (`en`) — Source, shipped
- Spanish (`es`) — Target
- French (`fr`) — Target
- German (`de`) — Target
- Japanese (`ja`) — Target
- Korean (`ko`) — Target
- Simplified Chinese (`zh-Hans`) — Target

### Rollout Strategy

1. **Phase 1:** English launch (done)
2. **Phase 2:** Spanish + French (EOY 2026)
3. **Phase 3:** German + Japanese (2027 Q1)
4. **Phase 4:** Korean + Simplified Chinese (2027 Q2)

### Enabling a New Language

1. **In Xcode:**
   ```
   Project → FocalPointApp target → Localizations
   → Click + → Select language → Done
   ```
   This creates a new section in `Localizable.xcstrings`.

2. **In Code:** No changes needed; `String(localized:)` automatically picks up new language.

3. **Build & Test:**
   ```bash
   xcodebuild -scheme FocalPointApp build
   # Set app language to new language in Settings
   ```

---

## Translator Handoff Process

### 1. Export Base Strings
```bash
cd apps/ios/FocalPoint
# Create XLIFF for external translation
xcodebuild -exportLocalizations \
  -localizationPath FocalPoint_en.xliff \
  -project FocalPointApp.xcodeproj
```

### 2. Send to Translation Partner
- Email `FocalPoint_en.xliff` to translator
- Include glossary: `docs/guides/localization_glossary.md` (define app-specific terms)
- Request back: `FocalPoint_<language>.xliff` with all strings translated

### 3. Import Translations
```bash
# Receive translated XLIFF from partner
xcodebuild -importLocalizations \
  -localizationPath FocalPoint_es.xliff \
  -project FocalPointApp.xcodeproj
```

### 4. Review in Xcode
- Open `Localizable.xcstrings`
- Check each translation for context fit
- Mark as `translated` or flag `needs_review` if uncertain

### 5. QA Testing
- Set app language to new language
- Manually test all views (Home, Today, Tasks, Stats, Rewards, Activity)
- Check for text overflow, missing translations, context mismatches
- Report issues back to translator

### 6. Merge & Release
- Commit `Localizable.xcstrings` with all target languages
- Build & ship release with multi-language support

---

## Quality Assurance Checklist for Translators

- [ ] **Grammar:** All strings use correct grammar, verb forms, and tenses for target language
- [ ] **Terminology:** App-specific terms (e.g., "Credits", "Focus", "Rule", "Ritual") are consistent throughout
- [ ] **Length:** Translation fits UI bounds (no text overflow in buttons, labels)
- [ ] **Punctuation:** Proper use of periods, commas, colons (varies by language)
- [ ] **Placeholders:** Interpolated values like `%lld`, `{rule_priority}` are preserved
- [ ] **Tone:** Matches app's friendly, encouraging voice (review Coachy dialogue samples)
- [ ] **Accessibility:** VoiceOver label translations are natural and descriptive

---

## Common Pitfalls

### 1. Hardcoded Strings in Swift
**Bad:**
```swift
Text("My task")  // Won't localize!
```
**Good:**
```swift
Text(String(localized: "My task", defaultValue: "My task"))
```

### 2. Untranslated Interpolations
**Bad:**
```swift
Text("You earned \(points) points")  // "You earned" is hard-coded
```
**Good:**
```swift
Text(String(localized: "You earned \(points) points", defaultValue: "You earned \(points) points"))
```

### 3. Missing Coachy Dialogue
Coachy's bubble text **must** be localized for personality to carry over:
```swift
CoachyView(state: CoachyState(
    bubbleText: String(localized: "Great work! Keep it up.", defaultValue: "Great work! Keep it up.")
))
```

### 4. Accessibility Labels Not Localized
**Bad:**
```swift
Image(systemName: "bolt.fill")
    .accessibilityLabel("Lightning bolt")  // Won't localize!
```
**Good:**
```swift
Image(systemName: "bolt.fill")
    .accessibilityLabel(String(localized: "Lightning bolt", defaultValue: "Lightning bolt"))
```

---

## Tools & Resources

### CAT Tools (Crowdsourced Translation)
- **Crowdin:** https://crowdin.com — XLIFF import/export, glossary management
- **Phrase:** https://phrase.com — Real-time collaboration, context screenshots
- **Lokalise:** https://lokalise.com — iOS-native integration, pseudo-translation QA

### Community Translation
- **Weblate:** Open-source, self-hosted translation platform
- **Open Translation Collective:** If FocalPoint becomes community-driven

### Glossary & Context
- See `docs/guides/localization_glossary.md` for app-specific terminology
- Xcode String Catalog includes inline comments for context (where available)

---

## Contact & Support

- **Localization Lead:** @koosha (GitHub)
- **Translation Issues:** File under `docs/guides/localization_issues.md`
- **Update Cadence:** Strings extracted on every build; translations updated quarterly

---

**Current String Count:** 122 user-visible strings  
**Translation Capacity:** ~3-5 languages per quarter (depends on translator availability)  
**Maintenance:** Strings automatically added/removed as code evolves; no manual sync needed.
