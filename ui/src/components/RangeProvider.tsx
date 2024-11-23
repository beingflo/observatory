import { createSignal, createContext, useContext, JSX } from "solid-js";
import { getDate } from "./DateRangeSelector";

const DateRangeContext = createContext();

export type RangeProviderProps = {
  children: JSX.Element;
  fromOption: string;
};

export function RangeProvider(props: RangeProviderProps) {
  const [fromOption, setFromOption] = createSignal(props.fromOption);
  const [to, setTo] = createSignal();
  const [customFrom, setCustomFrom] = createSignal(getDate(props.fromOption));

  const from = () => {
    if (fromOption() === "C") {
      return customFrom();
    }
    return getDate(fromOption());
  };

  const range = [
    { fromOption, from, to },
    {
      setFromOption(from: string) {
        setFromOption(from);
        if (from !== "C") {
          setCustomFrom(getDate(from));
        }
      },
      setFrom(from: string) {
        setCustomFrom(from);
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
