package org.nativescript.canvas

import android.graphics.Bitmap
import android.graphics.Color
import android.util.Log
import dalvik.annotation.optimization.FastNative

class NSCCanvasRenderingContext2D {
	companion object {

		init {
			NSCCanvas.loadLib()
		}

		@JvmStatic
		fun drawAtlas(
			context: Long,
			bitmap: Bitmap,
			xform: FloatArray,
			tex: FloatArray,
			colors: IntArray,
			blendMode: Int
		) {
			nativeDrawAtlasWithBitmap(context, bitmap, xform, tex, colors, blendMode)
		}

		@JvmStatic
		fun createPattern(context: Long, bitmap: Bitmap, repetition: String): Long {
			return nativeCreatePattern(context, bitmap, repetition)
		}

		@JvmStatic
		fun drawImage(context: Long, image: Bitmap, dx: Float, dy: Float): Boolean {
			val width = image.width.toFloat()
			val height = image.height.toFloat()
			return nativeDrawImageDxDyWithBitmap(context, image, width, height, dx, dy)
		}

		@JvmStatic
		fun drawImage(
			context: Long,
			image: Bitmap,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean {
			val width = image.width.toFloat()
			val height = image.height.toFloat()
			return nativeDrawImageDxDyDwDhWithBitmap(
				context,
				image,
				width,
				height,
				dx,
				dy,
				dWidth,
				dHeight
			)
		}

		@JvmStatic
		fun drawImage(
			context: Long,
			image: Bitmap,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean {
			val width = image.width.toFloat()
			val height = image.height.toFloat()
			return nativeDrawImageWithBitmap(
				context, image, width, height, sx,
				sy,
				sWidth,
				sHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
		}

		@JvmStatic
		fun drawImage(context: Long, image: Long, dx: Float, dy: Float): Boolean {
			return nativeDrawImageDxDyWithAsset(context, image, dx, dy)
		}

		@JvmStatic
		fun drawImage(
			context: Long,
			image: Long,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean {
			return nativeDrawImageDxDyDwDhWithAsset(
				context,
				image,
				dx,
				dy,
				dWidth,
				dHeight
			)
		}

		@JvmStatic
		fun drawImage(
			context: Long,
			image: Long,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean {
			return nativeDrawImageWithAsset(
				context, image, sx,
				sy,
				sWidth,
				sHeight,
				dx,
				dy,
				dWidth,
				dHeight
			)
		}

		@JvmStatic
		fun scale(
			context: Long,
			x: Float,
			y: Float,
		) {
			nativeScale(context, x, y)
		}

		@JvmStatic
		@FastNative
		private external fun nativeCreatePattern(
			context: Long,
			bitmap: Bitmap,
			repetition: String
		): Long


		@JvmStatic
		@FastNative
		private external fun nativeDrawImageDxDyWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float
		): Boolean

		@JvmStatic
		@FastNative
		private external fun nativeDrawImageDxDyDwDhWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean

		@JvmStatic
		@FastNative
		private external fun nativeDrawImageWithBitmap(
			context: Long,
			bitmap: Bitmap,
			width: Float,
			height: Float,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean


		@JvmStatic
		@FastNative
		private external fun nativeDrawImageDxDyWithAsset(
			context: Long,
			asset: Long,
			dx: Float,
			dy: Float
		): Boolean

		@JvmStatic
		@FastNative
		private external fun nativeDrawImageDxDyDwDhWithAsset(
			context: Long,
			asset: Long,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean

		@JvmStatic
		@FastNative
		private external fun nativeDrawImageWithAsset(
			context: Long,
			asset: Long,
			sx: Float,
			sy: Float,
			sWidth: Float,
			sHeight: Float,
			dx: Float,
			dy: Float,
			dWidth: Float,
			dHeight: Float
		): Boolean


		@JvmStatic
		@FastNative
		private external fun nativeDrawAtlasWithBitmap(
			context: Long,
			bitmap: Bitmap,
			xform: FloatArray,
			tex: FloatArray,
			colors: IntArray,
			blendMode: Int
		)


		@JvmStatic
		@FastNative
		private external fun nativeScale(
			context: Long,
			x: Float,
			y: Float
		)
	}

}
