import { resolve } from 'node:path';
import react from '@vitejs/plugin-react';
import { defineConfig, loadEnv } from 'vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';
import tsconfigPaths from 'vite-tsconfig-paths';

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
	process.env = { ...process.env, ...loadEnv(mode, process.cwd(), '') };
	return {
		server: {
			host: process.env.HOST || 'localhost',
			watch: {
				usePolling: true,
			},
		},
		build: {
			rollupOptions: {
				input: {
					main: resolve(__dirname, 'index.html'),
					app: resolve(__dirname, 'app/index.html'),
					openmct: resolve(__dirname, 'openmct/index.html'),
				},
				onwarn: (warning, warn) => {
					if (warning.code === 'MODULE_LEVEL_DIRECTIVE') return;
					warn(warning);
				},
			},
		},
		plugins: [
			viteStaticCopy({
				targets: [
					{
						src: './node_modules/openmct/dist/*',
						dest: 'openmct-lib',
					},
				],
			}),
			tsconfigPaths(),
			react(),
		],
	};
});
