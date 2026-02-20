package com.woxel.app

import android.content.ContentResolver
import android.graphics.Bitmap
import android.media.ThumbnailUtils
import android.provider.MediaStore

object WoxelPlatformBridge {
    @JvmStatic
    fun requestImageThumb(path: String, size: Int): Bitmap? {
        return ThumbnailUtils.createImageThumbnail(path, android.util.Size(size, size), null)
    }

    @JvmStatic
    fun requestVideoThumb(path: String, size: Int): Bitmap? {
        return ThumbnailUtils.createVideoThumbnail(path, android.util.Size(size, size), null)
    }

    @JvmStatic
    fun resolveMime(contentResolver: ContentResolver, uri: String): String? {
        return contentResolver.getType(android.net.Uri.parse(uri))
    }
}
