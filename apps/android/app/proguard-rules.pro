# ProGuard rules for FocalPoint Android
# Placeholder; no obfuscation in debug builds.

-keepattributes SourceFile, LineNumberTable
-renamesourcefileattribute SourceFile

# Keep generated FFI bindings
-keep class com.focalpoint.ffi.** { *; }
