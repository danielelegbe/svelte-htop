import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
				ws: true
			},
			'/api/usage': {
				target: 'ws://localhost:3000',
				changeOrigin: true,
				ws: true
			}
		}
	}
});
