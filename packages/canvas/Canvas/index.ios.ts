import { CanvasBase, doc, ignoreTouchEventsProperty, DOMRect } from './common';
import { DOMMatrix } from '../Canvas2D';
import { CanvasRenderingContext2D } from '../Canvas2D/CanvasRenderingContext2D';
import { WebGLRenderingContext } from '../WebGL/WebGLRenderingContext';
import { WebGL2RenderingContext } from '../WebGL2/WebGL2RenderingContext';
import { ImageSource, Utils, Screen } from '@nativescript/core';
import { GPUCanvasContext } from '../WebGPU';
import { handleContextOptions } from './utils';
declare var NSCCanvas, NSCCanvasListener;

export function createSVGMatrix(): DOMMatrix {
	return new DOMMatrix();
}

const defaultOpts = {
	alpha: true,
	antialias: true,
	depth: true,
	failIfMajorPerformanceCaveat: false,
	powerPreference: 'default',
	premultipliedAlpha: true,
	preserveDrawingBuffer: false,
	stencil: false,
	desynchronized: false,
	xrCompatible: false,
};

enum ContextType {
	None,
	Canvas,
	WebGL,
	WebGL2,
	WebGPU,
}

const viewRect_ = Symbol('[[viewRect]]');

function valueToNumber(value) {
	switch (typeof value) {
		case 'string':
			return parseFloat(value);
		case 'number':
			return value;
		default:
			return NaN;
	}
}

export class Canvas extends CanvasBase {
	private _2dContext: CanvasRenderingContext2D;
	private _webglContext: WebGLRenderingContext;
	private _webgl2Context: WebGL2RenderingContext;
	private _gpuContext: GPUCanvasContext;
	private _canvas: any;
	private _didPause: boolean = false;
	private _isReady: boolean = false;
	private _readyListener: any;

	private _contextType = ContextType.None;
	private _is2D = false;
	private _isBatch = false;
	_didLayout = false;

	static useSurface = false;
	_renderer;

	constructor(nativeInstance?) {
		super();
		NSCCanvas.forceGL = false;
		if (nativeInstance) {
			// allows Worker usage
			this._canvas = nativeInstance;
			(global as any).__canvasLoaded = true;
		} else {
			this._canvas = NSCCanvas.alloc().initWithFrame(CGRectZero);
			this._canvas.userInteractionEnabled = true;
			const ref = new WeakRef(this);
			const listener = (NSObject as any).extend(
				{
					contextReady() {
						if (!this._isReady) {
							const owner = ref.get();
							if (owner) {
								owner._readyEvent();
								this._isReady = true;
							}
						}
					},
				},
				{
					protocols: [NSCCanvasListener],
				}
			);
			this._readyListener = listener.new();
			this._canvas.setListener(this._readyListener);
			this._canvas.enterBackgroundListener = () => {
				if (!this.native) {
					return;
				}
				this.native.__stopRaf();
			};

			(global as any).__canvasLoaded = true;

			this._canvas.becomeActiveListener = () => {
				if (!this.native) {
					return;
				}
				this.native.__startRaf();
			};

			this._canvas.touchEventListener = (event, recognizer) => {
				this._handleEvents(event);
			};
		}
	}

	static get forceGL() {
		return NSCCanvas.forceGL;
	}

	static set forceGL(value) {
		NSCCanvas.forceGL = value;
	}

	get lang() {
		return NSLocale.currentLocale.languageCode;
	}

	set lang(value: string) {
		// todo
	}

	[ignoreTouchEventsProperty.setNative](value: boolean) {
		this._canvas.ignoreTouchEvents = value;
	}

	// @ts-ignore
	get ios() {
		return this._canvas;
	}

	get clientWidth() {
		return Math.floor(this.getMeasuredWidth() / Screen.mainScreen.scale);
	}

	get clientHeight() {
		return Math.floor(this.getMeasuredHeight() / Screen.mainScreen.scale);
	}

	get drawingBufferHeight() {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.drawingBufferHeight;
	}

	get drawingBufferWidth() {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.drawingBufferWidth;
	}

	// @ts-ignore
	get width(): number {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.surfaceWidth;
	}

