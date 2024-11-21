import { children, JSX } from "solid-js";
import { DateRangeSelector } from "./DateRangeSelector";

export type DashboardProps = {
  title: string;
  children: JSX.Element;
};

export const Dashboard = (props: DashboardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="bg-stone-100 p-2 md:p-8 w-full h-full flex flex-col">
      <div class="flex flex-col md:flex-row mb-4 justify-between">
        <a href="/" class="text-6xl md:mb-8 mb-2">
          {props.title}
        </a>
        <DateRangeSelector />
      </div>
      {ch()}
    </div>
  );
};
