# c2pastrip - remove C2PA manifests

`c2pastrip` is a simple digital self-defense tool to remove C2PA manifests from files.

## Usage

By default, the target file is overwritten in-place:
```
c2pastrip --file picture.jpg
```

Alternatively, specify a different output file:
```
c2pastrip --file picture.jpg --out picture_stripped.jpg
```

## Supported file formats

Since this is a simple wrapper around functionality found in c2pa-rs, the same filetypes are supported. Please note that most have however not been tested by me.

 | Extensions    | MIME type                                           |
 |---------------| --------------------------------------------------- |
 | `avi`         | `video/msvideo`, `video/avi`, `application-msvideo` |
 | `avif`        | `image/avif`                                        |
 | `dng`         | `image/x-adobe-dng`                                 |
 | `heic`        | `image/heic`                                        |
 | `heif`        | `image/heif`                                        |
 | `jpg`, `jpeg` | `image/jpeg`                                        |
 | `m4a`         | `audio/mp4`                                         |
 | `mp3`         | `"audio/mpeg"`                                      |
 | `mp4`         | `video/mp4`, `application/mp4`                      |
 | `mov`         | `video/quicktime`                                   |
 | `png`         | `image/png`                                         |
 | `svg`         | `image/svg+xml`                                     |
 | `tif`,`tiff`  | `image/tiff`                                        |
 | `wav`         | `audio/x-wav`                                       |
 | `webp`        | `image/webp`                                        |

