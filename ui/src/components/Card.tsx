import { children, JSX } from "solid-js";

export type CardProps = {
  title: string;
  children: JSX.Element;
};

export const Card = (props: CardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="p-4 border border-black bg-white rounded-md flex flex-col justify-between">
      <div class="mb-4 text-center text-sm">{props.title}</div>
      {ch()}
    </div>
  );
};
