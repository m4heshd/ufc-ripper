import path from 'node:path';
import {spawn} from 'node:child_process';
import {platform} from 'node:os';
import {defineConfig} from 'vite';
import vue from '@vitejs/plugin-vue';

// https://vitejs.dev/config/
export default defineConfig(async ({command}) => ({
    root: path.resolve(__dirname, 'frontend'),
    build: {
        outDir: path.resolve(__dirname, 'dist'),
        emptyOutDir: true
    },
    plugins: [
        vue(),
        {
            name: 'server-trigger',
            async buildStart() {
                if (command === 'serve') {
                    spawn(
                        platform() === 'win32' ? 'npm.cmd' : 'npm',
                        ['run', 'dev:backend'],
                        {stdio: 'inherit'}
                    );
                }
            },
        }
    ],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'frontend', 'src')
        }
    },
    server: {
        port: 8384,
        strictPort: true
    }
}));
