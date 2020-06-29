import copy from 'rollup-plugin-copy';
import autoPreprocess from 'svelte-preprocess';
import postcss from 'rollup-plugin-postcss';
import rust from '@wasm-tool/rollup-plugin-rust';
import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import del from 'del'

const production = !process.env.ROLLUP_WATCH;
const staticDir = 'static'
const distDir = 'dist'
const buildDir = `${distDir}/build`

del.sync(distDir + '/**')
const transform = bundledTransform;
export default {
  input: 'src/main.js',
  output: {
    inlineDynamicImports: true,
    sourcemap: true,
    format: 'iife',
    name: 'app',
    file: `${buildDir}/bundle.js`
  },
  plugins: [
    copy({
      targets: [
        { src: [staticDir + "/*", "!*/(__index.html)"], dest: distDir },
        { src: `${staticDir}/__index.html`, dest: distDir, rename: '__app.html', transform },
      ],
      copyOnce: true,
      flatten: false
    }),
    svelte({
      // enable run-time checks when not in production
      dev: !production,
      // we'll extract any component CSS out into
      // a separate file - better for performance
      css: css => {
        css.write(`${buildDir}/bundle.css`);
      },
      preprocess: autoPreprocess(),
      emitCss: true
    }),

    // If you have external dependencies installed from
    // npm, you'll most likely need these plugins. In
    // some cases you'll need additional configuration -
    // consult the documentation for details:
    // https://github.com/rollup/plugins/tree/master/packages/commonjs
    resolve({
      browser: true,
      dedupe: ['svelte']
    }),
    commonjs(),
    rust({debug: false, serverPath: "/build/"}),
    postcss({
          extract: true,
          minimize: true,
          use: [
            [
              "sass",
              {
                includePaths: ["./node_modules", "./node_modules/bulma", "./src"],
              },
            ],
          ],
        }
    ),

    // In dev mode, call `npm run start` once
    // the bundle has been generated
    !production && serve(),

    // Watch the `public` directory and refresh the
    // browser on changes when not in production
    !production && livereload(distDir),

    // If we're building for production (npm run build
    // instead of npm run dev), minify
    production && terser()
  ],
  watch: {
    clearScreen: false
  }
};

function serve() {
  let started = false;
  return {
    writeBundle() {
      if (!started) {
        started = true;
        require('child_process').spawn('npm', ['run', 'serve'], {
          stdio: ['ignore', 'inherit', 'inherit'],
          shell: true
        });
      }
    }
  };
}

function bundledTransform(contents) {
  return contents.toString().replace('__SCRIPT__', `
	<script defer src="/build/bundle.js" ></script>
	`)
}

function dynamicTransform(contents) {
  return contents.toString().replace('__SCRIPT__', `
	<script type="module" defer src="https://unpkg.com/dimport@1.0.0/dist/index.mjs?module" data-main="/build/main.js"></script>
	<script nomodule defer src="https://unpkg.com/dimport/nomodule" data-main="/build/main.js"></script>
	`)
}
