import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import { GlobalVariablesPlugin } from "./plugins/GlobalVariablesPlugin"; // Import your plugin
import { Buffer } from "buffer";

(window as any).Buffer = Buffer;

const app = createApp(App);

app
  .use(store)
  .use(router)
  .use(GlobalVariablesPlugin) // Use your custom plugin
  .mount("#app");
