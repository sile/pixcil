import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import pkg from './package.json';

const banner = `/**
 * ${pkg.name}
 * ${pkg.description}
 * @version: ${pkg.version}
 * @author: ${pkg.author}
 * @license: ${pkg.license}
 **/
`;

export default [
  {
    input: 'src/pixcil.ts',
    plugins: [
      typescript({module: "esnext"}),
      commonjs(),
      resolve(),
    ],
    output: {
      sourcemap: false,
      file: './dist/pixcil.mjs',
      format: 'module',
      name: 'Pixcil',
      banner: banner,
    }
  },
  {
    input: 'src/pixcil.ts',
    plugins: [
      typescript({module: "esnext"}),
      commonjs(),
      resolve(),
    ],
    output: {
      sourcemap: false,
      file: './dist/pixcil.js',
      format: 'umd',
      name: 'Pixcil',
      banner: banner,
    }
  }
];
