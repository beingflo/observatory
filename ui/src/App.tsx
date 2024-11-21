import { For } from "solid-js";
import { dashboards } from "./dashboards";

function App() {
  return (
    <div>
      <div class="text-6xl p-2 md:p-6 md:mb-8 mb-2">Observatory</div>
      <div class="flex flex-row gap-8 px-8">
        <For each={dashboards}>
          {(item) => (
            <a href={item.target}>
              <p class="text-2xl">{item.title}</p>
            </a>
          )}
        </For>
      </div>
    </div>
  );
}

export default App;