	set width(value: number) {
		if (this._canvas === undefined || this._canvas === null) {
			return;
		}

		value = valueToNumber(value);
		if (!Number.isNaN(value)) {
			const newValue = Math.floor(value);
			this._canvas.surfaceWidth = newValue;
			const { fit, transform } = this._getFit(this._canvas.frame);
			this._updateScale(fit, transform ?? CATransform3DIdentity);
		}
	}

	// @ts-ignore
	get height(): number {
		if (this._canvas === undefined || this._canvas === null) {
			return 0;
		}
		return this._canvas.surfaceHeight;
	}

	set height(value: number) {
		if (this._canvas === undefined || this._canvas === null) {
			return;
		}

		value = valueToNumber(value);
		if (!Number.isNaN(value)) {
			const newValue = Math.floor(value);
			this._canvas.surfaceHeight = newValue;
			const { fit, transform } = this._getFit(this._canvas.frame);
			this._updateScale(fit, transform ?? CATransform3DIdentity);
		}
	}

	private _iosOverflowSafeArea = false;

	//@ts-ignore
	get iosOverflowSafeArea() {
		return this._iosOverflowSafeArea;
	}

	set iosOverflowSafeArea(value: boolean) {
		const window = UIApplication.sharedApplication.windows[0];
		//const topPadding = window.safeAreaInsets.top;
		const bottomPadding = window.safeAreaInsets.bottom;
		if (bottomPadding === 0) {
			this._iosOverflowSafeArea = false;
		} else {
			this._iosOverflowSafeArea = value;
		}
	}

	static createCustomView() {
		const canvas = new Canvas();
		canvas._isCustom = true;
		return canvas;
	}

	onLayout(left: number, top: number, right: number, bottom: number) {
		super.onLayout(left, top, right, bottom);
		if (!this.parent) {
			return;
		}
		if (!Object.hasOwn(this.parent, 'clientWidth') && !Object.hasOwn(this.parent, 'clientHeight')) {
			Object.defineProperties(this.parent, {
				clientWidth: {
					get: function () {
						return Math.floor(this.getMeasuredWidth() / Screen.mainScreen.scale);
					},
				},
				clientHeight: {
					get: function () {
						return Math.floor(this.getMeasuredHeight() / Screen.mainScreen.scale);
					},
				},
			});
		}

		if (typeof (<any>this.parent).getBoundingClientRect !== 'function') {
			(<any>this.parent).getBoundingClientRect = function () {
				const view = this;
				if (!view) {
					return new DOMRect(0, 0, 0, 0);
				}
				const nativeView = view?.nativeView;
				if (!view[viewRect_]) {
					view[viewRect_] = new Float32Array(8);
				}

				if (!nativeView) {
					return new DOMRect(0, 0, 0, 0);
				}

				nativeView.getBoundingClientRect(view[viewRect_]);

				return new DOMRect(view[viewRect_][6], view[viewRect_][7], view[viewRect_][4], view[viewRect_][5], view[viewRect_][0], view[viewRect_][1], view[viewRect_][2], view[viewRect_][3]);
			};
		}

		if (!Object.hasOwn(this.parent, 'ownerDocument')) {
			Object.defineProperty(this.parent, 'ownerDocument', {
				get: function () {
					return global?.window?.document ?? doc;
				},
			});
		}
	}

	public onMeasure(widthMeasureSpec: number, heightMeasureSpec: number) {
		const nativeView = this._canvas;
		if (nativeView) {
			const width = Utils.layout.getMeasureSpecSize(widthMeasureSpec);
			const height = Utils.layout.getMeasureSpecSize(heightMeasureSpec);
			this.setMeasuredDimension(width, height);
		}
	}

	createNativeView() {
		return this._canvas;
	}

	disposeNativeView(): void {
		this._canvas?.setListener?.(null);
		this._readyListener = undefined;
		this._canvas = undefined;
		super.disposeNativeView();
	}

