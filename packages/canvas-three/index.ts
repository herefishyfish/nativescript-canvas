declare var global: any;
import '@nativescript/canvas-polyfill';
import * as THREE from './three.webgpu';
(global as any).THREE = global.window.THREE = global.THREE || THREE;
