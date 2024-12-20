package org.nativescript.canvas.svgdemo

import android.graphics.Color
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.view.View
import android.view.ViewGroup
import org.nativescript.canvas.svg.NSCSVG
import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.net.URL
import java.util.concurrent.Executors

class MainActivity : AppCompatActivity() {
	val executor = Executors.newSingleThreadExecutor()
	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		setContentView(R.layout.activity_main)

		val svg = NSCSVG.fromRemoteSync(
			this,
			"https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/tiger.svg")


		val content = findViewById<ViewGroup>(android.R.id.content)
		content.addView(svg!!)

//		NSCSVG.fromRemote(
//			this,
//			"https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/tiger.svg",
//			object :
//				NSCSVG.Callback {
//				override fun onSuccess(view: NSCSVG?) {
//					view?.let { svg ->
//						Log.d("com.test", "svg $svg")
//						runOnUiThread {
//							val content = findViewById<ViewGroup>(android.R.id.content)
//							content.addView(svg)
//						}
//					}
//
//				}
//			})


		//	svg.sync = true

//		val svg = NSCSVG(this)
//		val content = findViewById<ViewGroup>(android.R.id.content)
//		content.addView(svg)
//		downloadSvg(svg)
	}


	fun downloadSvg(svg: NSCSVG) {
		executor.execute {
			try {
				val svgFile = File(filesDir, "svg_file.svg")
				if (svgFile.exists()) {
//					svg.setSrcPath(svgFile.absolutePath)
//					return@execute
					svgFile.delete()
				}

				val url =
					URL("https://raw.githubusercontent.com/RazrFalcon/resvg/7b26adbcc9698dcca687214c84d216794f60a5be/tests/svg/e-radialGradient-013.svg")
				val fs = FileOutputStream(svgFile)
				url.openStream().use { input ->
					fs.use { output ->
						input.copyTo(output)
					}
				}
				svg.setSrc(svgFile.readText())
//				svg.setSrcPath(svgFile.absolutePath)
			} catch (e: IOException) {
				e.printStackTrace()
			}
		}
	}
}
