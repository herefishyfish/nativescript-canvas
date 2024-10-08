import { PixelFormat, TextureDataType } from '../../constants.js';

export interface WebGLCapabilitiesParameters {
	precision?: string | undefined;
	logarithmicDepthBuffer?: boolean | undefined;
}

export class WebGLCapabilities {
	constructor(gl: WebGLRenderingContext, extensions: any, parameters: WebGLCapabilitiesParameters);

	readonly isWebGL2: boolean;

	getMaxAnisotropy: () => number;
	getMaxPrecision: (precision: string) => string;

	textureFormatReadable: (textureFormat: PixelFormat) => boolean;
	textureTypeReadable: (textureType: TextureDataType) => boolean;

	precision: string;
	logarithmicDepthBuffer: boolean;

	maxTextures: number;
	maxVertexTextures: number;
	maxTextureSize: number;
	maxCubemapSize: number;

	maxAttributes: number;
	maxVertexUniforms: number;
	maxVaryings: number;
	maxFragmentUniforms: number;

	vertexTextures: boolean;

	maxSamples: number;
}
