import { For } from "solid-js";
import { dashboards } from "./dashboards";

function App() {
  return (
    <div>
      <div class="text-xl font-bold p-8">Observatory</div>
      <For each={dashboards}>
        {(item) => (
          <div class="flex flex-row gap-8 px-8">
            <a href={item.target}>{item.title}</a>
          </div>
        )}
      </For>
    </div>
  );
}

export default App;
