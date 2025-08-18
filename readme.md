```
sqlx migrate add initalize -r
```
```
cargo run test --workspace
```


```bash
cargo run -p plugin -- gen #生成签名证书，保存好，私钥给签名插件，公钥校验签名
cargo run -p plugin -- sign target_plugin.{ dylib | dll | so } #签名，默认使用ed25519_sk.bin文件签名
cargo run -p plugin -- verify target_plugin.{ dylib | dll | so } #校验签名
```

### 前端方案：react cdn加载 + importmap + web components挂载
```html:主应用
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vite + React + TS</title>
    <!--导入地图-->
    <script type="importmap"> 
    {
      "imports": { 
        "react": "https://esm.sh/react@19",
        "react-dom": "https://esm.sh/react-dom@19",
        "react-dom/client": "https://esm.sh/react-dom@19/client"
      }
    }
    </script>
    <script type="module" src="http://localhost:4173/plugin-a.es.js"></script><!--插件js-->
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>

```
```tsx
import { createElement } from 'react'
import './App.css'

function App() {
  const A = ()=> createElement('plugin-a');
  return (
    <>
      <A/>应用被webcomponents包裹的组件
    </>
  )
}

export default App
```
vite.config.ts
```js
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
   build: {
    rollupOptions: {
      external: ["react", "react-dom", "react-dom/client"],
    },
  },
})
```

### 插件应用vite.config.ts
```js
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      external: ["react", "react-dom", "react-dom/client"],
    },
    lib: {
      entry: 'src/main.tsx',
      name: 'PluginA',
      formats: ['es', 'umd'],
      fileName: (format) => `plugin-a.${format}.js`,
    },
    minify: true,
  },
  define: {
    "process.env.NODE_ENV": JSON.stringify("production")  // <-- 关键，防止浏览器端报错
  }
})
```
导出的组件
```tsx
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

// createRoot(document.getElementById('root')!).render(
//   <StrictMode>
//     <App />
//   </StrictMode>,
// )
class PluginAElement extends HTMLElement {
  connectedCallback() {
    const root = createRoot(this)
    root.render(<App />)
  }
}

customElements.define('plugin-a', PluginAElement)
```
插件端没有html，主要就是组件和打包配置