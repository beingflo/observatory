import { children, JSX } from "solid-js";
import { DateRangeSelector } from "./DateRangeSelector";

export type DashboardProps = {
  title: string;
  children: JSX.Element;
};

export const Dashboard = (props: DashboardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="bg-slate-100 p-8 w-full h-full flex flex-col">
      <div class="flex flex-row justify-between">
        <h1 class="font-serif text-4xl mb-12">{props.title}</h1>
        <DateRangeSelector />
      </div>
      {ch()}
    </div>
  );
};
