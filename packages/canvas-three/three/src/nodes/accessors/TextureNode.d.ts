import { Texture } from '../../textures/Texture.js';
import Node from '../core/Node.js';
import UniformNode from '../core/UniformNode.js';
import { NodeRepresentation, ShaderNodeObject } from '../shadernode/ShaderNode.js';

export default class TextureNode extends UniformNode<Texture> {
	readonly isTextureNode: true;

	uvNode: ShaderNodeObject<Node> | null;
	levelNode: ShaderNodeObject<Node> | null;
	biasNode: ShaderNodeObject<Node> | null;
	compareNode: Node | null;
	depthNode: Node | null;
	gradNode: Node | null;

	sampler: boolean;
	updateMatrix: boolean;

	referenceNode: Node | null;

	constructor(value: Texture, uvNode?: ShaderNodeObject<Node> | null, levelNode?: ShaderNodeObject<Node> | null, biasNode?: ShaderNodeObject<Node> | null);

	getDefaultUV(): Node;

	grad(gradeNodeX: NodeRepresentation, gradeNodeY: NodeRepresentation): ShaderNodeObject<Node>;

	clone(): this;
}

export const texture: (value: Texture, uvNode?: NodeRepresentation, levelNode?: NodeRepresentation, biasNode?: NodeRepresentation) => ShaderNodeObject<TextureNode>;
export const sampler: (aTexture: Texture | TextureNode) => ShaderNodeObject<Node>;

declare module '../shadernode/ShaderNode.js' {
	interface NodeElements {
		texture: typeof texture;
	}
}
