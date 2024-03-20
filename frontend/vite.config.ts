import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	// proxy to backend
	server: {
		proxy: {
			'/api': 'http://localhost:4000/'
		},
		port: 8084
	}
});
