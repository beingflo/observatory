import { children, JSX } from "solid-js";

export type CardProps = {
  title: string;
  children: JSX.Element;
};

export const Card = (props: CardProps) => {
  const ch = children(() => props.children);

  return (
    <div class="p-4 border bg-white rounded-md">
      <div class="mb-4 text-center">{props.title}</div>
      {ch()}
    </div>
  );
};
