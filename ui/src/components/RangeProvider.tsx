import { createSignal, createContext, useContext, JSX } from "solid-js";

const DateRangeContext = createContext();

export type RangeProviderProps = {
  children: JSX.Element;
  from: string;
  to?: string;
};

export function RangeProvider(props: RangeProviderProps) {
  const [from, setFrom] = createSignal(props.from);
  const [to, setTo] = createSignal(props.to);

  const range = [
    { from, to },
    {
      setFrom(from: string) {
        setFrom(from);
      },
      setTo(to: string) {
        setTo(to);
      },
    },
  ];

  return (
    <DateRangeContext.Provider value={range}>
      {props.children}
    </DateRangeContext.Provider>
  );
}

export function useRange(): any {
  return useContext(DateRangeContext) as any;
}
