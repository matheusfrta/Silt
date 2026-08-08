import { transformSync } from '@babel/core';

export default function siltPlugin() {
  return {
    name: 'vite-plugin-silt',
    enforce: 'pre',
    transform(code, id) {
      if (!/\.[jt]sx$/.test(id)) return null;
      
      const res = transformSync(code, {
        filename: id,
        presets: [
          ['@babel/preset-typescript', { isTSX: true, allExtensions: true }]
        ],
        plugins: [
          ['@babel/plugin-transform-react-jsx', {
            runtime: 'automatic',
            importSource: 'silt-dom'
          }]
        ]
      });
      
      return { code: res.code, map: res.map };
    }
  };
}