	_getFit(frame: CGRect): { fit: number; frame: CGRect; transform?: CATransform3D } {
		const styleWidth = this.style.width;
		const styleHeight = this.style.height;
		let fit = 1;
		let newFrame = frame;

		if (typeof styleWidth === 'object' && typeof styleHeight === 'object') {
			if (styleWidth?.unit === '%' && styleWidth.value >= 1 && styleHeight?.unit === '%' && styleHeight.value >= 1) {
				fit = 1;
			} else if ((styleWidth?.unit === 'px' || styleWidth?.unit === 'dip') && (styleHeight?.unit === 'px' || styleHeight?.unit === 'dip')) {
				const width = Math.floor(this._canvas.surfaceWidth / Screen.mainScreen.scale);
				const height = Math.floor(this._canvas.surfaceHeight / Screen.mainScreen.scale);

				if (frame.size.width > width || frame.size.height > height) {
					fit = 4;
				} else {
					fit = 1;
				}
			} else {
				fit = 1;
			}
		} else if (typeof styleWidth === 'object' && styleHeight === 'auto') {
			if (styleWidth?.unit === 'px' || styleWidth?.unit === 'dip' || styleWidth?.unit === '%') {
				fit = 2;
			}
		} else if (styleWidth === 'auto' && typeof styleHeight === 'object') {
			if (styleHeight?.unit === 'px' || styleHeight?.unit === 'dip' || styleHeight?.unit === '%') {
				fit = 3;
			}
		} else if (styleWidth === 'auto' && styleHeight === 'auto') {
			// when auto/auto is set force the frame size to be the same as the canvas
			const width = Math.floor(this._canvas.surfaceWidth / Screen.mainScreen.scale);
			const height = Math.floor(this._canvas.surfaceHeight / Screen.mainScreen.scale);
			newFrame = CGRectMake(frame.origin.x, frame.origin.y, width, height);
			fit = 0;
		} else {
			fit = 1;
		}

		let transform: CATransform3D | undefined;

		const scaledInternalWidth = Math.floor(this._canvas.surfaceWidth / Screen.mainScreen.scale);
		const scaledInternalHeight = Math.floor(this._canvas.surfaceHeight / Screen.mainScreen.scale);

		let scaleX = frame.size.width / scaledInternalWidth;
		let scaleY = frame.size.height / scaledInternalHeight;

		switch (fit) {
			case 0:
				transform = CATransform3DIdentity;
				break;
			case 1:
				transform = CATransform3DMakeScale(scaleX, scaleY, 0);
				break;
			case 2:
				{
					const dx = (frame.size.width - scaledInternalWidth) / 2;
					const dy = (scaledInternalHeight * scaleX - scaledInternalHeight) / 2;

					transform = CATransform3DMakeScale(scaleX, scaleX, 0);

					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 3:
				{
					const dx = (scaledInternalWidth * scaleY - scaledInternalWidth) / 2;
					const dy = (frame.size.height - scaledInternalHeight) / 2;

					transform = CATransform3DMakeScale(scaleY, scaleY, 0);

					transform = CATransform3DConcat(transform, CATransform3DMakeTranslation(dx, dy, 0));
				}
				break;
			case 4:
				{
					const scale = Math.min(Math.min(scaleX, scaleY), 1);

					transform = CATransform3DMakeScale(scale, scale, 1);
				}
				break;
		}

		return { fit, frame: newFrame, transform };
	}

	_updateScale(fit: number, transform: CATransform3D) {
		CATransaction.begin();
		CATransaction.setDisableActions(true);
		this._canvas.subviews.objectAtIndex(0).layer.transform = transform;
		this._canvas.subviews.objectAtIndex(1).layer.transform = transform;
		CATransaction.commit();
	}

	_setNativeViewFrame(nativeView: any, currentFrame: CGRect): void {
		const { fit, frame, transform } = this._getFit(currentFrame);
		super._setNativeViewFrame(nativeView, frame);
		this._updateScale(fit, transform ?? CATransform3DIdentity);
	}

	getContext(type: string, options?: any): CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | GPUCanvasContext | null {
		if (!this._canvas) {
			return null;
		}
		if (typeof type === 'string') {
			if (type === '2d') {
				if (this._webglContext || this._webgl2Context) {
					return null;
				}

				if (!this._2dContext) {
					const opts = { ...defaultOpts, ...handleContextOptions(type, options), fontColor: this.parent?.style?.color?.android || -16777216 };

					const ctx = this._canvas.create2DContext(opts.alpha, opts.antialias, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible, opts.fontColor);

					this._2dContext = new (CanvasRenderingContext2D as any)(ctx);

					// // @ts-ignore
					(this._2dContext as any)._canvas = this;

					this._contextType = ContextType.Canvas;
					// @ts-ignore
					this._2dContext._type = '2d';
					this._is2D = true;
				}

				return this._2dContext;
			} else if (type === 'webgl' || type === 'experimental-webgl') {
				if (this._2dContext || this._webgl2Context) {
					return null;
				}
				if (!this._webglContext) {
					const opts = { version: 1, ...defaultOpts, ...handleContextOptions(type, options) };

					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

					this._webglContext = new (WebGLRenderingContext as any)(this._canvas, opts);
					(this._webglContext as any)._canvas = this;
					this._webglContext._type = 'webgl';
					this._contextType = ContextType.WebGL;
				}
				return this._webglContext;
			} else if (type === 'webgl2' || type === 'experimental-webgl2') {
				if (this._2dContext || this._webglContext) {
					return null;
				}

				if (!this._webgl2Context) {
					const opts = { version: 2, ...defaultOpts, ...handleContextOptions(type, options) };

					this._canvas.initContext(type, opts.alpha, false, opts.depth, opts.failIfMajorPerformanceCaveat, opts.powerPreference, opts.premultipliedAlpha, opts.preserveDrawingBuffer, opts.stencil, opts.desynchronized, opts.xrCompatible);

					this._webgl2Context = new (WebGL2RenderingContext as any)(this._canvas, opts);
					(this._webgl2Context as any)._canvas = this;
					(this._webgl2Context as any)._type = 'webgl2';
					this._contextType = ContextType.WebGL2;
				}
				return this._webgl2Context;
			} else if (type === 'webgpu') {
				if (this._2dContext || this._webglContext || this._webgl2Context) {
					return null;
				}

				if (!this._gpuContext) {
					const ptr = navigator.gpu.native.__getPointer();
					const number = NSNumber.numberWithLong(Number(ptr));
					this._canvas.initWebGPUContext(number);

					this._gpuContext = new (GPUCanvasContext as any)(this._canvas);

					(this._gpuContext as any)._canvas = this;
					(this._gpuContext as any)._type = 'webgpu';
					this._contextType = ContextType.WebGPU;
				}

				return this._gpuContext;
			}
		}
		return null;
	}

	get __native__context() {
		switch (this._contextType) {
			case ContextType.Canvas:
				return this._2dContext.native;
			case ContextType.WebGL:
				return this._webglContext.native;
			case ContextType.WebGL2:
				return this._webgl2Context.native;
			case ContextType.WebGPU:
				return this._gpuContext.native;
			default:
				return null;
		}
	}

	get native() {
		return this.__native__context;
	}

	toDataURL(type = 'image/png', encoderOptions = 0.92) {
		if (this.width === 0 || this.height === 0) {
			return 'data:,';
		}
		if (!this.native) {
			return this._canvas.toDataURL(type, encoderOptions);
		}
		if (this._contextType === ContextType.WebGPU) {
			return this._gpuContext.__toDataURL(type, encoderOptions);
		}
		return this.native?.__toDataURL?.(type, encoderOptions);
	}

	snapshot(flip: boolean = false): ImageSource | null {
		if (this._canvas) {
			const bm = this._canvas.snapshot?.(flip ?? false);
			if (bm) {
				return new ImageSource(bm);
			}
		}
		return null;
	}

	private _jsBuffer: Float32Array;

	private get _boundingClientRect() {
		if (this._jsBuffer === undefined) {
			this._jsBuffer = new Float32Array(8);
		}
		return this._jsBuffer;
	}

	getBoundingClientRect(): {
		x: number;
		y: number;
		width: number;
		height: number;
		top: number;
		right: number;
		bottom: number;
		left: number;
	} {
		if (!this._canvas || !this.parent) {
			return new DOMRect(0, 0, 0, 0);
		}

		this._canvas.getBoundingClientRect(this._boundingClientRect);

		return new DOMRect(this._boundingClientRect[6], this._boundingClientRect[7], this._boundingClientRect[4], this._boundingClientRect[5], this._boundingClientRect[0], this._boundingClientRect[1], this._boundingClientRect[2], this._boundingClientRect[3]);
	}
}
