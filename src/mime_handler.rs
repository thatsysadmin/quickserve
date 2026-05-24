use std::path::Path;

pub fn get_mime(file_name: &Path) -> Option<&str> {
    let file_extension = file_name.extension()?.to_str()?;

    // These were stolen from NGINX's default mime matching list.
    // Used Gemma 4 to convert that list over to a match block.
    // actix_files thought that an HTML file should be an application/octet-stream.
    // https://github.com/nginx/nginx/blob/master/conf/mime.types

    let mime_type = match file_extension.to_lowercase().as_str() {
        // --- Text and Document Types ---
        "html" | "htm" | "shtml" => "text/html",
        "css" => "text/css",
        "xml" => "text/xml",
        "txt" => "text/plain",
        "mml" => "text/mathml",
        "jad" => "text/vnd.sun.j2me.app-descriptor",
        "wml" => "text/vnd.wap.wml",
        "htc" => "text/x-component",
        "xhtml" => "application/xhtml+xml",
        "atom" => "application/atom+xml",
        "rss" => "application/rss+xml",

        // --- Image Types ---
        "gif" => "image/gif",
        "jpeg" | "jpg" => "image/jpeg",
        "avif" => "image/avif",
        "png" => "image/png",
        "svg" | "svgz" => "image/svg+xml",
        "tif" | "tiff" => "image/tiff",
        "wbmp" => "image/vnd.wap.wbmp",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "jng" => "image/x-jng",
        "bmp" => "image/x-ms-bmp",

        // --- Font Types ---
        "woff" => "font/woff",
        "woff2" => "font/woff2",

        // --- Application & Binary Types ---
        "js" => "application/javascript",
        "pdf" => "application/pdf",
        "ps" | "eps" | "ai" => "application/postscript",
        "rtf" => "application/rtf",
        "m3u8" => "application/vnd.apple.mpegurl",
        "kml" => "application/vnd.google-earth.kml+xml",
        "kmz" => "application/vnd.google-earth.kmz",

        // Office Documents (MS & ODF)
        "xls" => "application/vnd.ms-excel",
        "eot" => "application/vnd.ms-fontobject",
        "ppt" => "application/vnd.ms-powerpoint",
        "odg" => "application/vnd.oasis.opendocument.graphics",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odt" => "application/vnd.oasis.opendocument.text",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",

        // Archives & Executables
        "jar" | "war" | "ear" => "application/java-archive",
        "json" => "application/json",
        "hqx" => "application/mac-binhex40",
        "wasm" => "application/wasm",
        "7z" => "application/x-7z-compressed",
        "cco" => "application/x-cocoa",
        "jardiff" => "application/x-java-archive-diff",
        "jnlp" => "application/x-java-jnlp-file",
        "run" => "application/x-makeself",

        // Perl and OS Packages
        "pl" | "pm" => "application/x-perl",
        "prc" | "pdb" => "application/x-pilot",
        "rar" => "application/x-rar-compressed",
        "rpm" => "application/x-redhat-package-manager",
        "sea" => "application/x-sea",
        "swf" => "application/x-shockwave-flash",
        "sit" => "application/x-stuffit",

        // Cryptographic and Certificates
        "der" | "pem" | "crt" => "application/x-x509-ca-cert",

        // Installers & Utilities
        "xpi" => "application/x-xpinstall",
        "zip" => "application/zip",

        // Generic Binary Types
        "bin" | "exe" | "dll" => "application/octet-stream",
        "deb" | "dmg" | "iso" | "img" | "msi" | "msp" | "msm" => "application/octet-stream",

        // --- Audio Types ---
        "mid" | "midi" | "kar" => "audio/midi",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "m4a" => "audio/x-m4a",
        "ra" => "audio/x-realaudio",

        // --- Video Types ---
        "3gpp" | "3gp" => "video/3gpp",
        "ts" => "video/mp2t",
        "mp4" => "video/mp4",
        "mpeg" | "mpg" => "video/mpeg",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "flv" => "video/x-flv",
        "m4v" => "video/x-m4v",
        "mng" => "video/x-mng",
        "asx" | "asf" => "video/x-ms-asf",
        "wmv" => "video/x-ms-wmv",
        "avi" => "video/x-msvideo",

        // --- Fallback (Shouldn't happen if all are mapped) ---
        _ => "application/octet-stream",
    };

    Some(mime_type)
}