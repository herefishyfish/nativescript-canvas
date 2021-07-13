package org.nativescript.canvas

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Matrix
import android.util.AttributeSet
import android.view.View
import java.util.concurrent.Executors

/**
 * Created by triniwiz on 2019-08-05
 */
class TNSSVG : View {
	internal var bitmap: Bitmap? = null
	private var svgCanvas: Long = 0
	private var isInit = false
	internal val lock = Any()
	private var pendingInvalidate: Boolean = false
	private val executor = Executors.newSingleThreadExecutor()
	private var src: String = ""
	private var srcPath: String = ""
	private var mMatrix = Matrix()

	constructor(context: Context) : super(context, null) {
		init(context)
	}

	constructor(context: Context, useCpu: Boolean) : super(context, null) {
		init(context)
	}

	constructor(context: Context, attrs: AttributeSet?) : super(context, attrs) {
		init(context)
	}

	private fun init(context: Context) {
		if (isInEditMode) {
			return
		}
		if (!TNSCanvas.isLibraryLoaded) {
			System.loadLibrary("canvasnative")
			TNSCanvas.isLibraryLoaded = true
		}
	}


	var ignorePixelScaling: Boolean = false
		set(value) {
			field = value
			if (value) {
				val density = resources.displayMetrics.density
				mMatrix.postScale(density, density)
			} else {
				mMatrix.reset()
			}
			if (width > 0 || height > 0) {
				invalidate()
			}
		}

	private fun resize(w: Int, h: Int) {
		executor.execute {
			val metrics = resources.displayMetrics
			TNSCanvas.nativeResizeCustomSurface(
				svgCanvas,
				w.toFloat(),
				h.toFloat(),
				metrics.density,
				true,
				metrics.densityDpi,
			)

			if (srcPath.isNotEmpty() || src.isNotEmpty()) {
				if (srcPath.isNotEmpty()) {
					nativeDrawSVGFromPath(svgCanvas, srcPath)
				} else {
					nativeDrawSVG(svgCanvas, src)
				}
				bitmap?.let {
					TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
					handler!!.post {
						pendingInvalidate = false
						invalidate()
					}
				}
			}
		}
	}


	override fun onSizeChanged(w: Int, h: Int, oldw: Int, oldh: Int) {
		super.onSizeChanged(w, h, oldw, oldh)
		if (w != 0 && h != 0) {
			val metrics = resources.displayMetrics

			bitmap = Bitmap.createBitmap(w, h, Bitmap.Config.ARGB_8888)

			if (svgCanvas == 0L) {
				executor.execute {
					synchronized(lock) {
						svgCanvas = TNSCanvas.nativeInitContextWithCustomSurface(
							w.toFloat(),
							h.toFloat(),
							metrics.density,
							true,
							Color.BLACK,
							metrics.densityDpi.toFloat(),
							TNSCanvas.direction.toNative()
						)

						if (srcPath.isNotEmpty() || src.isNotEmpty()) {

							if (srcPath.isNotEmpty()) {
								nativeDrawSVGFromPath(svgCanvas, srcPath)
							} else {
								nativeDrawSVG(svgCanvas, src)
							}
							bitmap?.let {
								TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
								handler!!.post {
									pendingInvalidate = false
									invalidate()
								}
							}

						}
					}
				}
			} else {
				resize(w, h)
			}
		}
	}

	override fun onDraw(canvas: Canvas) {
		bitmap?.let {
			canvas.drawBitmap(it, mMatrix, null)
		}
	}

	fun flush() {
		bitmap?.let {
			synchronized(lock) {
				if (svgCanvas != 0L) {
					executor.execute {
						TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
						handler!!.post {
							pendingInvalidate = false
							invalidate()
						}
					}
				}
			}
		}
	}

	private fun doDraw() {
		synchronized(lock) {
			if (svgCanvas != 0L) {
				executor.execute {
					if (srcPath.isNotEmpty()) {
						nativeDrawSVGFromPath(svgCanvas, srcPath)
						bitmap?.let {
							if (svgCanvas != 0L) {
								executor.execute {
									TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
									handler!!.post {
										pendingInvalidate = false
										invalidate()
									}
								}
							}
						}
					} else {
						nativeDrawSVG(svgCanvas, src)
						bitmap?.let {
							if (svgCanvas != 0L) {
								executor.execute {
									TNSCanvas.nativeCustomWithBitmapFlush(svgCanvas, it)
									handler!!.post {
										pendingInvalidate = false
										invalidate()
									}
								}
							}
						}
					}
				}
			}
		}
	}

	fun setSrc(src: String) {
		this.src = src
		doDraw()
	}

	fun setSrcPath(path: String) {
		this.srcPath = path
		doDraw()
	}

	companion object {
		@JvmStatic
		private external fun nativeDrawSVG(svgCanvas: Long, svg: String)

		@JvmStatic
		private external fun nativeDrawSVGFromPath(svgCanvas: Long, svg: String)
	}
}