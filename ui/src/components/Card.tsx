import { children, JSX } from "solid-js";

export type CardProps = {
  children: JSX.Element;
  title: string;
};

export const Card = (props: CardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="p-4 border bg-white rounded-xl">
      <div class="mb-4">{props.title}</div>
      {ch()}
    </div>
  );
};
