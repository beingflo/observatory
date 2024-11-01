import { createResource, For } from "solid-js";

const fetchWeight = async () => {
  const response = await fetch("/api/weight");
  return response.json();
};

function App() {
  const [data] = createResource(fetchWeight);

  const formatter = new Intl.DateTimeFormat("en-CH", {
    timeStyle: "medium",
    dateStyle: "short",
  });

  return (
    <div>
      <div class="text-xl font-bold p-8">Observatory test</div>
      <For each={data()}>
        {(item) => (
          <div class="flex flex-row gap-8 px-8">
            <p>{formatter.format(new Date(item.timestamp))}</p>
            <p>{item.weight}</p>
          </div>
        )}
      </For>
    </div>
  );
}

export default App;
