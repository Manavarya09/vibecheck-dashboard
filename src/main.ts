import App from "./App.svelte";
import CompanionApp from "./CompanionApp.svelte";
import { mount } from "svelte";

const params = new URLSearchParams(window.location.search);
const RootComponent = params.get("view") === "companion" ? CompanionApp : App;
document.body.dataset.view = params.get("view") ?? "main";

const app = mount(RootComponent, {
  target: document.getElementById("app")!,
});

export default app;
