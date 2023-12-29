## 1、环境配置
### 1.1、trunk
#### 1.1.1、安装trunk
```shell
cargo install trunk
```

#### 1.1.2、trunk启动服务
```shell
trunk serve
```

#### 1.1.3、trunk打包
```shell
trunk build --release
```

### 1.2、集成tailwindcss
#### 1.2.1、集成tailwindcss
创建`tailwind.config.js`文件
```js
module.exports = {
  content: ["./src/**/*.{html,rs}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

创建`index.css`文件
```js
@tailwind base;
@tailwind components;
@tailwind utilities;
```

创建`Trunk.toml`文件
```js
[[hooks]]
stage = "build"
command = "tailwindcss"
command_arguments = [
  "build",
  "-i", 
  "index.css", 
  "-o", 
  "dist/.stage/tailwind.css"
]

[build]
target = "index.html"
dist = "dist"
```

### 1.3、集成tauri
#### 1.3.1、tauri初始化
```shell
cargo install tauri-cli
cargo tauri init
```

#### 1.3.2、tauri注册方法
```js
fn main() {
  tracing_subscriber::fmt::init();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![set_window_decorations])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn set_window_decorations(window: tauri::Window, decorations: bool) {
    info!("Window: {}", window.label());
    window.set_decorations(decorations).unwrap();
}
```
#### 1.3.3、tauri运行
```shell
cargo tauri dev
```
#### 1.3.4、tauri打包
```shell
cargo tauri build
```


### 1.4、wasm-bindgen包装js
创建`glue.js`
```js
const invoke = window.__TAURI__.invoke;

export async function invokeSetWindowDecorations(decorations) {
  return await invoke("set_window_decorations", { decorations });
}
```

`lib.rs`
```js
// 绑定js方法
#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = "invokeSetWindowDecorations")]
    pub async fn set_window_decorations(decorations: bool);
}
```