import { children, JSX } from "solid-js";
import { DateRangeSelector } from "./DateRangeSelector";

export type DashboardProps = {
  title: string;
  children: JSX.Element;
};

export const Dashboard = (props: DashboardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="bg-stone-100 p-8 w-full h-full flex flex-col">
      <div class="flex flex-row justify-between">
        <a href="/" class="text-6xl mb-12">
          {props.title}
        </a>
        <DateRangeSelector />
      </div>
      {ch()}
    </div>
  );
};
