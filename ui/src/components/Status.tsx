import { children, JSX, Show } from "solid-js";
import { Card } from "./Card";

export type StatusProps = {
  title: string;
  content?: string;
  children?: JSX.Element;
};

export const Status = (props: StatusProps) => {
  const ch = children(() => props.children);

  return (
    <Card title={props.title}>
      <Show when={props.content} fallback={ch()}>
        <p class="text-center text-2xl font-bold">{props.content}</p>
      </Show>
    </Card>
  );
};
